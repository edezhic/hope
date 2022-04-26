# Hope
Human-oriented programming environment

This project is about making a 'programming language' as close to natural English as possible. You can find an example of such "code" and Hobot compiled into wasm module [on the github pages](https://edezhic.github.io/Hope/). 

## Script example

```
CustomScript X of Y 
Z is 1, xyz is [x, y, z], s is (x + y + z). Show s, sum and show xyz. If x is less than 2 and y is more than 3 then show {x, y, z, xyz, s}. 
For each number in xyz show (number * 2). For each message at @/endpoint/path/ show @message/content. For each message from @URI return x.
Request 'query' from @http://wikipedia.com and store the result in @db:wiki. Request 'query' from @db:wiki, sign as user and send to @scheme://domain/path.
Select 'query' from @db:x where element is more than 0 and (element * 2) is less than 10.
Match X: 
0 then show 0
1 then show 1
Try sum [0, 'a']. If result contains 'error' show 'error expected here'. Try panic with 'error', show result.
```

## Roadmap

targets: interpreter, basic operations, networking(+p2p?), some cryptography and PWA IDE.
Short-term targets: 
Long-term targets:


## How to run locally
1. install latest stable Node/NPM
2. install latest stable Rust/rustup
3. install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
4. build hobot with `cd hobot && wasm-pack build --out-name index`  
5. build and run hope with `cd .. && npm run dev`
6. visit [localhost:3000](http://localhost:3000)
