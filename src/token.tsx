import Chip from '@mui/material/Chip'
import Divider from '@mui/material/Divider'
import * as CONFIG from './config'

export default function Token ({ item, i } : any) {
    let [key, token] = item; 
    let color = CONFIG.FLOW_COLOR;
    let label = '';
    if (token === 'Break') return (<Divider key={key} sx={CONFIG.INVISIBLE_DIVIDER} />)
    else if (token === 'This') { label = 'result'; color = CONFIG.VALUE_COLOR }
    else if (token === 'ListEnd') { label = ']'; color = CONFIG.COLLECTION_COLOR }
    else if (token === 'ListStart') { label = '['; color = CONFIG.COLLECTION_COLOR }
    else if (token === 'StructEnd') { label = '}'; color = CONFIG.COLLECTION_COLOR }
    else if (token === 'StructStart') { label = '{'; color = CONFIG.COLLECTION_COLOR }
    else if (token === 'FormulaEnd') { label = ')'; color = CONFIG.FORMULA_COLOR }
    else if (token === 'FormulaStart') { label = '('; color = CONFIG.FORMULA_COLOR }
    else if (typeof token === 'string') label = token.toLowerCase()
    else if ('Cmd' in token) { label = token.Cmd.toLowerCase(); color = CONFIG.COMMAND_COLOR }
    else if ('Mod' in token) { label = token.Mod.toLowerCase(); color = CONFIG.MOD_COLOR }
    else if ('Op' in token) {
      color = CONFIG.FORMULA_COLOR
      label = token.Op.toLowerCase()
        .replace('plus', '+')
        .replace('minus', '-')
        .replace('multiplication', '*')
        .replace('division', '/')
    }
    else if ('Value' in token) {
      const value = token.Value
      color = CONFIG.VALUE_COLOR;
      if ('Number' in value) label = value.Number.value
      if ('Id' in value) { 
        if (value.Id.scheme === 'Ref') {
          label = value.Id.domain.toLowerCase()
        } 
        else label = value.Id.scheme.Custom 
      }
      if ('Text' in value) label = value.Text
      if ('Fact' in value) label = value.Fact.toString()
    }
    if (i == 0) { color = CONFIG.WHITE } // for script name
    return (<Chip key={key} size={'small'} label={label} sx={{ background: 'none', color }}/>)
  }