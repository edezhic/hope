import React, { Suspense, useEffect, useRef, useState } from 'react'
import Container from '@mui/material/Container'
import Divider from '@mui/material/Divider'
import * as CONFIG from '../src/config'
import Token from '../src/token'
import ScriptForm from '../src/script_form'
import Graph from '../src/graph'

export default function HOPE() {
  const [graph, setGraph] = useState({nodes: [], edges: []})
  const [script, setScript] = useState(['', '']);
  const [tokens, setTokens] = useState([]);

  const workerRef = useRef<Worker>();
  useEffect(() => {
    workerRef.current = new Worker(
      new URL('../src/hobot_worker.ts', import.meta.url)
    );
    workerRef.current.addEventListener('message', (evt: any) => {
      if (evt.data.type == 'test') { setScript(evt.data.tests) }
      if (evt.data.type == 'tokens&graph') { setTokens(evt.data.tokens), setGraph(evt.data.graph) }
    });
    workerRef.current.postMessage({ type: 'get_test' });
  }, []);
  useEffect(() => {
    if (workerRef.current && script[0] != '') {
      workerRef.current.postMessage({ type: 'get_build', title: script[0], body: script[1] });
    }
  }, [script]);
  
  return (
    <Container maxWidth='md'>
      <Divider sx={CONFIG.DIVIDER}>Script</Divider>
      <ScriptForm script={script} setScript={setScript} />

      { /* 
      <Divider sx={CONFIG.DIVIDER}>Tokens (example for syntax highlight)</Divider>
      {tokens?.map((item: any, i) => <Token item={item} key={JSON.stringify(item) + i} i={i}/>)}
      */ }
      <Divider sx={CONFIG.DIVIDER}>Graph</Divider>
      <Graph nodes={graph.nodes} edges={graph.edges}/>
    </Container>
  );
}