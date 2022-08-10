```rust
#![feature(type_name_of_val)]

use single_borrow::SingleBorrow;

fn main() {
    let t = &&&&&&42;
    dbg!(type_name_of_val(&t));
    let t = t.single_borrow();
    dbg!(type_name_of_val(&t));
    //broke
    //dbg!(t);
}

use std::any::type_name_of_val;
```

```text
[src/main.rs:7] type_name_of_val(&t) = "&&&&&&i32"
[src/main.rs:9] type_name_of_val(&t) = "&i32"
[src/main.rs:10] t = 42
```
