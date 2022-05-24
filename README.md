### Map to Struct

```rust
use std::collections::HashMap;
use structmap::FromMap;
use structmap_derive::FromMap;

#[derive(FromMap, Default)]
struct TestStruct {
    name: String,
    value: i64,
}

fn main() {
    let mut map = HashMap::new();

    hm.insert("name".to_string(), Value::new("example".to_string()));
    hm.insert("value".to_string(), Value::new(0_i64));

    let test: TestStruct = TestStruct::from_map(map);
}
```

### Struct to Map

```rust
use structmap::ToMap;
use structmap_derive::ToMap;

#[derive(ToMap, Default)]
struct TestStruct {
    name: String,
    value: i64,
}

fn main() {
    let test_struct = TestStruct {
        name: "example".to_string(),
        value: 0,
    };

    let map = TestStruct::to_map(test_struct);
}
```
