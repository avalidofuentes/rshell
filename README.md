# rshell

A Unix-like shell written in Rust.

## Motivation

I started this project to better understand how Unix-like shells work internally and to explore systems programming using Rust.
Instead of relying on tutorials or isolated examples, the goal is to learn by building a real, non-trivial piece of software
that interacts directly with the operating system.

This project is intentionally developed incrementally. Each feature is added step by step, focusing on correctness,
clarity, and good engineering practices rather than rushing towards a “complete” shell.

## Goals

- Build a minimal but functional Unix-like shell in Rust
- Understand process execution, command parsing, and OS interaction
- Apply Rust concepts in a realistic systems programming context
- Keep the codebase clean, readable, and easy to extend
- Grow the project progressively towards more advanced shell features

## Roadmap

- [ ] Basic REPL (read–eval–print loop)
- [ ] Execute external commands
- [ ] Built-in commands (`exit`, `cd`, `pwd`)
- [ ] Improved command parsing
- [ ] Pipes (`|`)
- [ ] Input/output redirections (`>`, `<`)
- [ ] Environment variables
- [ ] Command history
- [ ] Text-based user interface (TUI)
- [ ] Experimental terminal features

## Project Status

Early development.  
Features are added incrementally as the project evolves.

## License

This project is licensed under the MIT License.
