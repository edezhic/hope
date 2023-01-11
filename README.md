# Archived
Project stopped because I realized the complexity that morphology brings into all of that. I've tried to process only the bare minimum (posessives and plurals) but got into lots of language-specific issues. While I still think that something like that is possible to create, it will require lots of code for each language and there won't be much reusability/translatability. It occurs to me that evolving current PLs (mostly Rust and Python) might be the way to go, also recent progress with GPT shows that NNs are pretty good at translating natural language to programming ones. That's probably the optimal way since project could be described in a natural language and "compiled" into something machines could execute. This approach still requires thoughtful manual validation of the result but can already speed up development significantly. So, I don't think there is much sense in continuing this project. If anyone will be pursuing similar ideas, I'd suggest exploring NN-powered tokenization to avoid dealing with morphology and syntax, and start from building on top of abstract trees instead of raw text. 

# Hope
Human-oriented programming environment

This project is about making a 'programming language' that is as close to natural language (currently targeting English) as possible. You can find an example of such "code" and HOBot (human-oriented bot) compiled into wasm module that builds the script on the fly [on the github pages of this repo](https://edezhic.github.io/hope/). Also there you can find the actual example of such a script.

## Motivation

The initial idea came from realization that in order to build truly decentralized systems they have to be understandable for every participant. For example, many financial and public services can become much more efficient if we'll replace the burocracy with digital alternatives, but to function properly the broad public must be able to understand, inspect and propose changes to the underlying mechanisms. And it's just not the case with modern software. You might argue that everyone has to learn programming, but to understand the modern digital world you'll have to learn many languages and frameworks, which is not an easy task even for experienced developers. 

I believe that the best way to solve this is to create a programming language(PL) that is as close to what everyone is used to already. I don't think that tweaking any of the existing PLs makes sense because they were initially designed for engineers, and this way will require too many breaking changes for a smooth transition. Also, I think that in some sense we need to reverse the whole process: develop technical possibilities around language instead of developing a language around technical requirements. 

Besides that, the whole compilation process needs to be redesigned. Inspectable PL can't be done with a dozen of unintuitive intermediate representations, so for now I want to keep only tokens and computational graph. Execution flow has to be simplified. Things like mandatory semicolons/indentation and base2 float arithmetic rounding errors have to leave the stage already. And there is plenty of other details that are intuitive to software engineers but very confusing to regular folks. Also, I think it should be comfortable to read and write on small devices like smartphones and tablets. 

The end result should look like an instruction for execution of a command in regular formal english with a few(!) special symbols for convenience. And anyone with the knowledge of natural language should be able to grasp the core ideas in less than an hour.

## Roadmap

Short-term targets: 
- tokenization(mostly done) 
- linking tokens into computational graph(wip)
- execution engine for basic arithmetic, networking, OS interactions(through WASI) and UI development(outputing html/css/js for now).  

Long-term questionable ideas:
- Hope itself - intuitive IDE with all the debugging tools etc
- integration with RedoxOS (lots of common underlying ideas)
- WebGPU-based GPGPU for machine learning and/or graphics
- advanced cryptography
- [insert what **you** want in a perfect programming language here]

## Known issues

- **Ambitions**. Many attempts to make computers understand commands in natural languages have been made over the last 70 years and most didn't meet even modest expectations. Symbolic/expert systems, virtual assistants, you name it. Recent advances in deep learning based code generation have produced some interesting results, but I don't think that this approach is viable because of unpredictability of results. However, I think NNs might play an important role in some parts of this system in the future. 
- **Performance**. For now it's not a priority in any way and even in the long run this language is not supposed to compete with PLs like Rust. However, I think that building this language in Rust and keeping it as simple as possible leaves a lot of room for future optimizations. 
- **Languages beside English**. I can't think of a decent solution right now except for forking and rewriting lots of stuff. Maybe NNs would be able to help with that in the future, maybe English will become global enough to alleviate this problem, I don't know yet.
- [insert **your** complaint here]

## How to run it locally
1. install latest stable Node/NPM
2. install latest nightly Rust/rustup
3. install latest wasm-pack. For Linux/MacOS: `npm run install:wasm-pack`, to find installation instructions for other platforms visit [rust-wasm.github.io/wasm-pack/installer](https://rustwasm.github.io/wasm-pack/installer/#)
4. `npm run dev:hope` and visit [localhost:3000](http://localhost:3000) (sometimes wasm-pack crate compilation fails, but usually works fine when you run it again)