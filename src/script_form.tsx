import Box from '@mui/material/Box';
import TextField from '@mui/material/TextField'
import * as CONFIG from './config'

export default function ScriptForm(props: any) {
    return (
        <Box
            component='form'
            noValidate
            autoComplete='off'
            onSubmit={(event: any) => event.preventDefault()}
        >
            <TextField
                id='multiline-static'
                sx={{ marginBottom: 0, marginTop: 0, fontSize: "13px !important" }}
                inputProps={{
                    style: { fontSize: 14 }
                }}
                multiline
                rows={CONFIG.SCRIPT_BODY_ROWS}
                fullWidth
                onChange={(event: any) => props.setScript(event.target.value)}
                value={props.script}
                spellCheck={false}
            />
        </Box>
    )
}