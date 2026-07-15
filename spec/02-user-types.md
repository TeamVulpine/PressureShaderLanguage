# Pressure Shader Language: User Defined Types

Pressure provides three ways to define types, structs, enums, and type aliases.

## Type Aliases

Pressure uses the `type` keyword to declare a type alias.
A type alias directly refers to that type, and is not a new type itself.

Example:
```psi
type f32vec2 = vec[2]f32;

type SomeBufferType = storage<[SomeBufferValue]>;
```

## Enums

Pressure uses the `enum` keyword to declare an enum.
Enums are integer backed enumerations, similar to those found in C.

They contain a list of variants, with an optional explicit value, and it is automatically incremented from the previous variant's value if not specified explicitly.
Unlike C, enums are not put into the global scope, instead you need to use the name of the enum to access the values.
Enums can also specify a bit width, using any unsigned integer type.

Example:
```psi
enum MyEnum : u8 {
    MyValueA,
    MyValueB = 2,
    MyValueC, // = 3
}
```

## Structs

Pressure uses the `struct` keyword to declare a struct.
Structs are a collection of named fields, that are stored linearly.

Example:
```psi
struct MyStruct {
    myValue: f32vec2,
    myOtherValue: MyEnum,
}
```
