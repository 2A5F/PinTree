# PinTree
`HashMap/Set<Pin<Arc<T>>>` based fully safety tree collection
## Examples
```rust
let pt = PinTree::<i32>::new();

let a = pt.node(1);
let b = pt.node(2);
let c = pt.node(2);

pt.set_parent(b, a);
pt.set_parent(c, a);
//    a
//  ↙ ↘
// b    c

assert_eq!(pt.is_parent(b, a), true);
assert_eq!(pt.is_child(c, a), true);
```