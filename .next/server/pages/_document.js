(function() {
var exports = {};
exports.id = 660;
exports.ids = [660];
exports.modules = {

/***/ 98:
/***/ (function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {

"use strict";
// ESM COMPAT FLAG
__webpack_require__.r(__webpack_exports__);

// EXPORTS
__webpack_require__.d(__webpack_exports__, {
  "default": function() { return /* binding */ MyDocument; }
});

// EXTERNAL MODULE: external "react/jsx-runtime"
var jsx_runtime_ = __webpack_require__(282);
// EXTERNAL MODULE: external "react"
var external_react_ = __webpack_require__(297);
// EXTERNAL MODULE: ./node_modules/next/document.js
var next_document = __webpack_require__(859);
;// CONCATENATED MODULE: external "@emotion/server/create-instance"
var create_instance_namespaceObject = require("@emotion/server/create-instance");;
var create_instance_default = /*#__PURE__*/__webpack_require__.n(create_instance_namespaceObject);
// EXTERNAL MODULE: ./styles/createEmotionCache.js + 1 modules
var createEmotionCache = __webpack_require__(492);
// EXTERNAL MODULE: ./styles/theme.js + 1 modules
var theme = __webpack_require__(453);
;// CONCATENATED MODULE: ./pages/_document.js



function ownKeys(object, enumerableOnly) { var keys = Object.keys(object); if (Object.getOwnPropertySymbols) { var symbols = Object.getOwnPropertySymbols(object); if (enumerableOnly) symbols = symbols.filter(function (sym) { return Object.getOwnPropertyDescriptor(object, sym).enumerable; }); keys.push.apply(keys, symbols); } return keys; }

function _objectSpread(target) { for (var i = 1; i < arguments.length; i++) { var source = arguments[i] != null ? arguments[i] : {}; if (i % 2) { ownKeys(Object(source), true).forEach(function (key) { _defineProperty(target, key, source[key]); }); } else if (Object.getOwnPropertyDescriptors) { Object.defineProperties(target, Object.getOwnPropertyDescriptors(source)); } else { ownKeys(Object(source)).forEach(function (key) { Object.defineProperty(target, key, Object.getOwnPropertyDescriptor(source, key)); }); } } return target; }

function _defineProperty(obj, key, value) { if (key in obj) { Object.defineProperty(obj, key, { value: value, enumerable: true, configurable: true, writable: true }); } else { obj[key] = value; } return obj; }






class MyDocument extends next_document.default {
  render() {
    return /*#__PURE__*/(0,jsx_runtime_.jsxs)(next_document.Html, {
      lang: "en",
      children: [/*#__PURE__*/(0,jsx_runtime_.jsxs)(next_document.Head, {
        children: [/*#__PURE__*/jsx_runtime_.jsx("meta", {
          name: "theme-color",
          content: theme/* default.palette.primary.main */.Z.palette.primary.main
        }), /*#__PURE__*/jsx_runtime_.jsx("link", {
          rel: "stylesheet",
          href: "https://fonts.googleapis.com/css?family=Roboto:300,400,500,700&display=swap"
        })]
      }), /*#__PURE__*/(0,jsx_runtime_.jsxs)("body", {
        children: [/*#__PURE__*/jsx_runtime_.jsx(next_document.Main, {}), /*#__PURE__*/jsx_runtime_.jsx(next_document.NextScript, {})]
      })]
    });
  }

}
MyDocument.displayName = 'MyDocument'; // `getInitialProps` belongs to `_document` (instead of `_app`),
// it's compatible with static-site generation (SSG).

MyDocument.getInitialProps = async ctx => {
  // Resolution order
  //
  // On the server:
  // 1. app.getInitialProps
  // 2. page.getInitialProps
  // 3. document.getInitialProps
  // 4. app.render
  // 5. page.render
  // 6. document.render
  //
  // On the server with error:
  // 1. document.getInitialProps
  // 2. app.render
  // 3. page.render
  // 4. document.render
  //
  // On the client
  // 1. app.getInitialProps
  // 2. page.getInitialProps
  // 3. app.render
  // 4. page.render
  const originalRenderPage = ctx.renderPage; // You can consider sharing the same emotion cache between all the SSR requests to speed up performance.
  // However, be aware that it can have global side effects.

  const cache = (0,createEmotionCache/* default */.Z)();
  const {
    extractCriticalToChunks
  } = create_instance_default()(cache);

  ctx.renderPage = () => originalRenderPage({
    enhanceApp: App => function displayName(props) {
      return /*#__PURE__*/jsx_runtime_.jsx(App, _objectSpread({
        emotionCache: cache
      }, props));
    }
  });

  const initialProps = await next_document.default.getInitialProps(ctx); // This is important. It prevents emotion to render invalid HTML.
  // See https://github.com/mui-org/material-ui/issues/26561#issuecomment-855286153

  const emotionStyles = extractCriticalToChunks(initialProps.html);
  const emotionStyleTags = emotionStyles.styles.map(style => /*#__PURE__*/jsx_runtime_.jsx("style", {
    "data-emotion": `${style.key} ${style.ids.join(' ')}`,
    // eslint-disable-next-line react/no-danger
    dangerouslySetInnerHTML: {
      __html: style.css
    }
  }, style.key));
  return _objectSpread(_objectSpread({}, initialProps), {}, {
    // Styles fragment is rendered after the app and page rendering finish.
    styles: [...external_react_.Children.toArray(initialProps.styles), ...emotionStyleTags]
  });
};

/***/ }),

/***/ 492:
/***/ (function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {

"use strict";

// EXPORTS
__webpack_require__.d(__webpack_exports__, {
  "Z": function() { return /* binding */ createEmotionCache; }
});

;// CONCATENATED MODULE: external "@emotion/cache"
var cache_namespaceObject = require("@emotion/cache");;
var cache_default = /*#__PURE__*/__webpack_require__.n(cache_namespaceObject);
;// CONCATENATED MODULE: ./styles/createEmotionCache.js

function createEmotionCache() {
  return cache_default()({
    key: 'css'
  });
}

/***/ }),

/***/ 453:
/***/ (function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {

"use strict";

// EXPORTS
__webpack_require__.d(__webpack_exports__, {
  "Z": function() { return /* binding */ styles_theme; }
});

// EXTERNAL MODULE: external "@mui/material/styles"
var styles_ = __webpack_require__(35);
;// CONCATENATED MODULE: external "@mui/material/colors"
var colors_namespaceObject = require("@mui/material/colors");;
;// CONCATENATED MODULE: ./styles/theme.js

 // Create a theme instance.

let theme = (0,styles_.createTheme)({
  palette: {
    primary: colors_namespaceObject.deepPurple,
    secondary: colors_namespaceObject.amber
  }
});
theme = (0,styles_.responsiveFontSizes)(theme);
/* harmony default export */ var styles_theme = (theme);

/***/ }),

/***/ 35:
/***/ (function(module) {

"use strict";
module.exports = require("@mui/material/styles");;

/***/ }),

/***/ 227:
/***/ (function(module) {

"use strict";
module.exports = require("next/dist/next-server/lib/constants.js");;

/***/ }),

/***/ 932:
/***/ (function(module) {

"use strict";
module.exports = require("next/dist/next-server/lib/document-context.js");;

/***/ }),

/***/ 816:
/***/ (function(module) {

"use strict";
module.exports = require("next/dist/next-server/lib/head-manager-context.js");;

/***/ }),

/***/ 579:
/***/ (function(module) {

"use strict";
module.exports = require("next/dist/next-server/lib/utils.js");;

/***/ }),

/***/ 171:
/***/ (function(module) {

"use strict";
module.exports = require("next/dist/next-server/server/get-page-files.js");;

/***/ }),

/***/ 105:
/***/ (function(module) {

"use strict";
module.exports = require("next/dist/next-server/server/utils.js");;

/***/ }),

/***/ 229:
/***/ (function(module) {

"use strict";
module.exports = require("prop-types");;

/***/ }),

/***/ 297:
/***/ (function(module) {

"use strict";
module.exports = require("react");;

/***/ }),

/***/ 282:
/***/ (function(module) {

"use strict";
module.exports = require("react/jsx-runtime");;

/***/ }),

/***/ 168:
/***/ (function(module) {

"use strict";
module.exports = require("styled-jsx/server");;

/***/ })

};
;

// load runtime
var __webpack_require__ = require("../webpack-runtime.js");
__webpack_require__.C(exports);
var __webpack_exec__ = function(moduleId) { return __webpack_require__(__webpack_require__.s = moduleId); }
var __webpack_exports__ = __webpack_require__.X(0, [859], function() { return __webpack_exec__(98); });
module.exports = __webpack_exports__;

})();