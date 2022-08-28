// From https://webpack.js.org/loaders/worker-loader/#loading-without-worker-loader
declare module '*.worker.ts' {
  // You need to change `Worker`, if you specified a different value for the `workerType` option
  class WebpackWorker extends Worker {
    constructor();
  }

  // Uncomment this if you set the `esModule` option to `false`
  // export = WebpackWorker;
  export default WebpackWorker;
}

declare module "react-graph-vis" {
  import { Network, NetworkEvents, Options, Node, Edge, DataSet } from "vis";
  import { Component } from "react";

  export { Network, NetworkEvents, Options, Node, Edge, DataSet } from "vis";

  export interface graphEvents {
    [event: NetworkEvents]: (params?: any) => void;
  }

  //Doesn't appear that this module supports passing in a vis.DataSet directly. Once it does graph can just use the Data object from vis.
  export interface graphData {
    nodes: Node[];
    edges: Edge[];
  }

  export interface NetworkGraphProps {
    graph: graphData;
    options?: Options;
    events?: graphEvents;
    getNetwork?: (network: Network) => void;
    identifier?: string;
    style?: React.CSSProperties;
    getNodes?: (nodes: DataSet) => void;
    getEdges?: (edges: DataSet) => void;
  }

  export interface NetworkGraphState {
    identifier: string;
  }

  export default class NetworkGraph extends Component<
    NetworkGraphProps,
    NetworkGraphState
  > {
    render();
  }
}