# Pressure Shader Language: Types

## Scalars

Pressure defines the following scalar types, based off of Rust's type syntax

- `bool`
- `u8`
- `u16`
- `u32`
- `u64`
- `i8`
- `i16`
- `i32`
- `i64`
- `f32`
- `f64`

## Arrays, Vectors, and Matrices

Arrays, vectors, and matrices all use similar syntax, since they're all conceptually a list of values of the same type.

The basic slice syntax is the same as Rust's, `[T; N]` and `[T]`, where `T` is the type, and `N` is the number of elements.

Pressure differentiates vectors and matrices by having a keyword before the same slice syntax,
- `vec[T; N]` for vectors
- `mat[T; N]` for square matrices
- `mat[T; C, R]` for rectangular matrices
    - `C` is columns, `R` is rows
    - Matrices are treated column-major by default, but this can be changed in the compiler settings.

Slice lengths are only available when they are known at compile time. Runtime queries for slice lengths are unsupported, even if the backend supports it.

Swizzling is not a feature inherent to vectors. This will be expanded upon in future documents.

## Resource Types

Pressure defines the following resource types,
- `uniform<T>` for a GPU uniform buffer resource
- `storage<T>` for a GPU storage buffer resource
- `storage<mut T>` for a writable GPU storage buffer resource
- `sampler` for a GPU sampler resource
- `texture_1d<T>`, `texture_2d<T>`, `texture_2d_array<T>`, and `texture_3d<T>` for GPU texture resources
    - `T` is the scalar type of the sampled texels
    - Unless otherwise specified, textures sample as `vec[T; 4]`
    - `mut T` means the texture is also writeable
    - More texture types may be added in the future

## Resource References

Prefixing a resource type with a `*` denotes a resource reference.
Resource references are disabled by default, and must be enabled using a compiler flag.
Support for resource references is backend-dependent, and Pressure does not guarantee support for every backend.
Separate compiler flags exist for `storage` resource references and `texture` or `sampler` resource references.

Resource references are not required to be physical memory addresses, and their representation and semantics are defined by the backend implementation.
Likewise, resource references cannot be compared or used in arithmetic directly.
Backends may provide intrinsics for comparison or arithmetic, but Pressure does not require them.
