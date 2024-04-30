import esbuild from 'esbuild';
import { wasmLoader } from 'esbuild-plugin-wasm';

await esbuild.build({
  bundle: true,
  entryPoints: ['runestone/index.js'],
  format: 'esm',
  platform: 'browser',
  outfile: 'runestone/bundle.js',
  plugins: [wasmLoader({ mode: 'embedded' })],
});

console.log('Build complete!');
