# Types

## Primitive types

### Integral types

Tuploid has 8 integral types, 4 unsigned and 4 signed. Unsigned types have the
following shape `u@N@`, and signed types have the following shape `i@N@`. These
types can be considered to be N-tuples of bits, with the signed types reserving
a bit for the sign. `N` may be one of 8, 16, 32, 64.

Default initialization for an integral type will set its value to `0`.

### The `string` type

The `string` type is a tuple of UTF-8 codepoints. A `string` is always a valid UTF-8 encoded N-tuple of codepoints.

`string`s are interned by the Tuploid runtime and are immutable.

Default initialization for a `string` will set its value to `""`.

## Tuples

Tuples are the most important feature of Tuploid(it's in the name!)

Tuples can contain 0 or more properties, that may be given a name or not. Named
properties may be accessed using dot access. All properties may be accessed
using index access, however nameless properties must be accessed in that way.

Example:

```
let tuple : (i8, second: i16, i32) = (3, 4, 5);
tuple.0 = 5; // Dot access may be used as index access using constants
tuple.second = 8; // One may access named properties using dot access
let idx : u64 = 2;
tuple[idx] = 12; // Commonly called "array" indexing might also be used, to facilitate access using variables
tuple.1 = 3; // Named properties may also be accessed using indices.
```

You may assign a variable of a tuple type with less properties to a variable
of a tuple type with more properties, the remaining properties are default
initialised.

Default initialization for a tuple type is recursive, until a primitive type is
found, at which point, that type will be default initialised.

Tuploid featurs two kinds of tuples: *static tuples* and *dynamic tuples*.

### Static tuples

Static tuples are tuples to which you cannot add more properties. Static
tuples types and literals use `(` and `)`.

A variable of a dynamic tuple type may be assigned to a variable of a static
tuple type if and only if the dynamic tuple's properties form a subset of the
properties of the static tuple. If this condition is not satisfied, a runtime
error will occur.

Trying to access a property that does not exist will result in a compile-time
error.

### Dynamic tuples

Dynamic tuples are tuples to which you can add more properties dynamically.
Dynamic tuples types and literals use `[` and `]`.

A variable of static tuple type may be assigned to a variable of a dynamic tuple
type, with the following effects:

* properties that exist in the static tuple but not in the dynamic tuple will
be added to the dynamic tuple
* properties that exist in the dynamic tuple but not in the static tuple will
remain untouched
* properties that exist in both will have their values assigned from the static
tuple.

See previous notes on assigning a dynamic tuple to a static one.

Trying to access a property that does not exist will result in a runtime error.

## Type aliases

A type alias is introduced with the `type` keyword using the following syntax:

```
type alias = old_name;
```

### The special type alias `self`

The type alias `self` is an implicitly declared type alias by the compiler.

`self` is declared by the compiler during the definition of a tuple type allowing
for recursive tuples, or tuples containing a function which takes a variable of
said tuple type as parameter.
