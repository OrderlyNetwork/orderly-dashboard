/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly DATA_API_URL: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
