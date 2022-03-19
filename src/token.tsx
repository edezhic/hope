import Chip from '@mui/material/Chip'
import Divider from '@mui/material/Divider'
import * as S from './styles'

export default function Token (props: any) {
    let [key, token] = props.item; 
    let color = S.FLOW_COLOR;
    let label = '';
    
    if (token === 'Break') return (<Divider key={key} sx={S.INVISIBLE_DIVIDER} />);
    else if (token === 'Being') label = 'be';
    else if (token === 'This') { label = 'result'; color = S.TERM_COLOR; }
    else if (token === 'ListEnd') label = ']';
    else if (token === 'ListStart') label = '[';
    else if (token === 'StructEnd') label = '}';
    else if (token === 'StructStart') label = '{';
    else if (token === 'FormulaEnd') label = ')';
    else if (token === 'FormulaStart') label = '(';
    else if (typeof token === 'string') label = token.toLowerCase();
    else if ('N' in token) { label = token.N.toLowerCase(); color = S.TERM_COLOR; }
    else if ('C' in token) { label = token.C.toLowerCase(); color = S.CASE_COLOR; }
    else if ('M' in token) { label = token.M.toLowerCase(); color = S.MOD_COLOR; }
    else if ('O' in token) {
      color = S.FORMULA_COLOR;
      label = token.O.toLowerCase().replace('plus', '+').replace('multiplication', '*');
    }
    else if ('V' in token) {
      color = S.VALUE_COLOR;
      if ('Number' in token.V) label = token.V.Number.value;
      if ('Id' in token.V) label = token.V.Id.scheme.Custom;
      if ('Text' in token.V) label = token.V.Text;
      if ('Fact' in token.V) label = token.V.Fact.toString();
    }
    return (<Chip key={key} size={'small'} label={label} sx={{ background: 'none', color }}></Chip>);
  }