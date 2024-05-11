import CopyPlugin from "copy-webpack-plugin";
import TerserPlugin from "terser-webpack-plugin";
import JsonMinimizerPlugin from "json-minimizer-webpack-plugin";
import HTMLMinimizerPlugin from "html-minimizer-webpack-plugin";
import path from "path";
import exp from "constants";

module.exports = {
  module: {
    rules: [
      {
        test: /\.json$/i,
        type: "asset/resource",
      },
      {
        test: /\.txt$/i,
        type: "asset/resource",
      },
      {
        test: /\.html$/i,
        type: "asset/resource",
      },
      {
        test: /\.wav$/,
        type: "asset/resource",
      },
      {
        test: /\.midi?$/,
        type: "asset/resource",
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
      patterns: [
        "./src/index.html",
        "./src/chords.json",
        "./src/chords.txt",
        "./src/a1.wav",
        "./src/sample.mid",
      ],
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
    ],
  },
  experiments: {
    syncWebAssembly: true,
  },
};
