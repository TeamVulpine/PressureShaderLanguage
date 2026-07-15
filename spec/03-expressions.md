# Pressure Shader Language: Expressions

## Number Literals

Pressure supports numbers in decimal, hexadecimal, binary, and octal formats.

Underscores can be placed between digits as visual separators, and do not affect the value of the literal.

### Decimal

Decimal numbers consist of one or more decimal digits
```psi
0
10
1_000_000
```

Decimal literals can also include a fractional part, denoted with a `.`
```psi
0.5
123.45
0.00000001
```

Decimal literals can also include an exponent, using `e` or `E`, followed by an optional sign, and a decimal integer
```psi
6e7
1.5E-3
```

### Non-Decimal integers

Binary, octal, and hexadecimal literals are always integers and cannot include a fractional part or exponent.

Hexadecimal numbers are prefixed with `0x` or `0X`.
```psi
0x01_23_45_67_89_AB_CD_EF
```

Octal numbers are prefixed with `0o` or `0O`
```psi
0o01_234_567
```

Binary numbers are prefixed with `0b` or `0B`
```psi
0b01000101
```

### Suffixes

Number literals may also specify their type using a suffix of one of the builtin number types.
```psi
1u8

2.5f32

64i64
```

If a number litaral is not explicitly suffixed, the type is automatically determined based on usage.
When no unique type can be inferred, integer literals default to `i32` and floating point literals default to `f32`.

## Boolean Literals

Pressure has `true` and `false` for boolean literals.

## Array literals

Pressure uses `[]` to denote an array literal, and they contain a list of zero or more expressions, separated by a comma.
```psi
[1, 2, 3, 4, 5]
```

Vectors and matrices use similar syntax, except they are prefixed with `vec` and `mat` respectively
```psi
mat[
    vec[1, 0, 0],
    vec[0, 1, 0],
    vec[0, 0, 1]
]
```

## Identifiers

Identifiers refer to named symbols, like a name for a value.

Identifiers must begin with one of the following characters
- `a` through `z`
- `A` through `Z`
- `_` and `$`

Identifiers can be continued with the same characters, as well as any decimal digit `0` through `9`.

In regular expression form, `[a-zA-Z_$][a-zA-Z0-9_$]*`

```psi
my_value
AnotherValue
_numberOne_Rated_sales_MAN
$1997
```

Identifiers cannot be any of the reserved keywords.

## Access

The access operator uses the syntax `value.access`
```psi
a.b.c
SomeStruct.SomeField.value
```

## Indexing

The indexing operator uses the syntax `value[index]`.
```psi
array[0]
```

## Invocation

The invocation operator uses the syntax `value(arguments)`, where `arguments` is a list of zero or more expressions, separated by a comma.
```psi
function(1, 2, true, value)
```

## Prefix Unary Operators

Prefix unary operators operate on the expression immediately following them.
Multiple prefix unary operators associate from right to left.

Pressure defines the following prefix unary operators,
- `-` for arithmetic negation
- `!` for boolean negation
- `~` for bitwise negation

## Binary Operations

Binary operations are used to combine two expressions.

Pressure defines the following operators,

### Arithmetic
- `+` for addition
- `-` for subtraction
- `*` for multiplication
- `/` for division
- `%` for remainder

### Bitwise
- `&` for AND
- `|` for OR
- `^` for XOR

### Boolean
- `&&` for AND
- `||` for OR
- `^^` for XOR

## Casts

Pressure uses the `as` keyword for casts.

Casts follow the `value as Type` syntax.

```psi
someValue as i32
vec[1, 2, 3] as vec[f64; 4]
```

Casts higher precedence than binary operations but lower precedence than unary operations.
```psi
1 + 2 as i32
// same as
1 + (2 as i32)

-4 as i32
// same as
(-4) as i32
```

## Struct Construction

Struct construction has the syntax `new Path { initializers }`.

The `Path` is a list of one or more identifiers, separated by a dot.
The `initializers` are a list of field initializers, separated by a comma. Trailing commas are allowed.

Field initializers take two forms,
1. `name: value`, where name is an identifier, and value is an expression.
    - This sets the field in that struct to have that value
2. `name` where name is an identifier.
    - This sets the field in that struct to the value of a variable with that same name

Example,
```psi
new MyStruct {
    x: 32,
    y,
    z: vec[64, 128, 256],
}
```

## Parenthesized Expressions

Expressions can be grouped using `()`.

Parentheses override the normal operator precedence.
```psi
(1 + 2) * 3
```
