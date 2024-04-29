import esbuild from 'esbuild';
import { wasmLoader } from 'esbuild-plugin-wasm';

await esbuild.build({
  bundle: true,
  entryPoints: ['src/index.ts'],
  format: 'esm',
  platform: 'browser',
  outfile: 'runestone/bundle.js',
  plugins: [wasmLoader({ mode: 'embedded' })],
});

console.log('Build complete!');
