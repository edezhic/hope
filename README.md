# Hope
Human-oriented programming environment

Currently this project is about making an interpretable 'programming language' with intuitive syntax to make it human-readable for anyone, not just software engineers. Working name for this 'language' - Hopes (Hope Specification). An example: 
```
Column as [1.333, 2, 3,5], structure as {column, flag: yes}. Sum column of structure, show the result and send to @scheme:authority/path/.
```
Parsing is done following unicode standards and using decoupled language-specific vocabulary, so it can be easily extended to other input languages as well. However, interpreter is not the end goal and just the base for the broader project, whose long-term goals are not yet clear and up for discussion.

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
