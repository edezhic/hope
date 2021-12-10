import { useEffect, useRef, useState } from 'react';
import Container from "@mui/material/Container";
import Box from '@mui/material/Box';
import TextField from '@mui/material/TextField';
import Chip from '@mui/material/Chip';
import Divider from '@mui/material/Divider';

export default function Playground() {
  const [script, setScript] = useState('Column is [1.333, 2, 3,5], structure is {column, flag: yes}. Sort column of structure, sum it, show and send to @scheme://authority/path/ . If the code of the result is "200", then sign the structure with key and save it at @structures/bestOne');
  const [tokens, setTokens] = useState([]);
  const workerRef = useRef<Worker>();
  useEffect(() => {
    workerRef.current = new Worker(
      new URL('../src/worker.ts', import.meta.url)
    );
    workerRef.current.addEventListener('message', (evt: { data: { tokens: any; }; }) => {
      setTokens(evt.data.tokens);
    });
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
      <Divider sx={{ margin: '1em 0 0.5em 0' }}>Tokens</Divider>
      <div className="tokens">
        {
          tokens.map((item: any) => {
            let key = item[0];
            let token = item[1];
            let className: "default" | "N" | "V" | "S" | "M" = "default";
            let label = "";
            if (token === "Break") return (<Divider key={key} sx={{ margin: '0.1em 0 0.5em 0' }} />);
            else if (token === "Being") label = "=";
            else if (token === 'This') { label = "result"; className = "N"; }
            else if (token === 'ListEnd') label = "]";
            else if (token === 'ListStart') label = "[";
            else if (token === 'StructEnd') label = "}";
            else if (token === 'StructStart') label = "{";
            else if (typeof token === "string") label = token.toLowerCase();
            else if ('N' in token) { label = token.N.toLowerCase(); className = "N"; }
            else if ('S' in token) { label = token.S.toLowerCase(); className = "S"; }
            else if ('M' in token) { label = token.M.toLowerCase(); className = "M"; }
            else if ('V' in token) {
              className = "V";
              if ("Number" in token.V) label = token.V.Number.value;
              if ("Id" in token.V) label = token.V.Id.scheme.Custom;
              if ("Text" in token.V) label = token.V.Text;
              if ("Fact" in token.V) label = token.V.Fact.toString();
            }
            return (<Chip key={key} size={"small"} className={className} label={label}></Chip>);
          })
        }
        <Divider sx={{ margin: '0.1em 0 0.5em 0' }} />
      </div>
    </Container>
  );
}
