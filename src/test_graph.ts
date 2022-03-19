import * as STYLES from './styles'

const SCRIPT_NODE = { mass: 4, font: { color: STYLES.WHITE, size: 30 } }
const ASSIGNMENT_NODE = { mass: 4, font: { color: STYLES.WHITISH } }
const COMMAND_NODE = { mass: 4, font: { color: STYLES.COMMAND_COLOR } }
const COLLECTION_NODE = { mass: 4, font: { color: STYLES.COLLECTION_COLOR } }
const FORMULA_NODE = { mass: 4, font: { color: STYLES.FORMULA_COLOR } }
const FLOW_NODE = { mass: 4, font: { color: STYLES.FLOW_COLOR } }
const CASE_NODE = { mass: 4, font: { color: STYLES.CASE_COLOR } }
const TERM_NODE = { font: { color: STYLES.TERM_COLOR } }
const VALUE_NODE = { font: { color: STYLES.GREYISH } }

const ARG_EDGE = { color: STYLES.GREYISH, dashes: true, arrows: { to: { enabled: false } } }
const SET_EDGE = { color: STYLES.WHITE }
const GET_EDGE = { color: STYLES.GREY }
const THEN_EDGE = { color: STYLES.FLOW_COLOR, dashes: true }
const BREAK_EDGE = { color: STYLES.FLOW_COLOR }
const YES_EDGE = { color: STYLES.YES_COLOR, dashes: true, label: 'yes' }
const NO_EDGE = { color: STYLES.NO_COLOR, dashes: true, label: 'no' }

export const GRAPH_OPTIONS = {
    clickToUse: true,
    nodes: {
      borderWidth: 0,
      color: STYLES.BACKGROUND_COLOR,
      mass: 0.3,
      font: {
        size: 22,
      },
    },
    edges: {
      color: STYLES.GREY,
      font: {
        size: 18,
        color: STYLES.WHITISH,
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

export const GRAPH_DATA = {
    graph: {
        nodes: [
            { id: 'graphscript', label: 'graphscript', ...SCRIPT_NODE },
            { id: 'assign_z', label: 'be', ...ASSIGNMENT_NODE },
            { id: 'list', label: 'list', ...COLLECTION_NODE },
            { id: 'assign_xyz', label: 'be', ...ASSIGNMENT_NODE },
            { id: 'formula', label: 'x + y + z', ...FORMULA_NODE },
            { id: 'assign_s', label: 'be', ...ASSIGNMENT_NODE },
            { id: 'show1', label: 'show', ...COMMAND_NODE },
            { id: 'sum', label: 'sum', ...COMMAND_NODE },
            { id: 'show2', label: 'show', ...COMMAND_NODE },
            { id: 'if', label: 'if', ...FLOW_NODE },
            { id: 'less', label: 'less', ...FLOW_NODE },
            { id: 'more', label: 'more', ...FLOW_NODE },
            { id: 'show3', label: 'show', ...COMMAND_NODE },
            { id: 'struct', label: 'struct', ...COLLECTION_NODE },
            { id: 'when', label: 'when', ...FLOW_NODE },
            { id: 'x1', label: 'x', ...TERM_NODE },
            { id: 'y1', label: 'y', ...TERM_NODE },
            { id: 'z1', label: 'z', ...TERM_NODE },
            { id: 'x2', label: 'x', ...TERM_NODE },
            { id: 'y2', label: 'y', ...TERM_NODE },
            { id: 'z2', label: 'z', ...TERM_NODE },
            { id: 'x3', label: 'x', ...TERM_NODE },
            { id: 'y3', label: 'y', ...TERM_NODE },
            { id: 'z3', label: 'z', ...TERM_NODE },
            { id: 'x4', label: 'x', ...TERM_NODE },
            { id: 'y4', label: 'y', ...TERM_NODE },
            { id: 'x5', label: 'x', ...TERM_NODE },
            { id: 'y5', label: 'y', ...TERM_NODE },
            { id: 'z5', label: 'z', ...TERM_NODE },
            { id: 's1', label: 's', ...TERM_NODE },
            { id: 's2', label: 's', ...TERM_NODE },
            { id: 's3', label: 's', ...TERM_NODE },
            { id: 'xyz1', label: 'xyz', ...TERM_NODE },
            { id: 'xyz2', label: 'xyz', ...TERM_NODE },
            { id: 'xyz3', label: 'xyz', ...TERM_NODE },
            { id: 'it1', label: 'it', ...TERM_NODE },
            { id: 'it2', label: 'it', ...TERM_NODE },
            { id: 'it3', label: 'it', ...TERM_NODE },
            { id: 'it4', label: 'it', ...TERM_NODE },
            { id: 'it5', label: 'it', ...TERM_NODE },
            { id: 'it6', label: 'it', ...TERM_NODE },
            { id: 'it7', label: 'it', ...TERM_NODE },
            { id: 'it8', label: 'it', ...TERM_NODE },
            { id: 1, label: '1', ...VALUE_NODE },
            { id: 2, label: '2', ...VALUE_NODE },
            { id: 3, label: '3', ...VALUE_NODE },

        ],
        edges: [
            { from: 'graphscript', to: 'x1', ...ARG_EDGE },
            { from: 'graphscript', to: 'y1', label: 'of', ...ARG_EDGE },
            { from: 'graphscript', to: 'assign_z', ...THEN_EDGE },
            { from: 1, to: 'assign_z', ...GET_EDGE },
            { from: 'assign_z', to: 'z1', ...SET_EDGE },
            { from: 'assign_z', to: 'list', ...THEN_EDGE },
            { from: 'x2', to: 'list', ...GET_EDGE },
            { from: 'y2', to: 'list', ...GET_EDGE },
            { from: 'z2', to: 'list', ...GET_EDGE },
            { from: 'list', to: 'it7', ...SET_EDGE },
            { from: 'list', to: 'assign_xyz', ...THEN_EDGE },
            { from: 'it8', to: 'assign_xyz', ...GET_EDGE },
            { from: 'assign_xyz', to: 'xyz1', ...SET_EDGE },
            { from: 'assign_xyz', to: 'formula', ...THEN_EDGE },
            { from: 'x3', to: 'formula', ...GET_EDGE },
            { from: 'y3', to: 'formula', ...GET_EDGE },
            { from: 'z3', to: 'formula', ...GET_EDGE },
            { from: 'formula', to: 'it1', ...SET_EDGE },
            { from: 'it2', to: 'assign_s', ...GET_EDGE },
            { from: 'formula', to: 'assign_s', ...THEN_EDGE },
            { from: 'assign_s', to: 's1', ...SET_EDGE },
            { from: 'assign_s', to: 'show1', ...BREAK_EDGE },
            { from: 's2', to: 'show1', ...GET_EDGE },
            { from: 'show1', to: 'sum', ...THEN_EDGE },
            { from: 'xyz2', to: 'sum', ...GET_EDGE },
            { from: 'sum', to: 'show2', ...THEN_EDGE },
            { from: 'sum', to: 'it3', ...SET_EDGE },
            { from: 'it4', to: 'show2', ...GET_EDGE },
            { from: 'show2', to: 'if', ...BREAK_EDGE },
            { from: 'if', to: 'less', ...THEN_EDGE },
            { from: 'x4', to: 'less', ...GET_EDGE },
            { from: 2, to: 'less', ...GET_EDGE },
            { from: 'less', to: 'more', ...YES_EDGE },
            { from: 'y4', to: 'more', ...GET_EDGE },
            { from: 3, to: 'more', ...GET_EDGE },
            { from: 'more', to: 'struct', ...YES_EDGE },
            { from: 'show', to: 'if', ...THEN_EDGE },
            { from: 'if', to: 'when', ...BREAK_EDGE },
            { from: 'struct', to: 'show3', ...THEN_EDGE },
            { from: 'struct', to: 'it5', ...SET_EDGE },
            { from: 'it6', to: 'show3', ...GET_EDGE },
            { from: 'x5', to: 'struct', ...GET_EDGE },
            { from: 'y5', to: 'struct', ...GET_EDGE },
            { from: 'z5', to: 'struct', ...GET_EDGE },
            { from: 's3', to: 'struct', ...GET_EDGE },
            { from: 'xyz3', to: 'struct', ...GET_EDGE },
            { from: 'show3', to: 'if', ...BREAK_EDGE },
        ]
    }
}