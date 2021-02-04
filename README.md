# Hope
Human-oriented programming environment

Currently this project is about making an interpretable 'programming language' with intuitive syntax to make it human-readable for anyone, not just software engineers. Working name for this 'language' - Hopes (Hope Specification). An example: 
```
Column as [1.333, 2, 3,5], structure as {column, flag}. Sum column of structure and show the result.
```
This 'code' already works, with more coming. Also, it's parsed following unicode standards and decoupled language-specific vocabulary, so it can be easily extended to other input languages as well. However, interpreter is not the end goal and just the base for the broader project, whose long-term goals are not yet clear and up for discussion.

Current targets:
-[x] basic interpretation: variables, collections, some commands
-[ ] expressions
-[ ] conditions and loops
-[ ] http requests
-[ ] cryptographic primitives
-[ ] browser-based "IDE"
-[ ] p2p networking
-[ ] ...?

Milestones for v1.0:
* pandas-like data mangling
* functionality for distributed teams/companies
* 
* possibility of moving the repo and development process through the Hope itself using some kind of distributed protocol
* court-like functionality
* ...?

Questionable ideas:
* Support for WASM/WebGPU modules, for high-performance custom logic, ML and compatability with modules written in other languages while keeping them sandboxed for security
* Hope OS, based on Redox kernel or smth alike. Hopes as a replacement for shell.
* SNARKs and homomorphic encryption primitives
* ...!
