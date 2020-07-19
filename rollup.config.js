
import replace from '@rollup/plugin-replace'
import { nodeResolve } from '@rollup/plugin-node-resolve'
import vue from 'rollup-plugin-vue'

export default [
  {
    input: 'app/client.js',
    output: {
      dir: 'dist',
      format: 'es',
    },
    plugins: [
      nodeResolve(),
      vue(),
      replace({
        'process.env.NODE_ENV': '"production"',
      }),
    ]
  },
  {
    input: 'app/server.js',
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
