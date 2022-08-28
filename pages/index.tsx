import React, { useEffect, useRef, useState } from 'react'
import Container from '@mui/material/Container'
import Divider from '@mui/material/Divider'
import * as CONFIG from '../src/config'
import Token from '../src/token'
import ScriptForm from '../src/script_form'
import Graph from '../src/graph'
import { Alert, Grid } from '@mui/material'

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
    workerRef.current.addEventListener('message', (msg: any) => {
      switch (msg.data.type) {
        case 'test':
          setScript(msg.data.test)
          return;
        case 'build':
          setError(null)
          setTokens(msg.data.tokens)
          setGraph(msg.data.graph)
          return;
        case 'error':
          resetResults()
          setError(msg.data.e)
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
    <Container maxWidth='lg'>
      <Grid container spacing={2}>

        <Grid item xs={6}>
          <Divider sx={CONFIG.DIVIDER}>Script</Divider>
          <ScriptForm script={script} setScript={setScript} />
        </Grid>
        {error && (
          <Grid item xs={12}>
            <Alert severity="error">{formatError(error)}</Alert>
          </Grid>
        )}
        {tokens.length > 0 && (
          <Grid item xs={6}>
            <Divider sx={CONFIG.DIVIDER}>Tokens</Divider>
            <Container sx={{ padding: "12px 12px 0 !important" }}>
              {tokens?.map((item: any, i) => <Token item={item} key={JSON.stringify(item) + i} i={i} />)}
            </Container>
          </Grid>
        )}
      </Grid>
      {graph.nodes.length > 0 && (
        <>
          <Divider sx={CONFIG.DIVIDER}>Graph</Divider>
          <Graph nodes={graph.nodes} edges={graph.edges} />
        </>
      )}
    </Container>
  );
}