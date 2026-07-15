# Pressure Shader Language: Statements

## Blocks

A block is a statement consisting of zero or more statements enclosed in braces. (`{}`)

## If/Else

An if statement consists of an expression that resolves to a boolean and a block, preceded by the `if` keyword.

An if statement can also be followed by zero or more "else if" branches, taking the same form as the `if` statement, but with `else if` instead of just `if`.

An if statement or else if branch can be followed by an optional terminating `else` branch, that has no condition, only followed by a block.

Example,
```psi
if x == y {
    //...
} else if x == z {
    //...
} else {
    //...
}
```

## Loops

Pressure has two ways of defining loops, `while` and `for`.

While loops take a similar form to if statements, where they have an expression and a block, but instead of the `if` keyword they use the `while` keyword.
While loops keep running the inner block until the condition expression resolves to `false`.

Example,
```psi
while i < 100 {
    //...
}
```

For loops consist of a variable identifier, an expression that resolves to an iterator, and a block.
The statement starts with the `for` keyword, then the variable identifier, then the `in` keyword, then the expression, and finally the block.
The meaning of "iterator" will be defined in a later document.
The variable that is declared in the for loop is only valid within the scope of the following block.
The variable cannot be made mutable.

Example,
```psi
for i in [0, 1, 2, 3, 4, 5] {
    //...
}
```

## Declarations

A variable declaration can be used as a statement.
When declaring a variable, it is only accessible for the scope of the current block.
See [04-variables.md](./04-variables.md)

## Assignment

An assignment statement consists of an identifier followed by zero or more access arms, an equals sign, and an expression, followed by a semicolon.

An access arm can be either an array index (`[index]`), or a field access (`.field`). 

The assignment statement replaces the value at that path with the value produced by the expression.

Example,
```psi
x.y = 1;
vector.xy = vec[1, 2];
```

An assignment with an `_` on the left hand side is considered an expression discard statement, which evaluates the expression but discards the result.

Invoking functions just for the side effects require the use of an expression discard statement.
In shader code, running a function purely for the side effects is an uncommon need, and having a specialized statement would introduce more parsing complexity.

## Other Control Flow

Pressure also has `return`, `break`, and `discard`.

A `break` statement breaks out of the current loop, continuing execution at the after the loop block.

A `discard` statement immediately halts execution of the current shader invocation, and stops prevents the fragment from producing an output. This is only allowed in fragment shaders.

A `return` statement yields execution back to the caller of the function, and can have an optional expression if the function yields a non-void type.

Examples,
```
while true {
    if condition {
        break;
    }
}

if alpha < 0.05 {
    discard;
}

return vec[1, 1, 1, 1];
```
