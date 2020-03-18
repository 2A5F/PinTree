# PinTree
`HashMap/Set<Pin<Arc<T>>>` based fully safety tree collection
## Examples
```rust
let mut pt = PinTree::<i32>::new();

let a = &pt.node(1);
let b = &pt.node(2);
let c = &pt.node(3);

pt.set_parent(b, a);
pt.set_parent(c, a);
//    a
//  ↙  ↘
// b     c

assert_eq!(pt.is_parent(b, a), true);
assert_eq!(pt.is_child(a, c), true);
```
```rust
// Circular references are safe
pt.set_parent(b, a);
pt.set_parent(a, c);
pt.set_parent(c, b);
//    a
//  ↙  ↖
// b  →  c

pt.set_parent(a, a);
// a ⟲
```
```rust
let mut pt = PinTree::<Mutex<i32>>::new();

let a = pt.node(Mutex::new(1));
let mut x = a.lock().unwrap();
assert_eq!(*x, 1);
*x = 2;
assert_eq!(*x, 2);
```