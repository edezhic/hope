import Chip from '@mui/material/Chip'
import Divider from '@mui/material/Divider'
import * as STYLES from '../styles'

export default function TokenChip({ token, index }: any) {
  let color = STYLES.DEFAULT_COLOR;
  let label = '';
  if (token === 'Linebreak') return (<Divider key={index} sx={STYLES.INVISIBLE_DIVIDER} />)
  else if (token === 'Dot') { label = '.'; color = STYLES.FLOW_COLOR }
  else if (token === 'This') { label = 'this'; color = STYLES.VALUE_COLOR }
  else if (token === 'CollectionEnd') { label = '|'; color = STYLES.COLLECTION_COLOR }
  else if (token?.V?.Lst) { label = '['; color = STYLES.COLLECTION_COLOR }
  else if (typeof (token) === 'string') label = token.toLowerCase();
  else if (token.Term) { label = token.Term; color = STYLES.TERM_COLOR }
  else if ('F' in token) { label = token.F.toLowerCase(); color = STYLES.FUNC_COLOR }
  else if ('P' in token) { label = token.P.toLowerCase(); color = STYLES.PREP_COLOR }
  else if ('C' in token) { label = token.C.toLowerCase(); color = STYLES.RELATION_COLOR }
  else if ('A' in token) {
    color = STYLES.FORMULA_COLOR
    label = token.A.toLowerCase()
      .replace('plus', '+')
      .replace('minus', '-')
      .replace('multiplication', '*')
      .replace('division', '/')
  }
  else if ('V' in token) {
    const value = token.V
    color = STYLES.VALUE_COLOR;
    if ('Num' in value) label = value.Num.value
    if ('I' in value) {
      label = '@' + value.I.scheme.Custom + (value.I.domain ? ':' + value.I.domain : '');
    }
    if ('Txt' in value) label = value.Txt
  } else {
    label = JSON.stringify(token).toLowerCase();
  }
  if (index == 0) { color = STYLES.WHITE } // for script name
  return (
    <Chip key={index} size={'small'} label={label} sx={{
      background: 'none',
      height: '20px',
      color,
      "& .MuiChip-label": {
        padding: "0 2px",
      }
    }} />
  )
}