import * as CONFIG from './config'

// To avoid client/server id mismatch
import dynamic from 'next/dynamic'
const ReactGraphVis = dynamic(() => import("react-graph-vis"), { ssr: false })


export default function Graph ({ nodes, edges }: any) {
    if (nodes.length == 0) return null;
    
    const normalized_nodes = nodes.map((node: any, index: number) => {
        const id = index
        console.log(node)
        if (id == 0) return { id, label: node.Term, ...CONFIG.SCRIPT_NODE }
        if (node == 'Be') return { id, label: 'be', ...CONFIG.ASSIGNMENT_NODE }
        if (node == 'ListStart') return { id, label: 'list', ...CONFIG.COLLECTION_NODE }
        if (node == 'This') return { id, label: 'result', ...CONFIG.VALUE_NODE }
        //if (node.V?.I?.scheme == 'Ref') return { id, label: node.V.I.domain, ...CONFIG.VALUE_NODE }
        if (node.Term) return { id, label: node.Term, ...CONFIG.VALUE_NODE }
        if (node.F) return { id, label: node.F, ...CONFIG.COMMAND_NODE }
        if (node.V) return {
            id,
            label: 
                node.V.I?.scheme == 'Ref'
                ? node.V.I.domain
                : node.V.Lst
                ? '[ ]'
                : node.V.Num?.value || node.V.I?.scheme?.Custom || node.V,
            ...CONFIG.VALUE_NODE
        }
        return { id, label: node, ...CONFIG.COMMAND_NODE }
    })

    const normalized_edges = edges.map((edge: any) => {
        //console.log(edge)
        const from = edge[0]
        const to = edge[1]
        if (edge[2].P) return { from, to, label: edge[2].P, ...CONFIG.GET_EDGE }
        if (edge[2] == 'Input') return { from, to, ...CONFIG.GET_EDGE }
        //if (edge[2] == 'Send') return { from, to, ...CONFIG.SET_EDGE }
        if (edge[2] == 'Edge') return { from, to, ...CONFIG.THEN_EDGE }
    })
    console.log(normalized_edges, normalized_nodes)
    return <ReactGraphVis graph={{nodes: normalized_nodes, edges: normalized_edges} as any} options={CONFIG.GRAPH_OPTIONS} style={CONFIG.GRAPH_STYLE} />
}
