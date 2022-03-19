import Box from '@mui/material/Box';
import TextField from '@mui/material/TextField'
import * as S from './styles'

export default function ScriptForm(props: any) {
    return (
        <Box
            component='form'
            noValidate
            autoComplete='off'
            onSubmit={(event: any) => event.preventDefault()}
        >
            <TextField
                id='standard-basic'
                label='Title'
                fullWidth
                className='script-title'
                variant='standard'
                onChange={(event: any) => props.setScript([event.target.value, props.script[1]])}
                value={props.script[0]}
            />
            <TextField
                id='multiline-static'
                label='Body'
                sx={{ marginBottom: 2, marginTop: 4 }}
                multiline
                rows={S.SCRIPT_BODY_ROWS}
                fullWidth
                onChange={(event: any) => props.setScript([props.script[0], event.target.value])}
                value={props.script[1]}
            />
        </Box>
    )
}