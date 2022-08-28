import * as STYLES from './styles'

// To avoid client/server id mismatch
import dynamic from 'next/dynamic'
const ReactGraphVis = dynamic(() => import("react-graph-vis"), { ssr: false })


export default function Graph ({ nodes, edges }: any) {
    if (nodes.length == 0) return null;
    
    const normalized_nodes = nodes.map((node: any, index: number) => {
        const id = index
        //console.log(node)
        if (id == 0) return { id, label: node.Term, ...STYLES.SCRIPT_NODE }
        if (node == 'This') return { id, label: 'result', ...STYLES.VALUE_NODE }
        if (node.Term) return { id, label: node.Term.toLowerCase(), ...STYLES.TERM_NODE }
        if (node.F) return { id, label: node.F.toLowerCase(), ...STYLES.FUNC_NODE }
        if (node.V) return {
            id,
            label: 
                node.V.Lst
                ? '[ ]'
                : node.V.I
                ? '@' + node.V.I?.scheme?.Custom
                : node.V.Num?.value || node.V,
            ...STYLES.VALUE_NODE
        }
        return { id, label: node, ...STYLES.FUNC_NODE }
    })

    const normalized_edges = edges.map((edge: any) => {
        //console.log(edge)
        const from = edge[0]
        const to = edge[1]
        if (edge[2].P) return { from, to, label: edge[2].P.toLowerCase(), ...STYLES.ARG_EDGE }
        if (edge[2] == 'Input') return { from, to, ...STYLES.INPUT_EDGE }
        if (edge[2] == 'Edge') return { from, to, ...STYLES.EDGE }
    })
    //console.log(normalized_edges, normalized_nodes)
    return <ReactGraphVis graph={{nodes: normalized_nodes, edges: normalized_edges} as any} options={STYLES.GRAPH_OPTIONS} style={STYLES.GRAPH_STYLE} />
}
