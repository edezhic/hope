import { useEffect, useRef, useState } from 'react';
import Container from "@mui/material/Container";
import Box from '@mui/material/Box';
import TextField from '@mui/material/TextField';
import Chip from '@mui/material/Chip';
import Divider from '@mui/material/Divider';

function Tokens({ list }: any) {
  console.log(list);
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
                console.log(token[1]);
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

  const [inputValue, setInputValue] = useState("X is 0.5, y is 1.5. Add x to y, show result.");
  const [tokens, setTokens] = useState([]);
  const workerRef = useRef<Worker>();

  useEffect(() => {
    workerRef.current = new Worker(
      new URL('../src/worker.ts', import.meta.url)
    );
    workerRef.current.addEventListener('message', (evt) => {
      console.log(evt.data.tokens);
      setTokens(evt.data.tokens);
    });
    workerRef.current.postMessage({ type: 'translate', script: inputValue });
  }, [inputValue]);

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
          defaultValue="Default Value"
          onChange={(event) => {
            setInputValue(event.target.value);
          }}
          value={inputValue}
        />
      </Box>
      <Divider sx={{ marginBottom: 2, marginTop: 4 }}>Tokens</Divider>
      <Tokens list={tokens} />
    </Container>
  );
}
