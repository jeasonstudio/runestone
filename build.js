import esbuild from 'esbuild';
import { wasmLoader } from 'esbuild-plugin-wasm';

await Promise.all([
  esbuild.build({
    bundle: true,
    entryPoints: ['runestone/index.js'],
    format: 'esm',
    platform: 'browser',
    outfile: 'runestone/bundle.js',
    plugins: [wasmLoader({ mode: 'embedded' })],
  }),
  esbuild.build({
    bundle: true,
    entryPoints: ['runestone/cjs/index.js'],
    format: 'cjs',
    platform: 'node',
    outfile: 'runestone/cjs/bundle.js',
    plugins: [wasmLoader({ mode: 'embedded' })],
  }),
]);

console.log('Build complete!');
