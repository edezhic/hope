import Chip from '@mui/material/Chip'
import Divider from '@mui/material/Divider'
import * as CONFIG from './config'

export default function Token({ item, i }: any) {
  let [key, token] = item;
  let color = CONFIG.DEFAULT_COLOR;
  let label = '';
  if (token === 'Linebreak') return (<Divider key={key} sx={CONFIG.INVISIBLE_DIVIDER} />)
  else if (token === 'Dot') { label = '.'; color = CONFIG.CONTROL_COLOR }
  else if (token === 'This') { label = 'this'; color = CONFIG.VALUE_COLOR }
  else if (token === 'CollectionEnd') { label = ']/}'; color = CONFIG.COLLECTION_COLOR }
  else if (token?.V?.Lst) { label = '['; color = CONFIG.COLLECTION_COLOR }
  else if (typeof (token) === 'string') label = token.toLowerCase();
  else if (token.Term) { label = token.Term; color = CONFIG.TERM_COLOR }
  else if ('F' in token) { label = token.F.toLowerCase(); color = CONFIG.FUNC_COLOR }
  else if ('P' in token) { label = token.P.toLowerCase(); color = CONFIG.PREP_COLOR }
  else if ('C' in token) { label = token.C.toLowerCase(); color = CONFIG.CONTROL_COLOR }
  else if ('R' in token) { label = token.R.toLowerCase(); color = CONFIG.RELATION_COLOR }
  else if ('A' in token) {
    color = CONFIG.FORMULA_COLOR
    label = token.A.toLowerCase()
      .replace('plus', '+')
      .replace('minus', '-')
      .replace('multiplication', '*')
      .replace('division', '/')
  }
  else if ('V' in token) {
    const value = token.V
    color = CONFIG.VALUE_COLOR;
    if ('Num' in value) label = value.Num.value
    if ('I' in value) {
      label = '@' + value.I.scheme.Custom + (value.I.domain ? ':' + value.I.domain : '');
    }
    if ('Txt' in value) label = value.Txt
  } else {
    label = JSON.stringify(token).toLowerCase();
  }
  if (i == 0) { color = CONFIG.WHITE } // for script name
  return (
    <Chip key={key} size={'small'} label={label} sx={{
      background: 'none',
      color,
      "& .MuiChip-label": {
        padding: "0 3px",
      }
    }} />
  )
}