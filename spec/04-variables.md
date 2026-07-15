# Pressure Shader Language: Variables

Variables are declared two ways, using `let` and `const`.

Variables declared with `let` are considered local to the function they're declared in, and may depend on runtime state.

Variables declared with `const` are compile-time constant, and can only be computed from other constant data.
Constants also are the only variable type that can be declared in the global scope.

Local variables can also be declared with `let mut`, to signify that they can be mutated.

Variables are allowed to shadow older variables with the same name, and if a variable is shadowed it can no longer be accessed from the scope the shadowing variable is defined.

Variable declarations take the following forms,
```
const name: Type = initializer;
let name: Type = initializer;
let name = initializer;
let mut name: Type = initializer;
let mut name = initializer;
let mut name: Type;
let mut name;
```
Where `name` is the name of the variable, `Type` is the type of the variable, and `initializer` is an expression to initialize it.

Constant variables always require the type and initializer.
Local immutable variables always require an initializer, but the type can be inferred by usage.
Local mutable variables can omit both the type and initializer.

Examples,
```psi
const Pi: f64 = 3.1415926535897932384626;
let tau = Pi * 2;
let mut someValue;
let mut count: i32;
```

A variable without an initializer is considered uninitialized.
Before a variable can be used, all possible control flow paths must assign to it.
