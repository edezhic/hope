# Hope
Human-oriented programming environment

Currently this project is about making an interpretable 'programming language' with intuitive syntax to make it human-readable for anyone, not just software engineers. An example: 
```
Column as [1.333, 2, 3,5], structure as {column, flag: yes}. Sum column of structure, show the result and send to @scheme:authority/path/.
```

Current targets: interpreter, networking, dummy cryptography and browser-based IDE(simple jupyter-like UI with hope as a wasm module).

Milestones for v1.0:
* pandas-like data mangling, JAX-like basic ML
* functionality for distributed teams/companies
* possibility of moving the repo and development process through the Hope itself using some kind of distributed protocol
* court-like functionality
* ...?

Questionable ideas:
* Support for WASM/WebGPU modules, for high-performance custom logic, ML and compatability with modules written in other languages while keeping them sandboxed for security
* Hope OS, based on or merged with Redox (or smth alike). Hopes as a replacement for shell.
* SNARKs and homomorphic encryption primitives
* And many others
