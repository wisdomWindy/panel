declare module 'whisper-node' {
  export function whisper(audioPath: string, options?: Record<string, unknown>): Promise<string[]>;
  export default whisper;
}
