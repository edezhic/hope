# Hope
Human-oriented programming environment

This project is about making a 'programming language' that is as close to natural language (currently targeting English) as possible. You can find an example of such "code" and HOBot (human-oriented bot) compiled into wasm module that builds the script on the fly [on the github pages of this repo](https://edezhic.github.io/Hope/). 

## Motivation

The initial idea came from realization that in order to build truly decentralized systems they have to be understandable for every participant. For example, many financial and public services can become much more efficient if we'll replace the burocracy with digital alternatives, but to function properly the broad public must be able to understand, inspect and propose changes to the underlying mechanisms. And it's just not the case with modern software. You might argue that everyone has to learn programming, but to understand the modern digital world you'll have to learn many languages and frameworks, which is not an easy task even for experienced developers. 

I believe that the best way to solve this is to create a programming language(PL) that is as close to what everyone is used to already. I don't think that tweaking any of the existing PLs makes sense because they were initially designed for engineers, and this way will require too many breaking changes for a smooth transition. Also, I think that in some sense we need to reverse the whole process: develop technical possibilities around language instead of developing a language around technical requirements. 

Besides that, the whole compilation process needs to be redesigned. Inspectable PL can't be done with a dozen of unintuitive intermediate representations, so for now I want to keep only tokens and computational graph. Execution flow has to be simplified. Things like mandatory semicolons/indentation and base2 float arithmetic rounding errors have to leave the stage already. And there is plenty of other details that are intuitive to software engineers but very confusing to regular folks. Also, I think it should be comfortable to read and write on small devices like smartphones and tablets. 

The end result should look like an instruction for performing some command in regular formal english with a few(!) special symbols for convenience. And anyone with the knowledge of natural language should be able to grasp the core ideas in less than an hour.

## An example of script in hope
Which can be parsed into tokens already and some parts of it even linked into the graph. It was carefully crafted to capture syntax that could be linked and executed relatively easily in an intuitive manner without ambiguities.

Title/definition/header:
```
CustomScript X and Y 
```
Body:
```
Z is 1, xyz is [x, y, z], s is (x + y + z). Show s, sum and show xyz. If x is less than 2 and y is more than 3 then show {x, y, z, xyz, s}. 
For each number in xyz show (number * 2). For each message at @/endpoint/path/ show @message/content. For each message from @URI return x.
Request 'query' from @http://wikipedia.com, sign as user and store the result in @db:wiki. Select 'query' from @db:x where element is more than 0 and (element * 2) is less than 10.
Match X: 
0 then show 0
1 then show 1
Try sum [0, 'a']. If result contains 'error' show 'error expected here'. Try panic with 'error', show result.
```

## Roadmap

Short-term targets: 
- tokenization(mostly done) 
- linking tokens into computational graph(wip)
- execution engine for basic arithmetic, networking, OS interactions(through WASI) and UI development(outputing html/css/js for now).  

Long-term questionable ideas:
- integration with RedoxOS (lots of common underlying ideas)
- WebGPU-based GPGPU and/or graphics
- advanced cryptography
- [insert what **you** want in a perfect programming language here]

## Known issues

- **Ambitions**. Many attempts to make computers understand commands in natural languages have been made over the last 70 years and most didn't meet even modest expectations. Symbolic/expert systems, virtual assistants, you name it. Recent advances in deep learning based code generation have produced some interesting results, but I don't think that this approach is viable because of unpredictability of results. However, I think NNs might play an important role in some parts of this system in the future. 
- **Performance**. For now it's not a priority in any way and even in the long run this language is not supposed to compete with PLs like Rust. However, I think that building this language in Rust and keeping it as simple as possible leaves a lot of room for future optimizations. 
- **Languages beside English**. I can't think of a decent solution right now except for forking and rewriting lots of stuff. Maybe NNs would be able to help with that in the future, maybe English will become global enough to alleviate this problem, I don't know yet.
- [insert **your** complaint here]

## How to run source code locally
1. install latest stable Node/NPM
2. install latest stable Rust/rustup
3. install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
4. build hobot with `cd hobot && wasm-pack build --out-name index`  
5. build and run hope with `cd .. && npm run dev`
6. visit [localhost:3000](http://localhost:3000)
