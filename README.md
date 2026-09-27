# Pressure Shader Language

This is the repository for the Pressure Shader Language.

Initially, Pressure will support only Naga as a backend. This may be expanded in
the future.

## Rationale

Pressure is designed around the idea of a "GPU-native" language. GPU concepts
should have direct, first class representations in the language, rather than
trying to adapt the abstractions from specific CPU languages.

Despite being heavily inspired by Rust, Pressure isn't afraid to stray away from
Rust's syntax where it makes sense for the goals of the language.

## Status

Pressure is still a heavy work in progress. The specification, compiler, and
language are still growing, so expect breaking changes and bugs to crop up.

## LLM Usage

Pressure does not use any LLMs, and will not accept any clearly LLM-generated or
LLM-assisted pull requests.
