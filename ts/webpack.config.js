const CopyPlugin = require('copy-webpack-plugin');
const path = require('path');
const { env, argv } = require('process');
const TerserPlugin = require('terser-webpack-plugin');

module.exports = (env, argv) => {
	const isProduction = argv.mode === 'production';

	return {
		entry: './dist/bootstrap.js',
		output: {
			path: path.resolve(__dirname, '../../www/'),
			filename: 'bootstrap.js',
		},
		target: 'web',
		plugins: [
			new CopyPlugin({
				patterns: ['../www/index.html', '../www/styles.css'],
				options: {
					concurrency: 100,
				},
			}),
		],
		optimization: isProduction ? {
			minimize: true,
			minimizer: [
				new TerserPlugin({
					terserOptions: {
						compress: {
							drop_console: true,
							pure_funcs: ['console.log', 'console.info', 'console.debug', 'console.error', 'console.warn'],
						},
					},
				}),
			],
		} : {},
	}
};
