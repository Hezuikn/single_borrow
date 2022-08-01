```rust
#![feature(type_name_of_val)]

use single_borrow::single_borrow;

fn main() {
    let t = &&&&&&&&&&&&&&&&&&&&&&&&&&&&&42;
    dbg!(type_name_of_val(&t));
    let t: &_ = single_borrow(t);
    dbg!(type_name_of_val(&t));
    dbg!(t);
}

use std::any::type_name_of_val;
```

```test
[src/main.rs:7] type_name_of_val(&t) = "&&&&&&&&&&&&&&&&&&&&&&&&&&&&&i32"
[src/main.rs:9] type_name_of_val(&t) = "&i32"
[src/main.rs:10] t = 42
```
