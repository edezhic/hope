import React, { useEffect, useRef, useState } from 'react';
import Container from '@mui/material/Container';
import Divider from '@mui/material/Divider'
import Graph from 'react-graph-vis'
import { GRAPH_DATA, GRAPH_OPTIONS } from '../src/test_graph'
import * as STYLES from '../src/styles'
import Token from '../src/token'
import ScriptForm from '../src/script_form'

const DEFAULT_TEST = 0;

export default function IDE() {
  const [graphState, setGraphState] = useState(GRAPH_DATA)
  const [script, setScript] = useState(['', '']);
  const [tokens, setTokens] = useState([]);
  const [currentTest, setCurrentTest] = useState(DEFAULT_TEST);
  const workerRef = useRef<Worker>();
  useEffect(() => {
    workerRef.current = new Worker(
      new URL('../src/core_worker.ts', import.meta.url)
    );
    workerRef.current.addEventListener('message', (evt: any) => {
      if (evt.data.type == 'tests') { setScript(evt.data.tests[currentTest]) }
      if (evt.data.type == 'tokens') { setTokens(evt.data.tokens) }
    });
    workerRef.current.postMessage({ type: 'get_tests' });
  }, []);
  useEffect(() => {
    if (workerRef.current) {
      workerRef.current.postMessage({ type: 'build', title: script[0], body: script[1] });
    }
  }, [script]);

  return (
    <Container maxWidth='md'>
      <Divider sx={STYLES.DIVIDER}>Tokens</Divider>
      {tokens?.map((item: any, i) => <Token item={item} key={JSON.stringify(item)} i={i}/>)}

      <Divider sx={STYLES.DIVIDER}>Graph</Divider>
      <Graph graph={graphState.graph as any} options={GRAPH_OPTIONS} style={STYLES.GRAPH_STYLE} />

      <Divider sx={STYLES.DIVIDER}>Script</Divider>
      <ScriptForm script={script} setScript={setScript} />
    </Container>
  );
}