(self.webpackChunk_N_E=self.webpackChunk_N_E||[]).push([[405],{2562:function(t,e,n){"use strict";n.r(e),n.d(e,{default:function(){return l}});var s=n(5893),r=n(7294),a=n(4065),i=n(2094),u=n(8315),o=n(3733),c=n(2116);function l(){var t=(0,r.useState)("Column is [1.333, 2, 3,5], structure is {column, flag: yes}. Sort column of structure, sum it, show and send to @scheme://authority/path/\nIf the code of the result is 200, then sign the structure with key and save it at @structures/bestOne\nIf any from column is less than 2 or flag of structure is yes, then something1\nScript1 X1 of X2 of X3 with Script2 of X4 \nSend vs CustomSend\n@hopem://x/y @hopes://script1 @storage? @user local vs global and default scheme\nUser, account, key, auth, login\n(2 + 2 * (2 + 2))"),e=t[0],l=t[1],m=(0,r.useState)([]),f=m[0],d=m[1],h=(0,r.useRef)();return(0,r.useEffect)((function(){h.current=new Worker(new URL(n.p+n.u(895),n.b)),h.current.addEventListener("message",(function(t){d(t.data.tokens)}))}),[]),(0,r.useEffect)((function(){h.current&&h.current.postMessage({type:"translate",script:e})}),[e]),(0,s.jsxs)(a.Z,{maxWidth:"md",children:[(0,s.jsxs)(i.Z,{component:"form",noValidate:!0,autoComplete:"off",children:[(0,s.jsx)(u.Z,{id:"standard-basic",label:"?",fullWidth:!0,className:"script-title",variant:"standard",value:"MyScript"}),(0,s.jsx)(u.Z,{id:"multiline-static",label:"??",sx:{marginBottom:2,marginTop:4},multiline:!0,rows:5,fullWidth:!0,onChange:function(t){l(t.target.value)},value:e})]}),(0,s.jsx)(c.Z,{sx:{margin:"1em 0 0.5em 0"},children:"Tokens"}),(0,s.jsxs)("div",{className:"tokens",children:[f.map((function(t){var e=t[0],n=t[1],r="default",a="";return"Break"===n?(0,s.jsx)(c.Z,{sx:{margin:"0.1em 0 0.5em 0"}},e):("Being"===n?a="=":"This"===n?(a="result",r="N"):"ListEnd"===n?a="]":"ListStart"===n?a="[":"StructEnd"===n?a="}":"StructStart"===n?a="{":"FormulaEnd"===n?a=")":"FormulaStart"===n?a="(":"string"===typeof n?a=n.toLowerCase():"N"in n?(a=n.N.toLowerCase(),r="N"):"S"in n?(a=n.S.toLowerCase(),r="S"):"M"in n?(a=n.M.toLowerCase(),r="M"):"O"in n?(a=n.O.toLowerCase(),r="O"):"V"in n&&(r="V","Number"in n.V&&(a=n.V.Number.value),"Id"in n.V&&(a=n.V.Id.scheme.Custom),"Text"in n.V&&(a=n.V.Text),"Fact"in n.V&&(a=n.V.Fact.toString())),(0,s.jsx)(o.Z,{size:"small",className:r,label:a},e))})),(0,s.jsx)(c.Z,{sx:{margin:"0.1em 0 0.5em 0"}})]})]})}},5301:function(t,e,n){(window.__NEXT_P=window.__NEXT_P||[]).push(["/",function(){return n(2562)}])}},function(t){t.O(0,[633,774,888,179],(function(){return e=5301,t(t.s=e);var e}));var e=t.O();_N_E=e}]);