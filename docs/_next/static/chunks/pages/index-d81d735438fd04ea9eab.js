(self.webpackChunk_N_E=self.webpackChunk_N_E||[]).push([[405],{2562:function(e,t,o){"use strict";o.r(t),o.d(t,{default:function(){return m}});var r=o(5893),n=o(7294),a=o(4065),i=o(2094),s=o(8315),l=o(3733),c=o(2116),u=o(6123),d={clickToUse:!0,layout:{hierarchical:{enabled:!0,direction:"LR"}},nodes:{borderWidth:5,font:{color:"#ddd"}},edges:{color:"#FFFFFF",physics:!1}};function m(){var e=(0,n.useState)({counter:5,graph:{nodes:[{id:1,label:"termscript",color:"#50003b"},{id:2,label:"x",color:"#001b3e"},{id:3,label:"1",color:"#444"},{id:4,label:"if",color:"#403b00"},{id:5,label:"x",color:"#001b3e"},{id:6,label:"is",color:"#403b00"},{id:7,label:"less",color:"#403b00"},{id:8,label:"2",color:"#444"},{id:9,label:"show",color:"#444"},{id:10,label:"Ok",color:"#444"}],edges:[{from:1,to:2},{from:2,to:4},{from:3,to:2,color:"#dacc29"},{from:4,to:5,color:"#dacc29"},{from:5,to:6,color:"#dacc29"},{from:6,to:7,color:"#dacc29"},{from:7,to:8,color:"#dacc29"},{from:8,to:9},{from:10,to:9,color:"#dacc29"}]}}),t=e[0],l=(e[1],t.graph),m=(0,n.useState)(["",""]),b=m[0],p=m[1],h=(0,n.useState)([]),g=h[0],x=h[1],v=(0,n.useState)(4),w=v[0],N=(v[1],(0,n.useRef)());return(0,n.useEffect)((function(){N.current=new Worker(new URL(o.p+o.u(895),o.b)),N.current.addEventListener("message",(function(e){switch(e.data.type){case"tokens":return void x(e.data.tokens);case"tests":return void p(e.data.tests[w])}})),N.current.postMessage({type:"get_tests"})}),[]),(0,n.useEffect)((function(){N.current&&N.current.postMessage({type:"build",title:b[0],body:b[1]})}),[b]),(0,r.jsxs)(a.Z,{maxWidth:"md",children:[(0,r.jsxs)(i.Z,{component:"form",noValidate:!0,autoComplete:"off",onSubmit:function(e){return e.preventDefault()},children:[(0,r.jsx)(s.Z,{id:"standard-basic",label:"Script title",fullWidth:!0,className:"script-title",variant:"standard",onChange:function(e){return p([e.target.value,b[1]])},value:b[0]}),(0,r.jsx)(s.Z,{id:"multiline-static",label:"Script body",sx:{marginBottom:2,marginTop:4},multiline:!0,rows:5,fullWidth:!0,onChange:function(e){return p([b[0],e.target.value])},value:b[1]})]}),(0,r.jsx)(c.Z,{sx:{margin:"1em 0 0.5em 0"},children:"Graph"}),(0,r.jsx)(u.Z,{graph:l,options:d,style:{height:"300px"}}),(0,r.jsx)(c.Z,{sx:{margin:"1em 0 0.5em 0"},children:"Tokens"}),(0,r.jsxs)("div",{className:"tokens",children:[null===g||void 0===g?void 0:g.map((function(e){return(0,r.jsx)(f,{item:e},JSON.stringify(e))})),(0,r.jsx)(c.Z,{sx:{margin:"0.1em 0 0.5em 0"}})]})]})}function f(e){var t=e.item[0],o=e.item[1],n="default",a="";return"Break"===o?(0,r.jsx)(c.Z,{sx:{margin:"0.1em 0 0.5em 0"}},t):("Being"===o?a="=":"This"===o?(a="result",n="N"):"ListEnd"===o?a="]":"ListStart"===o?a="[":"StructEnd"===o?a="}":"StructStart"===o?a="{":"FormulaEnd"===o?a=")":"FormulaStart"===o?a="(":"string"===typeof o?a=o.toLowerCase():"N"in o?(a=o.N.toLowerCase(),n="N"):"C"in o?(a=o.C.toLowerCase(),n="C"):"M"in o?(a=o.M.toLowerCase(),n="M"):"O"in o?(n="O",a=o.O.toLowerCase().replace("plus","+").replace("multiplication","*")):"V"in o&&(n="V","Number"in o.V&&(a=o.V.Number.value),"Id"in o.V&&(a=o.V.Id.scheme.Custom),"Text"in o.V&&(a=o.V.Text),"Fact"in o.V&&(a=o.V.Fact.toString())),(0,r.jsx)(l.Z,{size:"small",className:n,label:a},t))}},5301:function(e,t,o){(window.__NEXT_P=window.__NEXT_P||[]).push(["/",function(){return o(2562)}])}},function(e){e.O(0,[814,959,340,774,888,179],(function(){return t=5301,e(e.s=t);var t}));var t=e.O();_N_E=t}]);