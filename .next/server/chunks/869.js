/******/ (function() { // webpackBootstrap
/******/ 	"use strict";
/******/ 	var __webpack_modules__ = ({

/***/ 869:
/***/ (function(__unused_webpack_module, __unused_webpack___webpack_exports__, __webpack_require__) {

const ctx = self;

async function translate(script) {
  const {
    tokenize
  } = await __webpack_require__.e(/* import() */ 734).then(__webpack_require__.bind(__webpack_require__, 734));
  ctx.postMessage({
    type: 'translation',
    tokens: tokenize(script)
  });
}

ctx.addEventListener('message', evt => {
  switch (evt.data.type) {
    case 'translate':
      translate(evt.data.script);
      return;
  }
});


/***/ })

/******/ 	});
/************************************************************************/
/******/ 	// The module cache
/******/ 	var __webpack_module_cache__ = {};
/******/ 	
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/ 		// Check if module is in cache
/******/ 		var cachedModule = __webpack_module_cache__[moduleId];
/******/ 		if (cachedModule !== undefined) {
/******/ 			return cachedModule.exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = __webpack_module_cache__[moduleId] = {
/******/ 			id: moduleId,
/******/ 			loaded: false,
/******/ 			exports: {}
/******/ 		};
/******/ 	
/******/ 		// Execute the module function
/******/ 		var threw = true;
/******/ 		try {
/******/ 			__webpack_modules__[moduleId](module, module.exports, __webpack_require__);
/******/ 			threw = false;
/******/ 		} finally {
/******/ 			if(threw) delete __webpack_module_cache__[moduleId];
/******/ 		}
/******/ 	
/******/ 		// Flag the module as loaded
/******/ 		module.loaded = true;
/******/ 	
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/ 	
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = __webpack_modules__;
/******/ 	
/******/ 	// expose the module cache
/******/ 	__webpack_require__.c = __webpack_module_cache__;
/******/ 	
/************************************************************************/
/******/ 	/* webpack/runtime/define property getters */
/******/ 	!function() {
/******/ 		// define getter functions for harmony exports
/******/ 		__webpack_require__.d = function(exports, definition) {
/******/ 			for(var key in definition) {
/******/ 				if(__webpack_require__.o(definition, key) && !__webpack_require__.o(exports, key)) {
/******/ 					Object.defineProperty(exports, key, { enumerable: true, get: definition[key] });
/******/ 				}
/******/ 			}
/******/ 		};
/******/ 	}();
/******/ 	
/******/ 	/* webpack/runtime/ensure chunk */
/******/ 	!function() {
/******/ 		__webpack_require__.f = {};
/******/ 		// This file contains only the entry chunk.
/******/ 		// The chunk loading function for additional chunks
/******/ 		__webpack_require__.e = function(chunkId) {
/******/ 			return Promise.all(Object.keys(__webpack_require__.f).reduce(function(promises, key) {
/******/ 				__webpack_require__.f[key](chunkId, promises);
/******/ 				return promises;
/******/ 			}, []));
/******/ 		};
/******/ 	}();
/******/ 	
/******/ 	/* webpack/runtime/get javascript chunk filename */
/******/ 	!function() {
/******/ 		// This function allow to reference async chunks
/******/ 		__webpack_require__.u = function(chunkId) {
/******/ 			// return url for filenames based on template
/******/ 			return "" + chunkId + ".js";
/******/ 		};
/******/ 	}();
/******/ 	
/******/ 	/* webpack/runtime/harmony module decorator */
/******/ 	!function() {
/******/ 		__webpack_require__.hmd = function(module) {
/******/ 			module = Object.create(module);
/******/ 			if (!module.children) module.children = [];
/******/ 			Object.defineProperty(module, 'exports', {
/******/ 				enumerable: true,
/******/ 				set: function() {
/******/ 					throw new Error('ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: ' + module.id);
/******/ 				}
/******/ 			});
/******/ 			return module;
/******/ 		};
/******/ 	}();
/******/ 	
/******/ 	/* webpack/runtime/hasOwnProperty shorthand */
/******/ 	!function() {
/******/ 		__webpack_require__.o = function(obj, prop) { return Object.prototype.hasOwnProperty.call(obj, prop); }
/******/ 	}();
/******/ 	
/******/ 	/* webpack/runtime/make namespace object */
/******/ 	!function() {
/******/ 		// define __esModule on exports
/******/ 		__webpack_require__.r = function(exports) {
/******/ 			if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 				Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 			}
/******/ 			Object.defineProperty(exports, '__esModule', { value: true });
/******/ 		};
/******/ 	}();
/******/ 	
/******/ 	/* webpack/runtime/require chunk loading */
/******/ 	!function() {
/******/ 		// no baseURI
/******/ 		
/******/ 		// object to store loaded chunks
/******/ 		// "1" means "loaded", otherwise not loaded yet
/******/ 		var installedChunks = {
/******/ 			869: 1
/******/ 		};
/******/ 		
/******/ 		// no on chunks loaded
/******/ 		
/******/ 		var installChunk = function(chunk) {
/******/ 			var moreModules = chunk.modules, chunkIds = chunk.ids, runtime = chunk.runtime;
/******/ 			for(var moduleId in moreModules) {
/******/ 				if(__webpack_require__.o(moreModules, moduleId)) {
/******/ 					__webpack_require__.m[moduleId] = moreModules[moduleId];
/******/ 				}
/******/ 			}
/******/ 			if(runtime) runtime(__webpack_require__);
/******/ 			for(var i = 0; i < chunkIds.length; i++)
/******/ 				installedChunks[chunkIds[i]] = 1;
/******/ 		
/******/ 		};
/******/ 		
/******/ 		// require() chunk loading for javascript
/******/ 		__webpack_require__.f.require = function(chunkId, promises) {
/******/ 			// "1" is the signal for "already loaded"
/******/ 			if(!installedChunks[chunkId]) {
/******/ 				if(true) { // all chunks have JS
/******/ 					installChunk(require("./" + __webpack_require__.u(chunkId)));
/******/ 				} else installedChunks[chunkId] = 1;
/******/ 			}
/******/ 		};
/******/ 		
/******/ 		// no external install chunk
/******/ 		
/******/ 		// no HMR
/******/ 		
/******/ 		// no HMR manifest
/******/ 	}();
/******/ 	
/******/ 	/* webpack/runtime/wasm chunk loading */
/******/ 	!function() {
/******/ 		// object to store loaded and loading wasm modules
/******/ 		var installedWasmModules = {};
/******/ 		
/******/ 		function promiseResolve() { return Promise.resolve(); }
/******/ 		
/******/ 		var wasmImportedFuncCache0;
/******/ 		var wasmImportedFuncCache1;
/******/ 		var wasmImportedFuncCache2;
/******/ 		var wasmImportedFuncCache3;
/******/ 		var wasmImportedFuncCache4;
/******/ 		var wasmImportedFuncCache5;
/******/ 		var wasmImportObjects = {
/******/ 			862: function() {
/******/ 				return {
/******/ 					"./index_bg.js": {
/******/ 						"__wbindgen_json_parse": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache0 === undefined) wasmImportedFuncCache0 = __webpack_require__.c[464].exports;
/******/ 							return wasmImportedFuncCache0["t$"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbindgen_object_drop_ref": function(p0i32) {
/******/ 							if(wasmImportedFuncCache1 === undefined) wasmImportedFuncCache1 = __webpack_require__.c[464].exports;
/******/ 							return wasmImportedFuncCache1["ug"](p0i32);
/******/ 						},
/******/ 						"__wbg_new_693216e109162396": function() {
/******/ 							if(wasmImportedFuncCache2 === undefined) wasmImportedFuncCache2 = __webpack_require__.c[464].exports;
/******/ 							return wasmImportedFuncCache2["Ih"]();
/******/ 						},
/******/ 						"__wbg_stack_0ddaca5d1abfb52f": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache3 === undefined) wasmImportedFuncCache3 = __webpack_require__.c[464].exports;
/******/ 							return wasmImportedFuncCache3["yq"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbg_error_09919627ac0992f5": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache4 === undefined) wasmImportedFuncCache4 = __webpack_require__.c[464].exports;
/******/ 							return wasmImportedFuncCache4["gk"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbindgen_throw": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache5 === undefined) wasmImportedFuncCache5 = __webpack_require__.c[464].exports;
/******/ 							return wasmImportedFuncCache5["Or"](p0i32,p1i32);
/******/ 						}
/******/ 					}
/******/ 				};
/******/ 			},
/******/ 		};
/******/ 		
/******/ 		var wasmModuleMap = {
/******/ 			"734": [
/******/ 				862
/******/ 			]
/******/ 		};
/******/ 		
/******/ 		// object with all WebAssembly.instance exports
/******/ 		__webpack_require__.w = {};
/******/ 		
/******/ 		// Fetch + compile chunk loading for webassembly
/******/ 		__webpack_require__.f.wasm = function(chunkId, promises) {
/******/ 		
/******/ 			var wasmModules = wasmModuleMap[chunkId] || [];
/******/ 		
/******/ 			wasmModules.forEach(function(wasmModuleId, idx) {
/******/ 				var installedWasmModuleData = installedWasmModules[wasmModuleId];
/******/ 		
/******/ 				// a Promise means "currently loading" or "already loaded".
/******/ 				if(installedWasmModuleData)
/******/ 					promises.push(installedWasmModuleData);
/******/ 				else {
/******/ 					var importObject = wasmImportObjects[wasmModuleId]();
/******/ 					var req = new Promise(function (resolve, reject) {
/******/ 						var { readFile } = require('fs');
/******/ 						var { join } = require('path');
/******/ 		
/******/ 						try {
/******/ 							readFile(join(__dirname, "static/wasm/" + {"734":{"862":"79176683a193ae77a89f"}}[chunkId][wasmModuleId] + ".wasm"), function(err, buffer){
/******/ 								if (err) return reject(err);
/******/ 		
/******/ 								// Fake fetch response
/******/ 								resolve({
/******/ 									arrayBuffer() { return buffer; }
/******/ 								});
/******/ 							});
/******/ 						} catch (err) { reject(err); }
/******/ 					});
/******/ 					var promise;
/******/ 					if(importObject instanceof Promise) {
/******/ 						var bytesPromise = req.then(function(x) { return x.arrayBuffer(); });
/******/ 						promise = Promise.all([
/******/ 							bytesPromise.then(function(bytes) { return WebAssembly.compile(bytes); }),
/******/ 							importObject
/******/ 						]).then(function(items) {
/******/ 							return WebAssembly.instantiate(items[0], items[1]);
/******/ 						});
/******/ 					} else {
/******/ 						var bytesPromise = req.then(function(x) { return x.arrayBuffer(); });
/******/ 						promise = bytesPromise.then(function(bytes) {
/******/ 							return WebAssembly.instantiate(bytes, importObject);
/******/ 						});
/******/ 					}
/******/ 					promises.push(installedWasmModules[wasmModuleId] = promise.then(function(res) {
/******/ 						return __webpack_require__.w[wasmModuleId] = (res.instance || res).exports;
/******/ 					}));
/******/ 				}
/******/ 			});
/******/ 		};
/******/ 	}();
/******/ 	
/************************************************************************/
/******/ 	
/******/ 	// module cache are used so entry inlining is disabled
/******/ 	// startup
/******/ 	// Load entry module and return exports
/******/ 	var __webpack_exports__ = __webpack_require__(869);
/******/ 	module.exports = __webpack_exports__;
/******/ 	
/******/ })()
;