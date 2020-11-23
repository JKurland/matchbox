# matchbox
This crate provides a macro `matchbox!`. This macro allows you to write a `match`-like statement to unpack a boxed trait using downcast.
There is also syntax to allow destructuring of the extracted struct.

### example ###
```rust
let string = matchbox!{my_box,
  Type1 => "something".to_string(),
  Type2: t => format!("t={}", t),
  MyTupleStruct|(a, b) => format!("a={}, b={}", a, b),
  MyStruct|{i, j} => format!("i={}, j={}", i, j),
  else => "something else".to_string(),
};
```

`matchbox!` requires you to add an `else` branch to handle an unknown type.

### ownership ###
`matchbox!` takes ownership of the box you give it. 


### repeat checking ###
`matchbox!` doesn't check for repeated (i.e. unreachable) branches.
