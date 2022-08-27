export const GRAPH_STYLE = { height: '600px', border: '1px dashed #3a3a3a', borderRadius: "4px" }

export const SCRIPT_BODY_ROWS = 3

export const WHITE = '#fff'
export const WHITISH = '#ddd'
export const GREYISH = '#bbb'
export const GREY = '#888'
export const BLACKISH = '#121212'

export const BACKGROUND_COLOR = BLACKISH
export const DEFAULT_COLOR = GREYISH
export const CONTROL_COLOR = '#c586c0'
export const FUNC_COLOR = '#ffec0c' //'#42d842'
export const COLLECTION_COLOR = '#00c184'
export const FORMULA_COLOR = '#8f0'
export const RELATION_COLOR = '#226feb'
export const VALUE_COLOR = '#ce9278'
export const PREP_COLOR = '#0082ab'
export const TERM_COLOR = '#73e2ff'
export const YES_COLOR = '#0f0'
export const NO_COLOR = '#f00'

export const DIVIDER = { margin: '1em 0 0.5em 0' }
export const INVISIBLE_DIVIDER = { margin: '0', opacity: 0 }

export const SCRIPT_NODE = { x: 0, y: 0, mass: 10, font: { color: WHITISH, size: 28 } }
export const TERM_NODE = {  font: { color: TERM_COLOR } }
export const FUNC_NODE = {  font: { color: FUNC_COLOR } }
export const COLLECTION_NODE = {  font: { color: COLLECTION_COLOR } }
export const FORMULA_NODE = {  font: { color: FORMULA_COLOR } }
export const CONTROL_NODE = {  font: { color: CONTROL_COLOR } }
export const RELATION_NODE = {  font: { color: RELATION_COLOR } }
export const VALUE_NODE = { font: { color: VALUE_COLOR } }

export const EDGE = { color: CONTROL_COLOR, dashes: true }
export const ARG_EDGE = { length: 5, color: VALUE_COLOR }
export const INPUT_EDGE = { color: VALUE_COLOR }
export const YES_EDGE = { color: YES_COLOR, dashes: true, label: 'yes' }
export const NO_EDGE = { color: NO_COLOR, dashes: true, label: 'no' }

export const GRAPH_OPTIONS = {
    clickToUse: true,
    nodes: {
      borderWidth: 0,
      color: BACKGROUND_COLOR,
      mass: 1.1,
      font: {
        size: 22,
      },
    },
    edges: {
      color: GREY,
      length: 0.01,
      font: {
        size: 20,
        color: WHITISH,
        strokeWidth: 0,
      },
      smooth: true,
      width: 1,
      arrows: {
        to: {
          enabled: true,
          scaleFactor: 0.6
        }
      }
    },
  };