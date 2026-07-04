import { create } from 'zustand';

type Status = 'idle' | 'listening' | 'processing' | 'typing' | 'complete';

interface TranscriptStore {
  partial: string;
  final: string;
  cleaned: string;
  status: Status;
  setPartial: (text: string) => void;
  reset: () => void;
}

export const useTranscriptStore = create<TranscriptStore>((set) => ({
  partial: '',
  final: '',
  cleaned: '',
  status: 'idle',
  setPartial: (partial) => set({ partial }),
  reset: () => set({ partial: '', final: '', cleaned: '', status: 'idle' }),
}));
