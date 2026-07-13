# Pressure Shader Language: Introduction

## Purpose

Pressure is a shader language designed around explicit data flow, predictable compilation, and efficient mapping to modern GPU features.

Pressure is designed for
1. Vertex / Fragment, Mesh, light Compute, and other graphics pipelines
2. Runtime shader compilation with powerful hooks for domain-specific use cases
3. Modular and reusable code that can be easily shared

Pressure does not intend to completely hide the complexity of GPU programming, and instead, provides a consistent model across different supported backends.

## Goals and Non-Goals

Pressure is not intended to be a general purpose programming language, nor is it intended to provide every feature found in other languages.
Features are added to Pressure only when they fulfill a tangible need that cannot otherwise be accomplished in a practical or efficient way.

The language prioritizes
1. Simple, predictable, and expressive grammar and semantics
2. Explicit resource usage and data flow
3. Portability and consistency between all supported backends
4. Clear, descriptive, and meaningful diagnostics
5. Powerful tooling and compiler APIs

The language deliberately does not support
1. Virtual functions and function pointers
2. Cross-compatibility with other shader languages or shader libraries
3. General purpose compute workloads

## Design Principles

Pressure has a set of principles that guide feature design and implementation
1. Explicitness over convenience
    - Features should not introduce implicit or undefined behaviors.
2. Simplicity empowers optimization
    - Simple rules help both the programmer and compiler understand and optimize code.
3. Backend differences should be visible
    - Pressure should provide a consistent programming model across backends, while allowing the programmer to fall back to backend specific features when necessary.
