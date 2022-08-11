# easy_safe

> An easy unstable crate to save strings into a map that is saved on the disk
 creates a new map environment with the specified name
    the name corresponds to the file saved and loaded in your filesystem
 If the file is already there it will load from that file

  This means you can always come back and access your file if you call it with the right name

Example

```rust
use easy_safe::{create_or_load_map_env, MapEnv};
  
let mut  map_env: MapEnv = create_or_load_map_env("somename");
map_env.put("somekey", "somevalue");
let value = map_env.get("somekey").unwrap();
assert_eq!(value, "somevalue");
    
let mut same_file_map_env: MapEnv = create_or_load_map_env("somename");
let also_the_value = same_file_map_env.get("somekey").unwrap();
assert_eq!(value, "somevalue");
```
