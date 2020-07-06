import vue from 'rollup-plugin-vue'

export default {
  input: 'src/index.js',
  output: {
    file: 'bundle.js',
    format: 'esm'
  },
  plugins: [
    vue()
  ]
}
