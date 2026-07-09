import { useEffect, useState, useTransition } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import {
  Activity,
  ArrowUpRight,
  AudioLines,
  Download,
  LoaderCircle,
  Mic,
  RefreshCw,
  Sparkles,
  WandSparkles,
} from 'lucide-react';
import { LiveOverlay } from './components/overlay/LiveOverlay';
import { useTranscriptStore } from './stores/transcriptStore';
import type { VoicePilotConfig } from './types/config';

type OllamaModel = {
  name: string;
  size: number | null;
  modified_at: string | null;
};

type OllamaStatus = {
  installed: boolean;
  version: string | null;
  api_reachable: boolean;
  api_url: string;
  models: OllamaModel[];
  recommended_model_present: boolean;
  error: string | null;
};

type RuntimeMode = 'tauri' | 'browser';

const DEMO_PROMPT = 'Refactor the Linux audio capture path and explain the hotkey edge cases.';
const RECOMMENDED_MODEL = 'qwen3:8b';

const formatHotkey = (config: VoicePilotConfig | null) =>
  config ? [...config.hotkey.modifiers, config.hotkey.key].join(' + ') : 'Ctrl + Space';

const formatBytes = (size: number | null) => {
  if (!size) return 'Unknown size';
  const units = ['B', 'KB', 'MB', 'GB'];
  let value = size;
  let unitIndex = 0;
  while (value >= 1024 && unitIndex < units.length - 1) {
    value /= 1024;
    unitIndex += 1;
  }
  return `${value.toFixed(value >= 10 || unitIndex === 0 ? 0 : 1)} ${units[unitIndex]}`;
};

export default function App() {
  const {
    partial,
    final,
    cleaned,
    status,
    setStatus,
    setPartial,
    setFinal,
    setCleaned,
    reset,
  } = useTranscriptStore();

  const [config, setConfig] = useState<VoicePilotConfig | null>(null);
  const [ollamaStatus, setOllamaStatus] = useState<OllamaStatus | null>(null);
  const [bridgeHealth, setBridgeHealth] = useState<string>('Checking');
  const [runtimeMode, setRuntimeMode] = useState<RuntimeMode>('browser');
  const [notes, setNotes] = useState<string>(DEMO_PROMPT);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const [selectedModel, setSelectedModel] = useState<string>(RECOMMENDED_MODEL);
  const [pullConfirming, setPullConfirming] = useState(false);
  const [isRefreshing, startRefresh] = useTransition();
  const [isCleaning, startCleaning] = useTransition();
  const [isPulling, startPull] = useTransition();

  useEffect(() => {
    let isMounted = true;

    const load = async () => {
      const isTauriRuntime = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
      if (!isMounted) return;
      setRuntimeMode(isTauriRuntime ? 'tauri' : 'browser');

      if (!isTauriRuntime) {
        setConfig({
          hotkey: { modifiers: ['ctrl'], key: 'Space', mode: 'push_to_talk' },
          audio: { deviceId: null, vadSensitivity: 0.5 },
          models: { sttModel: 'nvidia/parakeet-tdt-0.6b', llmModel: 'qwen3:8b' },
          typing: { mode: 'auto_type', speedCps: 50 },
        });
        setOllamaStatus({
          installed: false,
          version: null,
          api_reachable: false,
          api_url: 'http://127.0.0.1:11434',
          models: [],
          recommended_model_present: false,
          error: 'Launch inside Tauri to query the local runtime.',
        });
        setBridgeHealth('Browser preview');
        setPartial('Open the desktop app to exercise Rust, Python, and Ollama features.');
        setStatus('idle');
        return;
      }

      try {
        const [loadedConfig, health, loadedOllamaStatus] = await Promise.all([
          invoke<VoicePilotConfig>('get_config'),
          invoke<string>('health'),
          invoke<OllamaStatus>('get_ollama_status'),
        ]);

        if (!isMounted) return;
        setConfig(loadedConfig);
        setSelectedModel(loadedConfig.models.llmModel);
        setBridgeHealth(health === 'ok' ? 'Live' : health);
        setOllamaStatus(loadedOllamaStatus);
        setPartial('Voice bridge ready. Hold the hotkey, speak naturally, then review cleanup.');
        setStatus('idle');
      } catch (error) {
        if (!isMounted) return;
        setErrorMessage(error instanceof Error ? error.message : String(error));
        setBridgeHealth('Unavailable');
        setStatus('processing');
      }
    };

    void load();

    return () => {
      isMounted = false;
    };
  }, [setPartial, setStatus]);

  // Hotkey-driven events from the Rust push-to-talk flow. The listener is
  // attached only in Tauri; the browser preview has no global hotkey path.
  useEffect(() => {
    if (runtimeMode !== 'tauri') return;
    const unlistens: UnlistenFn[] = [];
    let cancelled = false;

    void (async () => {
      const subs = [
        listen<string>('voicepilot:status', (event) => {
          const value = event.payload;
          setStatus(
            value === 'listening'
              ? 'listening'
              : value === 'processing'
                ? 'processing'
                : value === 'typing'
                  ? 'typing'
                  : value === 'complete'
                    ? 'complete'
                    : 'idle',
          );
        }),
        listen<string>('voicepilot:final', (event) => setFinal(event.payload)),
        listen<string>('voicepilot:cleaned', (event) => setCleaned(event.payload)),
        listen<string>('voicepilot:error', (event) =>
          setErrorMessage(event.payload),
        ),
      ];
      for (const sub of subs) {
        // eslint-disable-next-line no-await-in-loop
        unlistens.push(await sub);
      }
      if (cancelled) {
        unlistens.splice(0).forEach((unlisten) => unlisten());
      }
    })();

    return () => {
      cancelled = true;
      unlistens.splice(0).forEach((unlisten) => unlisten());
    };
  }, [runtimeMode, setStatus, setFinal, setCleaned]);

  const refreshOllama = () => {
    startRefresh(() => {
      void (async () => {
        if (runtimeMode !== 'tauri') return;
        try {
          const nextStatus = await invoke<OllamaStatus>('refresh_ollama_status');
          setOllamaStatus(nextStatus);
          setErrorMessage(null);
        } catch (error) {
          setErrorMessage(error instanceof Error ? error.message : String(error));
        }
      })();
    });
  };

  const pullRecommended = () => {
    setPullConfirming(false);
    startPull(() => {
      void (async () => {
        if (runtimeMode !== 'tauri') return;
        try {
          await invoke('pull_ollama_model', { model: RECOMMENDED_MODEL });
          const nextStatus = await invoke<OllamaStatus>('refresh_ollama_status');
          setOllamaStatus(nextStatus);
          await invoke('set_llm_model', { model: RECOMMENDED_MODEL });
          setSelectedModel(RECOMMENDED_MODEL);
          const loadedConfig = await invoke<VoicePilotConfig>('get_config');
          setConfig(loadedConfig);
          setErrorMessage(null);
        } catch (error) {
          setErrorMessage(error instanceof Error ? error.message : String(error));
        }
      })();
    });
  };

  const runCleanup = () => {
    startCleaning(() => {
      void (async () => {
        reset();
        setStatus('listening');
        setFinal(notes.trim());
        setPartial('Raw dictation captured. Sending to local cleanup pipeline.');

        if (runtimeMode !== 'tauri') {
          setStatus('complete');
          setCleaned(notes.trim());
          return;
        }

        try {
          setStatus('processing');
          const response = await invoke<string>('cleanup_transcript', { transcript: notes });
          setCleaned(response);
          setStatus('complete');
        } catch (error) {
          setStatus('typing');
          setErrorMessage(error instanceof Error ? error.message : String(error));
        }
      })();
    });
  };

  const isBusy = isRefreshing || isCleaning || isPulling;
  const modelOptions = (() => {
    const available = ollamaStatus?.models.map((model) => model.name) ?? [];
    const configured = selectedModel || RECOMMENDED_MODEL;
    return available.includes(configured) ? available : [configured, ...available];
  })();
  const statusTone =
    status === 'idle'
      ? 'Quiet'
      : status === 'listening'
        ? 'Listening'
        : status === 'processing'
          ? 'Cleaning'
          : status === 'complete'
            ? 'Ready to paste'
            : 'Attention';

  return (
    <main className="app-shell">
      <div className="app-grid">
        <section className="session-panel">
          <header className="session-header">
            <div>
              <p className="eyebrow">Local voice workflow</p>
              <h1>Speak once. Ship cleaner prompts.</h1>
            </div>
            <span className="status-pill">
              <span className="status-dot" />
              {statusTone}
            </span>
          </header>

          <div className="session-callout">
            <div className="callout-badge">
              <Mic size={18} />
              Push to talk
            </div>
            <p>
              Hold <strong>{formatHotkey(config)}</strong> to start a flow session, let the local bridge clean the
              transcript, then auto-type or copy the result.
            </p>
          </div>

          <div className="composer-card">
            <div className="card-heading">
              <div>
                <p className="card-label">Cleanup playground</p>
                <h2>Test the Rust + Python + Ollama path</h2>
              </div>
              <button className="ghost-button" onClick={() => setNotes(DEMO_PROMPT)} type="button">
                Use sample
              </button>
            </div>

            <textarea
              className="prompt-input"
              value={notes}
              onChange={(event) => setNotes(event.target.value)}
              placeholder="Dictate a rough engineering prompt here."
            />

            <div className="composer-actions">
              <button className="primary-button" onClick={runCleanup} type="button" disabled={isBusy || !notes.trim()}>
                {isCleaning ? <LoaderCircle className="spin" size={18} /> : <WandSparkles size={18} />}
                Clean transcript
              </button>
              <p className="helper-copy">Runs the feature path created in `VOICE-003`.</p>
            </div>
          </div>

          <div className="transcript-stack">
            <article className="transcript-card">
              <p className="card-label">Raw capture</p>
              <p>{final || partial || 'No active session yet.'}</p>
            </article>
            <article className="transcript-card transcript-card-accent">
              <p className="card-label">Cleaned output</p>
              <p>{cleaned || 'Your cleaned prompt appears here after local processing.'}</p>
            </article>
          </div>
        </section>

        <aside className="control-panel">
          <section className="metric-card">
            <div className="metric-header">
              <AudioLines size={18} />
              <span>Runtime</span>
            </div>
            <strong>{runtimeMode === 'tauri' ? 'Desktop session' : 'Browser preview'}</strong>
            <p>{runtimeMode === 'tauri' ? 'Rust commands are live.' : 'Preview mode uses safe placeholders.'}</p>
          </section>

          <section className="metric-card">
            <div className="metric-header">
              <Activity size={18} />
              <span>Bridge health</span>
            </div>
            <strong>{bridgeHealth}</strong>
            <p>Backed by the `health` command from the Rust to Python bridge.</p>
          </section>

          <section className="metric-card">
            <div className="metric-header">
              <Sparkles size={18} />
              <span>Configuration</span>
            </div>
            <strong>{config?.models.llmModel ?? 'Loading model config'}</strong>
            <p>
              {config
                ? `${config.typing.mode.replace('_', ' ')} at ${config.typing.speedCps} cps`
                : 'Waiting for SQLite-backed config.'}
            </p>
          </section>

          <section className="feature-card">
            <div className="card-heading">
              <div>
                <p className="card-label">Ollama discovery</p>
                <h2>Local model inventory</h2>
              </div>
              <button className="icon-button" onClick={refreshOllama} type="button" disabled={isBusy}>
                <RefreshCw className={isRefreshing ? 'spin' : ''} size={16} />
              </button>
            </div>

            <div className="feature-row">
              <span>Install</span>
              <strong>{ollamaStatus?.installed ? ollamaStatus.version ?? 'Installed' : 'Missing'}</strong>
            </div>
            <div className="feature-row">
              <span>API</span>
              <strong>{ollamaStatus?.api_reachable ? 'Reachable' : 'Offline'}</strong>
            </div>
            <div className="feature-row">
              <span>Recommended</span>
              <strong>{ollamaStatus?.recommended_model_present ? 'Qwen ready' : 'Needs pull'}</strong>
            </div>

            {runtimeMode === 'tauri' && ollamaStatus && !ollamaStatus.api_reachable ? (
              <div className="setup-card">
                <p className="card-label">Setup</p>
                {ollamaStatus.error ? <p className="empty-copy">{ollamaStatus.error}</p> : null}
                <ol className="setup-steps">
                  <li>
                    Install Ollama: <code>curl -fsSL https://ollama.com/install.sh | sh</code>
                  </li>
                  <li>
                    Start the service: <code>ollama serve</code>
                  </li>
                  <li>
                    Pull a model: <code>ollama pull {RECOMMENDED_MODEL}</code>
                  </li>
                </ol>
              </div>
            ) : null}

            {ollamaStatus?.api_reachable && ollamaStatus.models.length > 0 ? (
              <label className="feature-row model-select" htmlFor="llm-model">
                <span>Active model</span>
                <select
                  id="llm-model"
                  value={selectedModel}
                  disabled={isBusy}
                  onChange={(event) => {
                    const model = event.target.value;
                    setSelectedModel(model);
                    void (async () => {
                      if (runtimeMode !== 'tauri') return;
                      try {
                        await invoke('set_llm_model', { model });
                        const loadedConfig = await invoke<VoicePilotConfig>('get_config');
                        setConfig(loadedConfig);
                        setErrorMessage(null);
                      } catch (error) {
                        setErrorMessage(error instanceof Error ? error.message : String(error));
                      }
                    })();
                  }}
                >
                  {modelOptions.map((model) => (
                    <option key={model} value={model}>
                      {model}
                    </option>
                  ))}
                </select>
              </label>
            ) : null}

            {runtimeMode === 'tauri' && ollamaStatus?.api_reachable && !ollamaStatus.recommended_model_present ? (
              pullConfirming ? (
                <div className="pull-confirm">
                  <p className="empty-copy">Download {RECOMMENDED_MODEL}? This can take several minutes.</p>
                  <div className="pull-confirm-actions">
                    <button className="pull-button" onClick={pullRecommended} type="button" disabled={isBusy}>
                      {isPulling ? <LoaderCircle className="spin" size={16} /> : <Download size={16} />}
                      Confirm download
                    </button>
                    <button
                      className="ghost-button"
                      onClick={() => setPullConfirming(false)}
                      type="button"
                      disabled={isBusy}
                    >
                      Cancel
                    </button>
                  </div>
                </div>
              ) : (
                <button
                  className="ghost-button pull-trigger"
                  onClick={() => setPullConfirming(true)}
                  type="button"
                  disabled={isBusy}
                >
                  <Download size={16} />
                  Pull {RECOMMENDED_MODEL}
                </button>
              )
            ) : null}

            {ollamaStatus?.api_reachable ? (
              <div className="model-list">
                {ollamaStatus.models.length > 0 ? (
                  ollamaStatus.models.slice(0, 4).map((model) => (
                    <div key={model.name} className="model-pill">
                      <span>{model.name}</span>
                      <small>{formatBytes(model.size)}</small>
                    </div>
                  ))
                ) : (
                  <p className="empty-copy">No models yet — pull {RECOMMENDED_MODEL} to begin.</p>
                )}
              </div>
            ) : runtimeMode !== 'tauri' ? (
              <p className="empty-copy">{ollamaStatus?.error ?? 'Launch inside Tauri to query the local runtime.'}</p>
            ) : null}
          </section>

          <section className="feature-card">
            <div className="card-heading">
              <div>
                <p className="card-label">Current setup</p>
                <h2>Session defaults</h2>
              </div>
              <ArrowUpRight size={16} />
            </div>
            <div className="feature-row">
              <span>Hotkey</span>
              <strong>{formatHotkey(config)}</strong>
            </div>
            <div className="feature-row">
              <span>ASR</span>
              <strong>{config?.models.sttModel ?? 'Loading'}</strong>
            </div>
            <div className="feature-row">
              <span>Typing</span>
              <strong>{config ? config.typing.mode.replace('_', ' ') : 'Loading'}</strong>
            </div>
            <div className="feature-row">
              <span>VAD</span>
              <strong>{config ? `${Math.round(config.audio.vadSensitivity * 100)}%` : 'Loading'}</strong>
            </div>
          </section>
        </aside>
      </div>

      {errorMessage ? <div className="error-banner">{errorMessage}</div> : null}
      <LiveOverlay />
    </main>
  );
}
