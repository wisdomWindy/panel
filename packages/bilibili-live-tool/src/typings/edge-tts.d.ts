declare module 'edge-tts' {
  export class EdgeTTS {
    toFile(text: string, filePath: string, options?: Record<string, unknown>): Promise<void>;
    setVoice(voice: string): void;
    setRate(rate: string): void;
    setVolume(volume: string): void;
    setPitch(pitch: string): void;
  }
  export default EdgeTTS;
}
