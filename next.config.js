const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const SSRPlugin =
  require('next/dist/build/webpack/plugins/nextjs-ssr-import').default;
const {
  dirname,
  relative,
  resolve,
  join,
} = require('path');

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
    // Warning: This allows production builds to successfully complete even if your project has ESLint errors.
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
        crateDirectory: resolve('./core'),
        args: '--log-level error',
      })
    );

    const ssrPlugin = config.plugins.find(
      (plugin) => plugin instanceof SSRPlugin
    );

    if (ssrPlugin) {
      patchSsrPlugin(ssrPlugin);
    }

    return config;
  },
}; 

// Patch the NextJsSSRImport plugin to not throw with WASM generated chunks.
function patchSsrPlugin(plugin) {
  plugin.apply = function apply(compiler) {
    compiler.hooks.compilation.tap(
      'NextJsSSRImport',
      (compilation) => {
        compilation.mainTemplate.hooks.requireEnsure.tap(
          'NextJsSSRImport',
          (code, chunk) => {
            // The patch that we need to ensure this plugin doesn't throw
            // with WASM chunks.
            if (!chunk.name) {
              return;
            }

            // Update to load chunks from our custom chunks directory
            const outputPath = resolve('/');
            const pagePath = join('/', dirname(chunk.name));
            const relativePathToBaseDir = relative(
              pagePath,
              outputPath
            );
            // Make sure even in windows, the path looks like in unix
            // Node.js require system will convert it accordingly
            const relativePathToBaseDirNormalized =
              relativePathToBaseDir.replace(/\\/g, '/');
            return code
              .replace(
                'require("./"',
                `require("${relativePathToBaseDirNormalized}/"`
              )
              .replace(
                'readFile(join(__dirname',
                `readFile(join(__dirname, "${relativePathToBaseDirNormalized}"`
              );
          }
        );
      }
    );
  };
}
