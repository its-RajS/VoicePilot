export interface VoicePilotConfig {
  hotkey: { modifiers: string[]; key: string; mode: 'push_to_talk' | 'toggle' };
  audio: { deviceId: string | null; vadSensitivity: number };
  models: { sttModel: string; llmModel: string };
  typing: { mode: 'auto_type' | 'clipboard'; speedCps: number };
}
