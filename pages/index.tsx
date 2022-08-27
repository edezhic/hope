import React, { useEffect, useRef, useState } from 'react'
import Container from '@mui/material/Container'
import Divider from '@mui/material/Divider'
import * as CONFIG from '../src/config'
import Token from '../src/token'
import ScriptForm from '../src/script_form'
import Graph from '../src/graph'
import { Alert, Chip } from '@mui/material'

const formatError = (e: any) => {
  return JSON.stringify(e).replaceAll('\\"', '').replaceAll('"', '').replaceAll(/[{}]/g, '').replace(':', ':  ')
}

export default function HOPE() {
  const [graph, setGraph] = useState({ nodes: [], edges: [] })
  const [script, setScript] = useState('');
  const [tokens, setTokens] = useState([]);
  const [error, setError] = useState(null);

  const resetResults = () => {
    setTokens([]);
    setGraph({ nodes: [], edges: [] });
  }

  const workerRef = useRef<Worker>();
  useEffect(() => {
    workerRef.current = new Worker(
      new URL('../src/hobot_worker.ts', import.meta.url)
    );
    workerRef.current.addEventListener('message', (evt: any) => {
      switch (evt.data.type) {
        case 'test':
          setScript(evt.data.test)
          return;
        case 'build':
          setError(null)
          setTokens(evt.data.tokens)
          setGraph(evt.data.graph)
          return;
        case 'error':
          resetResults()
          setError(evt.data.e)
          return;
      }
    });
    workerRef.current.postMessage({ type: 'get_test' });
  }, []);
  
  useEffect(() => {
    if (workerRef.current && script != '') {
      workerRef.current.postMessage({ type: 'get_build', script });
    }
  }, [script]);
  
  return (
    <Container maxWidth='md'>
      <Divider sx={CONFIG.DIVIDER} textAlign="right">Script</Divider>
      <ScriptForm script={script} setScript={setScript} />
      {error && (
        <>
          <Alert severity="error">{formatError(error)}</Alert>
        </>
      )}
      {tokens.length > 0 && (
        <>
          <Divider sx={CONFIG.DIVIDER} textAlign="right">Tokens</Divider>
          <Container sx={{ padding: "0 12px !important" }}>
            {tokens?.map((item: any, i) => <Token item={item} key={JSON.stringify(item) + i} i={i} />)}
          </Container>
        </>
      )}
      {graph.nodes.length > 0 && (
        <>
          <Divider sx={CONFIG.DIVIDER} textAlign="right">Graph</Divider>
          <Graph nodes={graph.nodes} edges={graph.edges} />
        </>
      )}
    </Container>
  );
}