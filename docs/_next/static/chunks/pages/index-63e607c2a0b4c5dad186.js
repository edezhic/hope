(self.webpackChunk_N_E=self.webpackChunk_N_E||[]).push([[405],{8191:function(t,e,o){"use strict";o.r(e),o.d(e,{default:function(){return R}});var r=o(5893),s=o(7294),i=o(4065),l=o(2116),a=o(6123),n=o(6265),f={height:"650px"},c="#ddd",m="#bbb",d="#dacc29",u="#42d842",b="#00c184",p="#8f0",h="#41afd2",x="#005772",y={margin:"1em 0 0.5em 0"},g={margin:"0.1em 0 0.5em 0",opacity:0};function w(t,e){var o=Object.keys(t);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(t);e&&(r=r.filter((function(e){return Object.getOwnPropertyDescriptor(t,e).enumerable}))),o.push.apply(o,r)}return o}function z(t){for(var e=1;e<arguments.length;e++){var o=null!=arguments[e]?arguments[e]:{};e%2?w(Object(o),!0).forEach((function(e){(0,n.Z)(t,e,o[e])})):Object.getOwnPropertyDescriptors?Object.defineProperties(t,Object.getOwnPropertyDescriptors(o)):w(Object(o)).forEach((function(e){Object.defineProperty(t,e,Object.getOwnPropertyDescriptor(o,e))}))}return t}var _={mass:4,font:{color:m}},j={mass:4,font:{color:u}},v={mass:4,font:{color:b}},O={mass:4,font:{color:p}},S={mass:4,font:{color:d}},k={font:{color:h}},C={font:{color:c}},E={color:m,dashes:!0,arrows:{to:{enabled:!1}}},Z={color:h},T={color:x},P={color:c,dashes:!0},L={color:m},N={clickToUse:!0,nodes:{borderWidth:0,color:"#121212",mass:.3,font:{size:22}},edges:{color:"#888",font:{size:18,color:c,strokeWidth:0},smooth:!0,width:1,arrows:{to:{enabled:!0,scaleFactor:.6}}}},W={graph:{nodes:[z({id:"graphscript",label:"graphscript"},{x:0,y:0,fixed:{x:!0,y:!0},mass:10,font:{color:c,size:30}}),z({id:"assign_z",label:"be"},_),z({id:"list",label:"list"},v),z({id:"assign_xyz",label:"be"},_),z({id:"formula",label:"+ +"},O),z({id:"assign_s",label:"be"},_),z({id:"show1",label:"show"},j),z({id:"sum",label:"sum"},j),z({id:"show2",label:"show"},j),z({id:"if",label:"if"},S),z({id:"less",label:"less"},S),z({id:"more",label:"more"},S),z({id:"show3",label:"show"},j),z({id:"struct",label:"struct"},v),z({id:"when",label:"when"},S),z({id:"x1",label:"x"},k),z({id:"y1",label:"y"},k),z({id:"z1",label:"z"},k),z({id:"x2",label:"x"},k),z({id:"y2",label:"y"},k),z({id:"z2",label:"z"},k),z({id:"x3",label:"x"},k),z({id:"y3",label:"y"},k),z({id:"z3",label:"z"},k),z({id:"x4",label:"x"},k),z({id:"y4",label:"y"},k),z({id:"x5",label:"x"},k),z({id:"y5",label:"y"},k),z({id:"z5",label:"z"},k),z({id:"s1",label:"s"},k),z({id:"s2",label:"s"},k),z({id:"s3",label:"s"},k),z({id:"xyz1",label:"xyz"},k),z({id:"xyz2",label:"xyz"},k),z({id:"xyz3",label:"xyz"},k),z({id:"it1",label:"it"},k),z({id:"it2",label:"it"},k),z({id:"it3",label:"it"},k),z({id:"it4",label:"it"},k),z({id:"it5",label:"it"},k),z({id:"it6",label:"it"},k),z({id:"it7",label:"it"},k),z({id:"it8",label:"it"},k),z({id:1,label:"1"},C),z({id:2,label:"2"},C),z({id:3,label:"3"},C)],edges:[z({from:"graphscript",to:"x1"},E),z({from:"graphscript",to:"y1",label:"of"},E),z({from:"graphscript",to:"assign_z"},P),z({from:1,to:"assign_z"},T),z({from:"assign_z",to:"z1"},Z),z({from:"assign_z",to:"list"},P),z({from:"x2",to:"list"},T),z({from:"y2",to:"list"},T),z({from:"z2",to:"list"},T),z({from:"list",to:"it7"},Z),z({from:"list",to:"assign_xyz"},P),z({from:"it8",to:"assign_xyz"},T),z({from:"assign_xyz",to:"xyz1"},Z),z({from:"assign_xyz",to:"formula"},P),z({from:"x3",to:"formula"},T),z({from:"y3",to:"formula"},T),z({from:"z3",to:"formula"},T),z({from:"formula",to:"it1"},Z),z({from:"it2",to:"assign_s"},T),z({from:"formula",to:"assign_s"},P),z({from:"assign_s",to:"s1"},Z),z({from:"assign_s",to:"show1"},L),z({from:"s2",to:"show1"},T),z({from:"show1",to:"sum"},P),z({from:"xyz2",to:"sum"},T),z({from:"sum",to:"show2"},P),z({from:"sum",to:"it3"},Z),z({from:"it4",to:"show2"},T),z({from:"show2",to:"if"},L),z({from:"if",to:"less"},P),z({from:"x4",to:"less"},T),z({from:2,to:"less"},T),z({from:"less",to:"more"},P),z({from:"y4",to:"more"},T),z({from:3,to:"more"},T),z({from:"more",to:"struct"},P),z({from:"show",to:"if"},P),z({from:"if",to:"when"},L),z({from:"struct",to:"show3"},P),z({from:"struct",to:"it5"},Z),z({from:"it6",to:"show3"},T),z({from:"x5",to:"struct"},T),z({from:"y5",to:"struct"},T),z({from:"z5",to:"struct"},T),z({from:"s3",to:"struct"},T),z({from:"xyz3",to:"struct"},T),z({from:"show3",to:"if"},L)]}},D=o(9227),F=o(3733);function B(t){var e=(0,D.Z)(t.item,2),o=e[0],s=e[1],i=d,a="";if("Break"===s)return(0,r.jsx)(l.Z,{sx:g},o);if("Being"===s)a="be",i=m;else if("This"===s)a="result",i=h;else if("ListEnd"===s)a="]",i=b;else if("ListStart"===s)a="[",i=b;else if("StructEnd"===s)a="}",i=b;else if("StructStart"===s)a="{",i=b;else if("FormulaEnd"===s)a=")",i=p;else if("FormulaStart"===s)a="(",i=p;else if("string"===typeof s)a=s.toLowerCase();else if("Term"in s)a=s.Term.toLowerCase(),i=h;else if("Cmd"in s)a=s.Cmd.toLowerCase(),i=u;else if("Mod"in s)a=s.Mod.toLowerCase(),i=x;else if("Op"in s)i=p,a=s.Op.toLowerCase().replace("plus","+").replace("multiplication","*");else if("Value"in s){var n=s.Value;i="#bbb","Number"in n&&(a=n.Number.value),"Id"in n&&(a=n.Id.scheme.Custom),"Text"in n&&(a=n.Text),"Fact"in n&&(a=n.Fact.toString())}return 0==t.i&&(i=c),(0,r.jsx)(F.Z,{size:"small",label:a,sx:{background:"none",color:i}},o)}var M=o(2094),V=o(8315);function I(t){return(0,r.jsxs)(M.Z,{component:"form",noValidate:!0,autoComplete:"off",onSubmit:function(t){return t.preventDefault()},children:[(0,r.jsx)(V.Z,{id:"standard-basic",label:"Title",fullWidth:!0,className:"script-title",variant:"standard",onChange:function(e){return t.setScript([e.target.value,t.script[1]])},value:t.script[0]}),(0,r.jsx)(V.Z,{id:"multiline-static",label:"Body",sx:{marginBottom:2,marginTop:4},multiline:!0,rows:3,fullWidth:!0,onChange:function(e){return t.setScript([t.script[0],e.target.value])},value:t.script[1]})]})}function R(){var t=(0,s.useState)(W),e=t[0],n=(t[1],(0,s.useState)(["",""])),c=n[0],m=n[1],d=(0,s.useState)([]),u=d[0],b=d[1],p=(0,s.useState)(0),h=p[0],x=(p[1],(0,s.useRef)());return(0,s.useEffect)((function(){x.current=new Worker(new URL(o.p+o.u(627),o.b)),x.current.addEventListener("message",(function(t){"tests"==t.data.type&&m(t.data.tests[h]),"tokens"==t.data.type&&b(t.data.tokens)})),x.current.postMessage({type:"get_tests"})}),[]),(0,s.useEffect)((function(){x.current&&x.current.postMessage({type:"build",title:c[0],body:c[1]})}),[c]),(0,r.jsxs)(i.Z,{maxWidth:"md",children:[(0,r.jsx)(l.Z,{sx:y,children:"Script"}),(0,r.jsx)(I,{script:c,setScript:m}),(0,r.jsx)(l.Z,{sx:y,children:"Tokens"}),null===u||void 0===u?void 0:u.map((function(t,e){return(0,r.jsx)(B,{item:t,i:e},JSON.stringify(t))})),(0,r.jsx)(l.Z,{sx:y,children:"Graph"}),(0,r.jsx)(a.Z,{graph:e.graph,options:N,style:f})]})}},5301:function(t,e,o){(window.__NEXT_P=window.__NEXT_P||[]).push(["/",function(){return o(8191)}])}},function(t){t.O(0,[814,959,737,774,888,179],(function(){return e=5301,t(t.s=e);var e}));var e=t.O();_N_E=e}]);