exports.id = 734;
exports.ids = [734];
exports.modules = {

/***/ 862:
/***/ (function(module, __unused_webpack_exports, __webpack_require__) {

"use strict";
"use strict";
// Instantiate WebAssembly module
var wasmExports = __webpack_require__.w[module.id];

// export exports from WebAssembly module
module.exports = wasmExports;
// exec imports from WebAssembly module (for esm order)
/* harmony import */ var m0 = __webpack_require__(464);


// exec wasm module
wasmExports[""]()

/***/ }),

/***/ 734:
/***/ (function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "__wbg_error_09919627ac0992f5": function() { return /* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.gk; },
/* harmony export */   "__wbg_new_693216e109162396": function() { return /* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.Ih; },
/* harmony export */   "__wbg_stack_0ddaca5d1abfb52f": function() { return /* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.yq; },
/* harmony export */   "__wbindgen_json_parse": function() { return /* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.t$; },
/* harmony export */   "__wbindgen_object_drop_ref": function() { return /* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.ug; },
/* harmony export */   "__wbindgen_throw": function() { return /* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.Or; },
/* harmony export */   "tokenize": function() { return /* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.wS; }
/* harmony export */ });
/* harmony import */ var _index_bg_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(464);



/***/ }),

/***/ 464:
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "wS": function() { return /* binding */ tokenize; },
/* harmony export */   "t$": function() { return /* binding */ __wbindgen_json_parse; },
/* harmony export */   "ug": function() { return /* binding */ __wbindgen_object_drop_ref; },
/* harmony export */   "Ih": function() { return /* binding */ __wbg_new_693216e109162396; },
/* harmony export */   "yq": function() { return /* binding */ __wbg_stack_0ddaca5d1abfb52f; },
/* harmony export */   "gk": function() { return /* binding */ __wbg_error_09919627ac0992f5; },
/* harmony export */   "Or": function() { return /* binding */ __wbindgen_throw; }
/* harmony export */ });
/* harmony import */ var _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(862);
/* module decorator */ module = __webpack_require__.hmd(module);

const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;
let cachedTextDecoder = new lTextDecoder('utf-8', {
  ignoreBOM: true,
  fatal: true
});
cachedTextDecoder.decode();
let cachegetUint8Memory0 = null;

function getUint8Memory0() {
  if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer) {
    cachegetUint8Memory0 = new Uint8Array(_index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer);
  }

  return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
  return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

const heap = new Array(32).fill(undefined);
heap.push(undefined, null, true, false);
let heap_next = heap.length;

function addHeapObject(obj) {
  if (heap_next === heap.length) heap.push(heap.length + 1);
  const idx = heap_next;
  heap_next = heap[idx];
  heap[idx] = obj;
  return idx;
}

function getObject(idx) {
  return heap[idx];
}

function dropObject(idx) {
  if (idx < 36) return;
  heap[idx] = heap_next;
  heap_next = idx;
}

function takeObject(idx) {
  const ret = getObject(idx);
  dropObject(idx);
  return ret;
}

let WASM_VECTOR_LEN = 0;
const lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;
let cachedTextEncoder = new lTextEncoder('utf-8');
const encodeString = typeof cachedTextEncoder.encodeInto === 'function' ? function (arg, view) {
  return cachedTextEncoder.encodeInto(arg, view);
} : function (arg, view) {
  const buf = cachedTextEncoder.encode(arg);
  view.set(buf);
  return {
    read: arg.length,
    written: buf.length
  };
};

function passStringToWasm0(arg, malloc, realloc) {
  if (realloc === undefined) {
    const buf = cachedTextEncoder.encode(arg);
    const ptr = malloc(buf.length);
    getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
    WASM_VECTOR_LEN = buf.length;
    return ptr;
  }

  let len = arg.length;
  let ptr = malloc(len);
  const mem = getUint8Memory0();
  let offset = 0;

  for (; offset < len; offset++) {
    const code = arg.charCodeAt(offset);
    if (code > 0x7F) break;
    mem[ptr + offset] = code;
  }

  if (offset !== len) {
    if (offset !== 0) {
      arg = arg.slice(offset);
    }

    ptr = realloc(ptr, len, len = offset + arg.length * 3);
    const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
    const ret = encodeString(arg, view);
    offset += ret.written;
  }

  WASM_VECTOR_LEN = offset;
  return ptr;
}
/**
* @param {string} script
* @returns {any}
*/


function tokenize(script) {
  var ptr0 = passStringToWasm0(script, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);
  var len0 = WASM_VECTOR_LEN;
  var ret = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.tokenize(ptr0, len0);
  return takeObject(ret);
}
let cachegetInt32Memory0 = null;

function getInt32Memory0() {
  if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer) {
    cachegetInt32Memory0 = new Int32Array(_index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer);
  }

  return cachegetInt32Memory0;
}

function __wbindgen_json_parse(arg0, arg1) {
  var ret = JSON.parse(getStringFromWasm0(arg0, arg1));
  return addHeapObject(ret);
}
;
function __wbindgen_object_drop_ref(arg0) {
  takeObject(arg0);
}
;
function __wbg_new_693216e109162396() {
  var ret = new Error();
  return addHeapObject(ret);
}
;
function __wbg_stack_0ddaca5d1abfb52f(arg0, arg1) {
  var ret = getObject(arg1).stack;
  var ptr0 = passStringToWasm0(ret, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);
  var len0 = WASM_VECTOR_LEN;
  getInt32Memory0()[arg0 / 4 + 1] = len0;
  getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}
;
function __wbg_error_09919627ac0992f5(arg0, arg1) {
  try {
    console.error(getStringFromWasm0(arg0, arg1));
  } finally {
    _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_free(arg0, arg1);
  }
}
;
function __wbindgen_throw(arg0, arg1) {
  throw new Error(getStringFromWasm0(arg0, arg1));
}
;

/***/ })

};
;