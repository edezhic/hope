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
        list.map((token: any) => {
          switch (typeof token[1]) {
            case "string":
              let label = token[1];
              if (label === 'Being') return (<Chip key={token[0]} sx={{ border: 'none' }} size="small" variant="outlined" label="="></Chip>);
              else if (label === 'Break') return (<Divider sx={{margin: '0.1em 0 0.5em 0' }} />);
              else if (label === 'This') return (<Chip key={token[0]} sx={{ backgroundColor: 'lightblue' }} size="small" label="result"></Chip>);
              else if (label === 'ListEnd') return (<Chip key={token[0]} sx={{ backgroundColor: '#fff' }} size="small" label="]"></Chip>);
              else if (label === 'ListStart') return (<Chip key={token[0]} sx={{ backgroundColor: '#fff' }} size="small" label="["></Chip>);
              else if (label === 'StructEnd') return (<Chip key={token[0]} sx={{ backgroundColor: '#fff' }} size="small" label="}"></Chip>);
              else if (label === 'StructStart') return (<Chip key={token[0]} sx={{ backgroundColor: '#fff' }} size="small" label="{"></Chip>);
              else return (<Chip key={token[0]} sx={{ backgroundColor: '#fff' }} size="small" label={label.toLowerCase()}></Chip>);
            default:
              if ('N' in token[1]) return (<Chip key={token[0]} sx={{ backgroundColor: 'lightblue' }} size="small" label={token[1].N.toLowerCase()}></Chip>);
              if ('S' in token[1]) return (<Chip key={token[0]} sx={{ backgroundColor: 'lightgreen' }} size="small" label={token[1].S.toLowerCase()}></Chip>);
              if ('M' in token[1]) return (<Chip key={token[0]} sx={{ backgroundColor: 'pink' }} size="small" label={token[1].M.toLowerCase()}></Chip>);
              if ('V' in token[1]) {
                let type = Object.keys(token[1].V)[0];
                switch (type) {
                  case "Number":
                    return (<Chip key={token[0]} size="small" label={token[1].V.Number.value}></Chip>);
                  case "Id":
                    return (<Chip key={token[0]} size="small" label={token[1].V.Id.scheme.Custom}></Chip>);
                  case "Text":
                    return (<Chip key={token[0]} size="small" label={token[1].V.Text}></Chip>);
                  case "Fact":
                    if (token[1].V.Fact == true)
                      return (<Chip key={token[0]} size="small" label="true"></Chip>);
                    else
                      return (<Chip key={token[0]} size="small" label="false"></Chip>);

                }
                return (<Chip key={token[0]} size="small" label={JSON.stringify(token[1].V, null, 0)}></Chip>);
              }
              return (
                <Chip key={token[0]} size="small" label={JSON.stringify(token[1], null, 0)}></Chip>);
          }
        })
      }
      <Divider sx={{margin: '0.1em 0 0.5em 0' }} />
    </div>
  );
}

export default function Home() {
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
