const CopyPlugin = require("copy-webpack-plugin");
const path = require("path");

module.exports = {
    entry: "./bootstrap.js",
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "bootstrap.js",
    },
    mode: "production",
    plugins: [
        new CopyPlugin({
            patterns: [
                "./index.html",
                "./styles.css"
            ],
            options: {
                concurrency: 100,
            },
        }),
    ],
};