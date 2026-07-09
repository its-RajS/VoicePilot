import { Mic } from 'lucide-react';
import { useTranscriptStore } from '../../stores/transcriptStore';

export function LiveOverlay() {
  const { partial, cleaned, status } = useTranscriptStore();
  const content =
    status === 'complete'
      ? cleaned || 'Prompt cleaned and ready.'
      : partial || 'Hold the session hotkey to start dictating.';

  return (
    <aside className="live-overlay" aria-live="polite">
      <Mic size={16} />
      <span>{content}</span>
    </aside>
  );
}
