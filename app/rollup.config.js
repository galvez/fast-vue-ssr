import vue from 'rollup-plugin-vue'

export default [
  {
    input: 'src/client.js',
    output: {
      dir: 'dist',
      format: 'esm',
    },
    plugins: [
      vue()
    ]
  },
  {
    input: 'src/server.js',
    output: {
      dir: 'dist',
      format: 'esm'
    },
    plugins: [
      vue({
        // template: {
        //   optimizeSSR: true,
        // }
      })
    ]
  },
]
