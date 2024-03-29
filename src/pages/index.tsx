import React, { useEffect, useRef, useState } from 'react'
import Container from '@mui/material/Container'
import Divider from '@mui/material/Divider'
import * as STYLES from '../styles'
import TokenChip from '../components/Token'
import ScriptForm from '../components/Script'
import Graph from '../components/Graph'
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
      new URL('../hobot_worker.ts', import.meta.url)
    );
    workerRef.current.addEventListener('message', ({ data: { type, test, tokens, graph, error } }) => {
      switch (type) {
        case 'test_script':
          setScript(test)
          return;
        case 'build':
          setError(null)
          setTokens(tokens)
          setGraph(graph)
          return;
        case 'error':
          resetResults()
          setError(error)
          return;
      }
    });
  }, []);

  useEffect(() => {
    if (workerRef.current) workerRef.current.postMessage({ type: 'build', script });
  }, [script]);

  return (
    <Container maxWidth='lg'>
      <Grid container spacing={2}>
        <Grid item xs={12} md={6}>
          <Divider sx={STYLES.DIVIDER}>Script</Divider>
          <ScriptForm script={script} setScript={setScript} />
        </Grid>
        {error && (
          <Grid item xs={12}>
            <Alert severity="error">{formatError(error)}</Alert>
          </Grid>
        )}
        {tokens.length > 0 && (
          <Grid item xs={12} md={6}>
            <Divider sx={STYLES.DIVIDER}>Tokens</Divider>
            <Container sx={{ padding: "12px 12px 0 !important" }}>
              {tokens?.map(({ index, token }) => <TokenChip key={index} token={token} index={index} />)}
            </Container>
          </Grid>
        )}
      </Grid>
      {graph.nodes.length > 0 && (
        <Grid item xs={12}>
          <Divider sx={STYLES.DIVIDER}>Graph</Divider>
          <Graph nodes={graph.nodes} edges={graph.edges} />
        </Grid>
      )}
    </Container>
  );
}