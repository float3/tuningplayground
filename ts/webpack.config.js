import CopyPlugin from 'copy-webpack-plugin';
import path from 'path';

module.exports = {
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
};
