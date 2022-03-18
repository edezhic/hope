import React, { useEffect, useRef, useState } from 'react';
import Container from "@mui/material/Container";
import Box from '@mui/material/Box';
import TextField from '@mui/material/TextField';
import Chip from '@mui/material/Chip';
import Divider from '@mui/material/Divider';
import Graph from "react-graph-vis";

const GRAPH_STYLE = { height: "650px" };

const SCRIPTBODYROWS = 2;

const options = {
  clickToUse: true,
  interaction: {
    //zoomView: false
  },
  nodes: {
    borderWidth: 0,
    color: '#121212',
    mass: 0.3,
    font: {
      color: "#42d842",
      size: 22,
    },
  },
  edges: {
    color: "#888",
    font: {
      size: 18,
      color: "#0f0",
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

const DEFAULT_TEST = 0;

export default function IDE() {
  
  const [state, setState] = useState({
    counter: 5,
    graph: {
      nodes: [
        { id: "graphscript", label: "graphscript", font: { color: "#fff", size: 30 }, mass: 4 },
        { id: "assign_z", label: "=", mass: 4 },
        { id: "list", label: "list", mass: 4, font: { color: "#00c184" } },
        { id: "assign_xyz", label: "=", mass: 4 },
        { id: "formula", label: "x + y + z", mass: 4, font: { color: "#88ff00" } },
        { id: "assign_s", label: "=", mass: 4 },
        { id: "show1", label: "show", mass: 4 },
        { id: "sum", label: "sum", mass: 4 },
        { id: "show2", label: "show", mass: 4 },
        { id: "if", label: "if", mass: 4, font: { color: "#dacc29" } }, 
        { id: "less", label: "less", mass: 4, font: { color: "#a500da" } },
        { id: "more", label: "more", mass: 4, font: { color: "#a500da" } },
        { id: "show3", label: "show", mass: 4 },
        { id: "struct", label: "struct", mass: 4, font: { color: "#00c184" } },
        { id: "when", label: "when", mass: 4, font: { color: "#dacc29" } },
        { id: "x1", label: "x", font: { color: "#41afd2" } },
        { id: "y1", label: "(of) y", font: { color: "#41afd2" } },
        { id: "z1", label: "z", font: { color: "#41afd2" } },
        { id: "x2", label: "x", font: { color: "#41afd2" } },
        { id: "y2", label: "y", font: { color: "#41afd2" } },
        { id: "z2", label: "z", font: { color: "#41afd2" } },
        { id: "x3", label: "x", font: { color: "#41afd2" } },
        { id: "y3", label: "y", font: { color: "#41afd2" } },
        { id: "z3", label: "z", font: { color: "#41afd2" } },
        { id: "x4", label: "x", font: { color: "#41afd2" } },
        { id: "y4", label: "y", font: { color: "#41afd2" } },
        { id: "x5", label: "x", font: { color: "#41afd2" } },
        { id: "y5", label: "y", font: { color: "#41afd2" } },
        { id: "z5", label: "z", font: { color: "#41afd2" } },
        { id: "s1", label: "s", font: { color: "#41afd2" } },
        { id: "s2", label: "s", font: { color: "#41afd2" } },
        { id: "s3", label: "s", font: { color: "#41afd2" } },
        { id: "xyz1", label: "xyz", font: { color: "#41afd2" } },
        { id: "xyz2", label: "xyz", font: { color: "#41afd2" } },
        { id: "xyz3", label: "xyz", font: { color: "#41afd2" } },
        { id: "it1", label: "it", font: { color: "#41afd2" } },
        { id: "it2", label: "it", font: { color: "#41afd2" } },
        { id: "it3", label: "it", font: { color: "#41afd2" } },
        { id: "it4", label: "it", font: { color: "#41afd2" } },
        { id: "it5", label: "it", font: { color: "#41afd2" } },
        { id: "it6", label: "it", font: { color: "#41afd2" } },
        { id: "it7", label: "it", font: { color: "#41afd2" } },
        { id: "it8", label: "it", font: { color: "#41afd2" } },
        { id: 1, label: "1", font: { color: "#bbb" } },
        { id: 2, label: "2", font: { color: "#bbb" } },
        { id: 3, label: "3", font: { color: "#bbb" } },
        
      ],
      edges: [ // control: color: "#dacc29"
        { from: "graphscript", to: "x1", color: "#fff" },
        { from: "graphscript", to: "y1", color: "#fff" },
        { from: "graphscript", to: "assign_z", color: "#dacc29", dashes: true },
        { from: 1, to: "assign_z" },
        { from: "assign_z", to: "z1", color: "#fff" },
        { from: "assign_z", to: "list", color: "#dacc29", dashes: true },
        { from: "x2", to: "list" },
        { from: "y2", to: "list" },
        { from: "z2", to: "list" },
        { from: "list", to: "it7", color: "#fff" },
        { from: "list", to: "assign_xyz", color: "#dacc29", dashes: true },
        { from: "it8", to: "assign_xyz" },
        { from: "assign_xyz", to: "xyz1", color: "#fff" },
        { from: "assign_xyz", to: "formula", color: "#dacc29", dashes: true },
        { from: "x3", to: "formula" },
        { from: "y3", to: "formula" },
        { from: "z3", to: "formula" },
        { from: "formula", to: "it1", color: "#fff" },
        { from: "it2", to: "assign_s" },
        { from: "formula", to: "assign_s", color: "#dacc29", dashes: true },
        { from: "assign_s", to: "s1", color: "#fff" },
        { from: "assign_s", to: "show1", color: "#dacc29" },
        { from: "s2", to: "show1" },
        { from: "show1", to: "sum", color: "#dacc29", dashes: true },
        { from: "xyz2", to: "sum" },
        { from: "sum", to: "show2", color: "#dacc29", dashes: true },
        { from: "sum", to: "it3", color: "#fff" },
        { from: "it4", to: "show2" },
        { from: "show2", to: "if", color: "#dacc29" },
        { from: "if", to: "less", color: "#dacc29", dashes: true },
        { from: "x4", to: "less" },
        { from: 2, to: "less" },
        { from: "less", to: "more", color: "#0f0", dashes: true, label: 'yes' },
        { from: "y4", to: "more" },
        { from: 3, to: "more" },
        { from: "more", to: "struct", color: "#0f0", dashes: true, label: 'yes' },
        { from: "show", to: "if", color: "#dacc29", dashes: true },
        { from: "if", to: "when", color: "#dacc29", dashes: true },
        { from: "struct", to: "show3", color: "#dacc29", dashes: true },
        { from: "struct", to: "it5", color: "#fff" },
        { from: "it6", to: "show3" },
        { from: "x5", to: "struct" },
        { from: "y5", to: "struct" },
        { from: "z5", to: "struct" },
        { from: "s3", to: "struct" },
        { from: "xyz3", to: "struct" },
        { from: "show3", to: "if", color: "#dacc29" },
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
      <Divider sx={{ margin: '1em 0 0.5em 0' }}>Graph</Divider>
      <Graph graph={graph as any} options={options} style={GRAPH_STYLE} />
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