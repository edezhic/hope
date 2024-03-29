const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const { resolve } = require('path');

let basePath = assetPrefix = '';
let publicPath = '/_next/';

if (process.env.NODE_ENV === 'production') {
  basePath = assetPrefix = '/hope';
  publicPath = '/hope/_next/';
}

module.exports = {
  basePath,
  assetPrefix,
  eslint: {
    ignoreDuringBuilds: true,
  },
  webpack: (config) => {
    config.output.publicPath = publicPath;

    config.experiments = {
      syncWebAssembly: true,
      topLevelAwait: true,
    };

    config.module.rules.push({
      test: /\.wasm$/,
      type: 'webassembly/sync',
    });

    config.plugins.push(
      new WasmPackPlugin({
        crateDirectory: resolve('./hobot'),
        args: '--log-level error',
      })
    );
    return config;
  },
}; 