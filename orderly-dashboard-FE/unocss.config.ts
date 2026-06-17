import { defineConfig, presetUno } from 'unocss';

export default defineConfig({
  presets: [presetUno()],
  theme: {
    fontFamily: {
      sans: "'Atyp BL Text', 'Atyp BL', -apple-system, BlinkMacSystemFont, sans-serif",
      display: "'Atyp BL', -apple-system, BlinkMacSystemFont, sans-serif"
    },
    breakpoints: {
      sm: '640px',
      md: '768px',
      lg: '1024px',
      xl: '1340px',
      '2xl': '1536px'
    }
  }
});
