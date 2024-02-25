const CopyPlugin = require('copy-webpack-plugin');
const TerserPlugin = require('terser-webpack-plugin');
const path = require('path');

module.exports = {
	entry: './dist/bootstrap.js',
	output: {
		path: path.resolve(__dirname, '../www/'),
		filename: 'bootstrap.js',
	},
	target: 'web',
	plugins: [
		new CopyPlugin({
			patterns: ['./src/index.html', './src/styles.css'],
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
						pure_funcs: ['console.log', 'console.info', 'console.debug', 'console.error', 'console.warn'],
					},
					mangle: true,
				},
			}),
		],
	},
};
