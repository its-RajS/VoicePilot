import { Mic, Settings } from 'lucide-react';
import { LiveOverlay } from './components/overlay/LiveOverlay';

export default function App() {
  return (
    <main className="app-shell">
      <section className="hero-card">
        <div className="brand-row">
          <Mic size={32} />
          <h1>VoicePilot</h1>
        </div>
        <p>Local-first voice-to-prompt assistant for Linux developers.</p>
        <button className="primary-button"><Settings size={18} /> Open Settings</button>
      </section>
      <LiveOverlay />
    </main>
  );
}
