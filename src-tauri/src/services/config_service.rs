use crate::models::{
    config::{AppConfig, AudioConfig, HotkeyConfig, ModelConfig, TypingConfig},
    error::VoicePilotError,
};
use crate::utils::paths::config_dir;
use rusqlite::{params, Connection, OptionalExtension};
use std::fs;
use std::path::PathBuf;

const SCHEMA_SQL: &str = include_str!("../../../config/schema.sql");
const DB_FILENAME: &str = "voicepilot.db";

pub struct ConfigService {
    db_path: PathBuf,
}

impl ConfigService {
    pub fn new(db_path: PathBuf) -> Self {
        Self { db_path }
    }

    pub fn for_app() -> Result<Self, VoicePilotError> {
        let dir = config_dir();
        fs::create_dir_all(&dir).map_err(|error| VoicePilotError::Config(error.to_string()))?;
        Ok(Self::new(dir.join(DB_FILENAME)))
    }

    pub fn load(&self) -> Result<AppConfig, VoicePilotError> {
        let connection =
            Connection::open(&self.db_path).map_err(|error| VoicePilotError::Config(error.to_string()))?;

        self.initialize_schema(&connection)?;
        self.seed_defaults(&connection)?;
        self.load_config(&connection)
    }

    pub fn set_llm_model(&self, model: &str) -> Result<(), VoicePilotError> {
        let connection =
            Connection::open(&self.db_path).map_err(|error| VoicePilotError::Config(error.to_string()))?;

        self.initialize_schema(&connection)?;
        self.seed_defaults(&connection)?;
        connection
            .execute(
                "UPDATE models SET llm_model = ?1 WHERE id = 1",
                params![model],
            )
            .map_err(|error| VoicePilotError::Config(error.to_string()))?;
        Ok(())
    }

    fn initialize_schema(&self, connection: &Connection) -> Result<(), VoicePilotError> {
        connection
            .execute_batch("PRAGMA foreign_keys = ON; PRAGMA journal_mode = WAL;")
            .map_err(|error| VoicePilotError::Config(error.to_string()))?;
        connection
            .execute_batch(SCHEMA_SQL)
            .map_err(|error| VoicePilotError::Config(error.to_string()))?;
        // ponytail: single schema, no migrations yet. Bump this and add a migration
        // runner keyed off it once SCHEMA_SQL needs a second version.
        connection
            .pragma_update(None, "user_version", 1)
            .map_err(|error| VoicePilotError::Config(error.to_string()))
    }

    fn seed_defaults(&self, connection: &Connection) -> Result<(), VoicePilotError> {
        let defaults = AppConfig::default();

        connection
            .execute(
                "INSERT INTO hotkey (id, modifiers, key, mode) VALUES (1, ?1, ?2, ?3)
                 ON CONFLICT(id) DO NOTHING",
                params![
                    defaults.hotkey.modifiers.join(","),
                    defaults.hotkey.key,
                    defaults.hotkey.mode
                ],
            )
            .map_err(|error| VoicePilotError::Config(error.to_string()))?;

        connection
            .execute(
                "INSERT INTO audio (id, device_id, vad_sensitivity) VALUES (1, ?1, ?2)
                 ON CONFLICT(id) DO NOTHING",
                params![defaults.audio.device_id, defaults.audio.vad_sensitivity],
            )
            .map_err(|error| VoicePilotError::Config(error.to_string()))?;

        connection
            .execute(
                "INSERT INTO models (id, stt_model, llm_model) VALUES (1, ?1, ?2)
                 ON CONFLICT(id) DO NOTHING",
                params![defaults.models.stt_model, defaults.models.llm_model],
            )
            .map_err(|error| VoicePilotError::Config(error.to_string()))?;

        self.seed_config_value(connection, "typing.mode", &defaults.typing.mode)?;
        self.seed_config_value(
            connection,
            "typing.speed_cps",
            &defaults.typing.speed_cps.to_string(),
        )?;

        Ok(())
    }

    fn seed_config_value(
        &self,
        connection: &Connection,
        key: &str,
        value: &str,
    ) -> Result<(), VoicePilotError> {
        connection
            .execute(
                "INSERT INTO config (key, value) VALUES (?1, ?2)
                 ON CONFLICT(key) DO NOTHING",
                params![key, value],
            )
            .map_err(|error| VoicePilotError::Config(error.to_string()))?;
        Ok(())
    }

    fn load_config(&self, connection: &Connection) -> Result<AppConfig, VoicePilotError> {
        let hotkey = connection
            .query_row(
                "SELECT modifiers, key, mode FROM hotkey WHERE id = 1",
                [],
                |row| {
                    let modifiers: String = row.get(0)?;
                    Ok(HotkeyConfig {
                        modifiers: split_modifiers(&modifiers),
                        key: row.get(1)?,
                        mode: row.get(2)?,
                    })
                },
            )
            .map_err(|error| VoicePilotError::Config(error.to_string()))?;

        let audio = connection
            .query_row(
                "SELECT device_id, vad_sensitivity FROM audio WHERE id = 1",
                [],
                |row| {
                    Ok(AudioConfig {
                        device_id: row.get(0)?,
                        vad_sensitivity: row.get(1)?,
                    })
                },
            )
            .map_err(|error| VoicePilotError::Config(error.to_string()))?;

        let models = connection
            .query_row(
                "SELECT stt_model, llm_model FROM models WHERE id = 1",
                [],
                |row| {
                    Ok(ModelConfig {
                        stt_model: row.get(0)?,
                        llm_model: row.get(1)?,
                    })
                },
            )
            .map_err(|error| VoicePilotError::Config(error.to_string()))?;

        let defaults = AppConfig::default();
        let typing = TypingConfig {
            mode: self
                .get_config_value(connection, "typing.mode")?
                .unwrap_or(defaults.typing.mode),
            speed_cps: self
                .get_config_value(connection, "typing.speed_cps")?
                .and_then(|value| value.parse::<u32>().ok())
                .unwrap_or(defaults.typing.speed_cps),
        };

        Ok(AppConfig {
            hotkey,
            audio,
            models,
            typing,
        })
    }

    fn get_config_value(
        &self,
        connection: &Connection,
        key: &str,
    ) -> Result<Option<String>, VoicePilotError> {
        connection
            .query_row(
                "SELECT value FROM config WHERE key = ?1",
                params![key],
                |row| row.get(0),
            )
            .optional()
            .map_err(|error| VoicePilotError::Config(error.to_string()))
    }
}

fn split_modifiers(modifiers: &str) -> Vec<String> {
    modifiers
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::ConfigService;
    use std::path::PathBuf;

    #[test]
    fn load_initializes_and_returns_defaults() {
        // ponytail: rusqlite treats the literal path ":memory:" as an in-memory DB,
        // so no on-disk file or cleanup is needed.
        let service = ConfigService::new(PathBuf::from(":memory:"));

        let config = service.load().expect("config should load");

        assert_eq!(config.hotkey.modifiers, vec!["ctrl"]);
        assert_eq!(config.hotkey.key, "Space");
        assert_eq!(config.audio.vad_sensitivity, 0.5);
        assert_eq!(config.models.stt_model, "nvidia/parakeet-tdt-0.6b");
        assert_eq!(config.typing.speed_cps, 50);
    }
}
