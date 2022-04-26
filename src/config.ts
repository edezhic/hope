export const GRAPH_STYLE = { height: '400px' }

export const SCRIPT_BODY_ROWS = 2

export const WHITE = '#fff'
export const WHITISH = '#ddd'
export const GREYISH = '#bbb'
export const GREY = '#888'
export const BLACKISH = '#121212'

export const BACKGROUND_COLOR = BLACKISH
export const FLOW_COLOR = '#dacc29'
export const COMMAND_COLOR = '#42d842'
export const COLLECTION_COLOR = '#00c184'
export const FORMULA_COLOR = '#8f0'
export const CASE_COLOR = '#a500da'
export const VALUE_COLOR = WHITISH
export const MOD_COLOR = '#41afd2'
export const YES_COLOR = '#0f0'
export const NO_COLOR = '#f00'

export const DIVIDER = { margin: '1em 0 0.5em 0' }
export const INVISIBLE_DIVIDER = { margin: '0.1em 0 0.5em 0', opacity: 0 }

export const SCRIPT_NODE = { x: 0, y: 0, fixed: { x: true, y: true }, mass: 10, font: { color: WHITISH, size: 30 } }
export const ASSIGNMENT_NODE = { mass: 4, font: { color: GREYISH } }
export const COMMAND_NODE = { mass: 4, font: { color: COMMAND_COLOR } }
export const COLLECTION_NODE = { mass: 4, font: { color: COLLECTION_COLOR } }
export const FORMULA_NODE = { mass: 4, font: { color: FORMULA_COLOR } }
export const FLOW_NODE = { mass: 4, font: { color: FLOW_COLOR } }
export const CASE_NODE = { mass: 4, font: { color: CASE_COLOR } }
export const VALUE_NODE = { font: { color: VALUE_COLOR } }

export const ARG_EDGE = { color: GREYISH, dashes: true, arrows: { to: { enabled: false } } }
export const SET_EDGE = { color: COMMAND_COLOR }
export const GET_EDGE = { color: MOD_COLOR }
export const THEN_EDGE = { color: WHITISH, dashes: true }
export const BREAK_EDGE = { color: GREYISH }
export const YES_EDGE = { color: YES_COLOR, dashes: true, label: 'yes' }
export const NO_EDGE = { color: NO_COLOR, dashes: true, label: 'no' }

export const GRAPH_OPTIONS = {
    clickToUse: true,
    nodes: {
      borderWidth: 0,
      color: BACKGROUND_COLOR,
      mass: 0.3,
      font: {
        size: 22,
      },
    },
    edges: {
      color: GREY,
      font: {
        size: 18,
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