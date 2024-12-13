## Description
Easy way to build nested structures. Such structures can be used as keys or database paths.

## Usage
```rust
pathify! {
    Root {
        Node1
        Node2 {
            Node21
        }
    }
}

fn main() {
    let root = Root::default();
    assert_eq!(root.node2.node21.to_string(), "root.node_2.node_21");
}
```
