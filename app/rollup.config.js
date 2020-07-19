import { nodeResolve } from '@rollup/plugin-node-resolve'
import vue from 'rollup-plugin-vue'

export default [
  {
    input: 'src/client.js',
    output: {
      dir: 'dist',
      format: 'es',
    },
    plugins: [
      vue(),
      nodeResolve()
    ]
  },
  {
    input: 'src/server.js',
    output: {
      dir: 'dist',
      format: 'esm',
      inlineDynamicImports: true,
    },
    treeshake: false,
    preserveEntrySignatures: false,
    plugins: [
      vue()
    ]
  },
]
