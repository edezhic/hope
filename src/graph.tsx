import * as CONFIG from './config'

// To avoid client/server id mismatch
import dynamic from 'next/dynamic'
const ReactGraphVis = dynamic(() => import("react-graph-vis"), { ssr: false })


export default function Graph ({ nodes, edges }: any) {
    if (nodes.length == 0) return null;
    
    const normalized_nodes = nodes.map((node: any, index: number) => {
        const id = index
        if (index == 0) return { id, label: node.Value.Id.domain, ...CONFIG.SCRIPT_NODE }
        if (node == 'Be') return { id, label: 'be', ...CONFIG.ASSIGNMENT_NODE }
        if (node == 'ListStart') return { id, label: 'list', ...CONFIG.COLLECTION_NODE }
        if (node == 'This') return { id, label: 'result', ...CONFIG.VALUE_NODE }
        if (node.Value?.Id?.scheme == 'Ref') return { id, label: node.Value.Id.domain, ...CONFIG.VALUE_NODE }
        if (node.Value?.Number) return { id, label: node.Value.Number.value, ...CONFIG.VALUE_NODE }
        if (node.Cmd) return { id, label: node.Cmd, ...CONFIG.COMMAND_NODE }
    })

    const normalized_edges = edges.map((edge: any) => {
        const from = edge[0]
        const to = edge[1]
        if (edge[2].Mod) return { from, to, label: edge[2].Mod, ...CONFIG.ARG_EDGE }
        if (edge[2].Cmd == 'Get') return { from, to, ...CONFIG.GET_EDGE }
        if (edge[2].Cmd == 'Send') return { from, to, ...CONFIG.SET_EDGE }
        if (edge[2] == 'Then') return { from, to, ...CONFIG.THEN_EDGE }
    })

    return <ReactGraphVis graph={{nodes: normalized_nodes, edges: normalized_edges} as any} options={CONFIG.GRAPH_OPTIONS} style={CONFIG.GRAPH_STYLE} />
}
