import { useEffect, useRef, useState } from 'react';
import Container from "@mui/material/Container";
import Box from '@mui/material/Box';
import TextField from '@mui/material/TextField';
import Chip from '@mui/material/Chip';
import Divider from '@mui/material/Divider';

function Tokens({ list }: any) {
  return (
    <div className="tokens">
      {
        list
          .map((token: any) => {
            switch (typeof token[1]) {
              case "string":
                let label = token[1];
                if (label === 'Being') label = '=';
                if (label === 'Break') label = '.';
                if (label === 'This') return (<Chip key={token[0]} sx={{ backgroundColor: 'lightblue' }} size="small" label='it'></Chip>);
                return (
                  <Chip key={token[0]} sx={{ border: 'none' }} size="small" variant="outlined" label={label}></Chip>);
              default:
                if ('N' in token[1]) return (<Chip key={token[0]} sx={{ backgroundColor: 'lightblue' }} size="small" label={token[1].N}></Chip>);
                if ('C' in token[1]) return (<Chip key={token[0]} sx={{ backgroundColor: 'lightgreen' }} size="small" label={token[1].C}></Chip>);
                if ('M' in token[1]) return (<Chip key={token[0]} sx={{ backgroundColor: 'pink' }} size="small" label={token[1].M}></Chip>);
                if ('V' in token[1]) return (<Chip key={token[0]} size="small" label={JSON.stringify(token[1].V, null, 0)}></Chip>);
                return (
                  <Chip key={token[0]} size="small" label={JSON.stringify(token[1], null, 0)}></Chip>);
            }
          })
      }
    </div>
  );
}

export default function Home() {
  const [script, setScript] = useState("X is 0.5, y is 1.5. Add x to y, show result.");
  const [tokens, setTokens] = useState([]);
  const workerRef = useRef<Worker>();
  useEffect(() => {
    workerRef.current = new Worker(
      new URL('../src/worker.ts', import.meta.url)
    );
    workerRef.current.addEventListener('message', (evt: { data: { tokens: any; }; }) => {
      setTokens(evt.data.tokens);
    });
    workerRef.current.postMessage({ type: 'translate', script: script });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);
  useEffect(() => {
    if (workerRef.current) {
      workerRef.current.postMessage({ type: 'translate', script: script });
    }
  }, [script]);

  return (
    <Container maxWidth="md">
      <Box
        component="form"
        noValidate
        autoComplete="off"
      >
        <TextField
          id="outlined-multiline-static"
          label="Script"
          sx={{ marginBottom: 2, marginTop: 4 }}
          multiline
          rows={4}
          fullWidth
          onChange={(event: { target: { value: any; }; }) => {
            setScript(event.target.value);
          }}
          value={script}
        />
      </Box>
      <Divider sx={{ marginBottom: 2, marginTop: 4 }}>Tokens</Divider>
      <Tokens list={tokens} />
    </Container>
  );
}
