# Pressure Shader Language

This is the repository for the Pressure Shader Language.

Initially, Pressure will support only Naga as a backend. This is planned to be
expanded in the future.

## Rationale

Pressure is designed around the idea of a "GPU-native" language. GPU concepts
should have direct, first class representations in the language, rather than
trying to adapt the abstractions from specific CPU languages.

Despite being heavily inspired by Rust, Pressure isn't afraid to stray away from
Rust's syntax where it makes sense for the goals of the language.

## Status

Pressure is still a heavy work in progress. The specification, compiler, and
language are still growing, so expect breaking changes and bugs to crop up.

### Post-MVP Roadmap

- [ ] Templates
- [ ] Variadic templates
- [ ] Mesh, tessellation, compute, and ray tracing pipelines
- [ ] Traits
- [ ] Impl blocks
- [ ] Support for nested symbols
- [ ] Range operators
- [ ] SPIR-V, DXIL, and MSL backends separate from Naga
- [ ] Compiler optimizations
- [ ] A standard library
- [ ] Reference types
- [ ] Pattern matching
- [ ] Destructuring
- [ ] Getter / setter functions
- [ ] LSP server
- [ ] Command-line toolchain
- [ ] Vectors and matrices as language-level declarations
- [ ] Additional layout rules (`std140`, `scalar_block`, `wgsl`)

## LLM Usage

Pressure does not use any LLMs, and will not accept any clearly LLM-generated or
LLM-assisted pull requests.
