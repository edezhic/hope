(function() {
var exports = {};
exports.id = 888;
exports.ids = [888];
exports.modules = {

/***/ 273:
/***/ (function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {

"use strict";
// ESM COMPAT FLAG
__webpack_require__.r(__webpack_exports__);

// EXPORTS
__webpack_require__.d(__webpack_exports__, {
  "default": function() { return /* binding */ MyApp; }
});

// EXTERNAL MODULE: external "react/jsx-runtime"
var jsx_runtime_ = __webpack_require__(282);
;// CONCATENATED MODULE: external "next/head"
var head_namespaceObject = require("next/head");;
var head_default = /*#__PURE__*/__webpack_require__.n(head_namespaceObject);
// EXTERNAL MODULE: external "@mui/material/styles"
var styles_ = __webpack_require__(35);
;// CONCATENATED MODULE: external "@mui/material/CssBaseline"
var CssBaseline_namespaceObject = require("@mui/material/CssBaseline");;
var CssBaseline_default = /*#__PURE__*/__webpack_require__.n(CssBaseline_namespaceObject);
;// CONCATENATED MODULE: external "@emotion/react"
var react_namespaceObject = require("@emotion/react");;
// EXTERNAL MODULE: ./styles/createEmotionCache.js + 1 modules
var createEmotionCache = __webpack_require__(492);
// EXTERNAL MODULE: ./styles/theme.js + 1 modules
var theme = __webpack_require__(453);
;// CONCATENATED MODULE: ./pages/_app.js



function ownKeys(object, enumerableOnly) { var keys = Object.keys(object); if (Object.getOwnPropertySymbols) { var symbols = Object.getOwnPropertySymbols(object); if (enumerableOnly) symbols = symbols.filter(function (sym) { return Object.getOwnPropertyDescriptor(object, sym).enumerable; }); keys.push.apply(keys, symbols); } return keys; }

function _objectSpread(target) { for (var i = 1; i < arguments.length; i++) { var source = arguments[i] != null ? arguments[i] : {}; if (i % 2) { ownKeys(Object(source), true).forEach(function (key) { _defineProperty(target, key, source[key]); }); } else if (Object.getOwnPropertyDescriptors) { Object.defineProperties(target, Object.getOwnPropertyDescriptors(source)); } else { ownKeys(Object(source)).forEach(function (key) { Object.defineProperty(target, key, Object.getOwnPropertyDescriptor(source, key)); }); } } return target; }

function _defineProperty(obj, key, value) { if (key in obj) { Object.defineProperty(obj, key, { value: value, enumerable: true, configurable: true, writable: true }); } else { obj[key] = value; } return obj; }

/*
import '../styles/globals.css'

export default function MyApp({ Component, pageProps }) {
    return <Component {...pageProps} />
  }
  */






 // Client-side cache, shared for the whole session of the user in the browser.

const clientSideEmotionCache = (0,createEmotionCache/* default */.Z)();
function MyApp(props) {
  const {
    Component,
    emotionCache = clientSideEmotionCache,
    pageProps
  } = props;
  return /*#__PURE__*/(0,jsx_runtime_.jsxs)(react_namespaceObject.CacheProvider, {
    value: emotionCache,
    children: [/*#__PURE__*/(0,jsx_runtime_.jsxs)((head_default()), {
      children: [/*#__PURE__*/jsx_runtime_.jsx("title", {
        children: "MUI5 Nextjs"
      }), /*#__PURE__*/jsx_runtime_.jsx("meta", {
        name: "viewport",
        content: "initial-scale=1, width=device-width"
      })]
    }), /*#__PURE__*/(0,jsx_runtime_.jsxs)(styles_.ThemeProvider, {
      theme: theme/* default */.Z,
      children: [/*#__PURE__*/jsx_runtime_.jsx((CssBaseline_default()), {}), /*#__PURE__*/jsx_runtime_.jsx(Component, _objectSpread({}, pageProps))]
    })]
  });
}

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

/***/ 282:
/***/ (function(module) {

"use strict";
module.exports = require("react/jsx-runtime");;

/***/ })

};
;

// load runtime
var __webpack_require__ = require("../webpack-runtime.js");
__webpack_require__.C(exports);
var __webpack_exec__ = function(moduleId) { return __webpack_require__(__webpack_require__.s = moduleId); }
var __webpack_exports__ = (__webpack_exec__(273));
module.exports = __webpack_exports__;

})();