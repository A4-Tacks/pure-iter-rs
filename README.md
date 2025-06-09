Iterator adapter without side-effects, possible better performance

# Examples

```rust
use std::sync::atomic::{AtomicU32, Ordering::*};
use pure_iter::PureIterExt;

let mut a = AtomicU32::new(0);
let mut b = AtomicU32::new(0);

let c = (0..3).map(|_| a.fetch_add(1, Release)).nth(2);
let d = (0..3).pure_map(|_| b.fetch_add(1, Release)).nth(2);

assert_eq!(c, Some(2));
assert_eq!(d, Some(0));

assert_eq!(a.load(Acquire), 3);
assert_eq!(b.load(Acquire), 1);
```
