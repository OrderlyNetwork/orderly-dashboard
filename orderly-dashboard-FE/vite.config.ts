import { vitePlugin as remix } from '@remix-run/dev';
import UnoCSS from 'unocss/vite';
import { defineConfig } from 'vite';
import tsconfigPaths from 'vite-tsconfig-paths';

export default defineConfig({
  plugins: [UnoCSS(), remix(), tsconfigPaths()],
  ssr: {
    noExternal: process.env.NODE_ENV === 'production' ? ['@mui/**/*'] : undefined
  },
  envPrefix: ['VITE_', 'DATA_API_']
});
