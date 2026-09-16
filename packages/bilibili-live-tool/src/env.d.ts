/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_BUILD_CHANNEL: 'free' | 'member';
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
