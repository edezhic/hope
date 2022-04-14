# Hope
Human-oriented programming environment

This project is about making a 'programming language' with intuitive syntax to make it human-readable for anyone, not just software engineers. You can find an example of such "code" and Hobot compiled into wasm module [on the github pages](https://edezhic.github.io/Hope/). Current targets: interpreter, basic operations, networking(+p2p?), some cryptography and PWA IDE.

# How to start
1. install latest stable Node/NPM
2. install latest stable Rust/rustup
3. install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
4. build hobot with `wasm-pack build --out-name index`  
5. build and run hope with `npm run dev`
6. visit [localhost:3000](http://localhost:3000)
