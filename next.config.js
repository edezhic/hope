const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const { resolve } = require('path');

let basePath = '/Hope';
let assetPrefix = '/Hope';
let publicPath = '/Hope/_next/';

if (process.env.NODE_ENV === 'development') {
  basePath = '';
  assetPrefix = '';
  publicPath = '/_next/';
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