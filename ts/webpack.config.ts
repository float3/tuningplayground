import CopyPlugin from "copy-webpack-plugin";
import TerserPlugin from "terser-webpack-plugin";
import JsonMinimizerPlugin from "json-minimizer-webpack-plugin";
import HTMLMinimizerPlugin from "html-minimizer-webpack-plugin";
import CssMinimizerPlugin from "css-minimizer-webpack-plugin";
import MiniCssExtractPlugin from "mini-css-extract-plugin";
import path from "path";

module.exports = {
  module: {
    rules: [
      {
        test: /\.json$/i,
        type: "asset/resource",
      },
      {
        test: /\.html$/i,
        type: "asset/resource",
      },
      {
        test: /.s?css$/,
        use: [MiniCssExtractPlugin.loader, "css-loader", "sass-loader"],
      },
    ],
  },
  entry: "./dist/bootstrap.js",
  output: {
    path: path.resolve(__dirname, "../www/"),
    filename: "bootstrap.js",
  },
  target: "web",
  plugins: [
    new CopyPlugin({
      patterns: ["./src/index.html", "./src/styles.css", "./src/chords.json"],
      options: {
        concurrency: 100,
      },
    }),
  ],
  optimization: {
    minimizer: [
      new TerserPlugin({
        terserOptions: {
          compress: {
            drop_console: true,
            pure_funcs: [
              "console.log",
              "console.info",
              "console.debug",
              "console.error",
              "console.warn",
              "console.assert",
            ],
          },
          mangle: true,
        },
      }),
      new JsonMinimizerPlugin(),
      new HTMLMinimizerPlugin(),
      new CssMinimizerPlugin(),
    ],
  },
};
