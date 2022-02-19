# Types

## Scalar Types

### Integer
  - 8 -bit : i8 : u8
    - 2^8 - 1 = 255
  - 16-bit : i16 : u16
    - 2^16 - 1 = 65,535
  - 32-bit
    - 2^32 - 1 = 4,294,967,295
  - 64-bit
    - 2^64 - 1 = 9,223,372,036,854,775,807
  - 128bit
    - 2^128  − 1
  - arch : isize : usize
    - 64-bit or 32-bit depending on computer architecture

```tip
- i : signed       (negatives      ) ex: -128 - 127
- u: unsigned (no negatives) ex: 0 - 255
- can write integer literals
- default integer type is u32
- The primary situation in which you’d use isize or usize is when indexing some sort of collection.
```

### Floating-point
  - f32 and f64 (default)

### Boolean
  - bool = true : false

### Char
 ```rust
   let c = 'a';  
   // single quotes
 ```
  - Unicode Scalar Values range from <em>U+0000</em> to <em>U+D7FF</em> and <em>U+E000</em> to <em>U+10FFFF</em> inclusive.


## Compound Types

### Tuple
```rust
// Group of variety of types, that have a fixed length when declared //

  let tup: (i32, f64, u8) = (500, 6.4, 1);

  // destructuring a tuple
  let (x, y, z) = tup;

  // dot notation access
  let first_tup = tup.0;
  let sec_tup = tup.1;
 ```
 
 needs review
---
 The tuple without any values, (), is a special type that has only one value, also written (). The type is called the unit type and the value is called the unit value. Expressions implicitly return the unit value if they don’t return any other value.

### Array

