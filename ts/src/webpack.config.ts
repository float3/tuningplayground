const CopyPlugin = require("copy-webpack-plugin");
const path = require("path");

module.exports = {
    entry: "./dist/bootstrap.js",
    output: {
        path: path.resolve(__dirname, "../../www/"),
        filename: "bootstrap.js",
    },
    target: "web",
    mode: "production",
    plugins: [
        new CopyPlugin({
            patterns: [
                "../www/index.html",
                "../www/styles.css"
            ],
            options: {
                concurrency: 100,
            },
        }),
    ],
};