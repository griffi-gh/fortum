module.exports = {
  plugins: [
    require('postcss-import'),
    require('postcss-preset-env')({ stage: 3 }),
    require('cssnano'),
  ],
}
