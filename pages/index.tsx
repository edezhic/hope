import React, { useEffect, useRef, useState } from 'react';
import Container from "@mui/material/Container";
import Box from '@mui/material/Box';
import TextField from '@mui/material/TextField';
import Chip from '@mui/material/Chip';
import Divider from '@mui/material/Divider';
import Graph from "react-graph-vis";

const GRAPH_STYLE = { height: "600px" };

const SCRIPTBODYROWS = 2;

const options = {
  clickToUse: true,
  interaction: {
    zoomView: false
  },
  nodes: {
    borderWidth: 0,
    color: '#121212',
    font: {
      color: "#eee",
      size: 22,
    },
  },
  edges: {
    color: "#eee",
  },
  
};

const DEFAULT_TEST = 0;

export default function IDE() {
  
  const [state, setState] = useState({
    counter: 5,
    graph: {
      nodes: [
        { id: "graphscript", label: "graphscript", font: { color: "#42d842" }, mass: 4 },
        { id: "x1", label: "x", font: { color: "#41afd2" } },
        { id: "y1", label: "y", font: { color: "#41afd2" } },
        { id: "z1", label: "z", font: { color: "#41afd2" } },
        { id: "x2", label: "x", font: { color: "#41afd2" } },
        { id: "y2", label: "y", font: { color: "#41afd2" } },
        { id: "z2", label: "z", font: { color: "#41afd2" } },
        { id: "x3", label: "x", font: { color: "#41afd2" } },
        { id: "y3", label: "y", font: { color: "#41afd2" } },
        { id: "z3", label: "z", font: { color: "#41afd2" } },
        { id: "it1", label: "it", font: { color: "#41afd2" } },
        { id: "assign_z", label: "assign", font: { color: "#42d842" }, mass: 4 },
        { id: 1, label: "1", font: { color: "#ddd" } },
        { id: "xyz", label: "xyz", font: { color: "#41afd2" } },
        { id: "assign_xyz", label: "assign", font: { color: "#42d842" }, mass: 4 },
        { id: "s", label: "s", font: { color: "#41afd2" } },
        { id: "assign_s", label: "assign", font: { color: "#42d842" }, mass: 4 },
        { id: "eval_s", label: "eval", font: { color: "#42d842" }, mass: 4 },
      ],
      edges: [ // control: color: "#dacc29"
        { from: "graphscript", to: "x1", dashes: true },
        { from: "graphscript", to: "y1", dashes: true, label: "of" },
        { from: "graphscript", to: "assign_z", shadow: { enabled: true, color: "#fff"}, color: "#dacc29" },
        { from: 1, to: "assign_z", dashes: true },
        { from: "assign_z", to: "z1", dashes: true },
        { from: "assign_z", to: "assign_xyz", shadow: { enabled: true, color: "#fff"}, color: "#dacc29" },
        { from: "x2", to: "assign_xyz", dashes: true },
        { from: "y2", to: "assign_xyz", dashes: true },
        { from: "z2", to: "assign_xyz", dashes: true },
        { from: "assign_xyz", to: "xyz", dashes: true },
        { from: "assign_xyz", to: "eval_s", shadow: { enabled: true, color: "#fff"}, color: "#dacc29" },
        { from: "x3", to: "eval_s", dashes: true },
        { from: "y3", to: "eval_s", dashes: true },
        { from: "z3", to: "eval_s", dashes: true },
        { from: "eval_s", to: "it1", dashes: true },
        { from: "it1", to: "assign_s", dashes: true },
        { from: "eval_s", to: "assign_s", shadow: { enabled: true, color: "#fff"}, color: "#dacc29" },
        { from: "assign_s", to: "s", dashes: true },
      ]
    }
  })
  const { graph } = state;
  const [script, setScript] = useState(["", ""]);
  const [tokens, setTokens] = useState([]);
  const [currentTest, setCurrentTest] = useState(DEFAULT_TEST);
  const workerRef = useRef<Worker>();
  useEffect(() => {
    workerRef.current = new Worker(
      new URL('../src/worker.ts', import.meta.url)
    );
    workerRef.current.addEventListener('message', (evt: any) => {
      switch (evt.data.type) {
        case 'tokens':
          setTokens(evt.data.tokens);
          return;
        case 'tests':
          setScript(evt.data.tests[currentTest]);
          return;
      }
    });
    workerRef.current.postMessage({ type: 'get_tests' });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);
  useEffect(() => {
    if (workerRef.current) {
      workerRef.current.postMessage({ type: 'build', title: script[0], body: script[1] });
    }
  }, [script]);

  return (
    <Container maxWidth="md">
      <Box
        component="form"
        noValidate
        autoComplete="off"
        onSubmit={(event: any) => event.preventDefault() }
      >
        <TextField
          id="standard-basic"
          label="Title"
          fullWidth
          className="script-title"
          variant="standard"
          onChange={(event: any ) => setScript([event.target.value, script[1]]) }
          value={script[0]}
        />
        <TextField
          id="multiline-static"
          label="Body"
          sx={{ marginBottom: 2, marginTop: 4 }}
          multiline
          rows={SCRIPTBODYROWS}
          fullWidth
          onChange={(event: any ) => setScript([script[0], event.target.value]) }
          value={script[1]}
        />
      </Box>
      <Divider sx={{ margin: '1em 0 0.5em 0' }}>Graph</Divider>
      <Graph graph={graph} options={options} style={GRAPH_STYLE} />
      <Divider sx={{ margin: '1em 0 0.5em 0' }}>Tokens</Divider>
      <div className="tokens">
        { tokens?.map((item: any) => <Token key={JSON.stringify(item)} item={item} />) }
        <Divider sx={{ margin: '0.1em 0 0.5em 0' }} />
      </div>
    </Container>
  );
}


function Token(props: any) {
  let key = props.item[0];
  let token = props.item[1];
  let className: "default" | "N" | "V" | "C" | "M" | "O" = "default";
  let label = "";
  if (token === "Break") return (<Divider key={key} sx={{ margin: '0.1em 0 0.5em 0' }} />);
  else if (token === "Being") label = "be";
  else if (token === 'This') { label = "result"; className = "N"; }
  else if (token === 'ListEnd') label = "]";
  else if (token === 'ListStart') label = "[";
  else if (token === 'StructEnd') label = "}";
  else if (token === 'StructStart') label = "{";
  else if (token === 'FormulaEnd') label = ")";
  else if (token === 'FormulaStart') label = "(";
  else if (typeof token === "string") label = token.toLowerCase();
  else if ('N' in token) { label = token.N.toLowerCase(); className = "N"; }
  else if ('C' in token) { label = token.C.toLowerCase(); className = "C"; }
  else if ('M' in token) { label = token.M.toLowerCase(); className = "M"; }
  else if ('O' in token) {
    className = "O";
    label = token.O.toLowerCase().replace("plus", "+").replace("multiplication", "*");
  }
  else if ('V' in token) {
    className = "V";
    if ("Number" in token.V) label = token.V.Number.value;
    if ("Id" in token.V) label = token.V.Id.scheme.Custom;
    if ("Text" in token.V) label = token.V.Text;
    if ("Fact" in token.V) label = token.V.Fact.toString();
  }
  return (<Chip key={key} size={"small"} className={className} label={label}></Chip>);
}

/*
  const createNode = (x: any, y: any) => {
    const color = "#BBBBBB";
    setState(({ graph: { nodes, edges }, counter, ...rest }) => {
      const id = counter + 1;
      const from = Math.floor(Math.random() * (counter - 1)) + 1;
      return {
        graph: {
          nodes: [
            ...nodes,
            { id, label: `Node ${id}`, color, x, y }
          ],
          edges: [
            ...edges,
            { from, to: id }
          ]
        },
        counter: id,
        ...rest
      }
    });
  }
  */