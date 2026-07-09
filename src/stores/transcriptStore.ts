import { create } from 'zustand';

export type TranscriptStatus = 'idle' | 'listening' | 'processing' | 'typing' | 'complete';

interface TranscriptStore {
  partial: string;
  final: string;
  cleaned: string;
  status: TranscriptStatus;
  setPartial: (text: string) => void;
  setFinal: (text: string) => void;
  setCleaned: (text: string) => void;
  setStatus: (status: TranscriptStatus) => void;
  reset: () => void;
}

export const useTranscriptStore = create<TranscriptStore>((set) => ({
  partial: '',
  final: '',
  cleaned: '',
  status: 'idle',
  setPartial: (partial) => set({ partial }),
  setFinal: (final) => set({ final }),
  setCleaned: (cleaned) => set({ cleaned }),
  setStatus: (status) => set({ status }),
  reset: () => set({ partial: '', final: '', cleaned: '', status: 'idle' }),
}));
