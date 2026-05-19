# elicit_rstar

`elicit_rstar` is the [elicitation] shadow crate for the [rstar] R\*-tree spatial index library.
It provides factory-driven dynamic MCP tool support for `rstar::RTree<T>`, keeping the runtime API
generic over the stored item type while using elicitable wrappers for the built-in 2D rstar
primitives.

## Dynamic factories

Two capability-based factories are provided based on the bounds the stored item type implements:

| Factory | Item bound | Tools |
|---|---|---|
| `RTreeObjectFactory` | `T: ElicitComplete + RTreeObject<Envelope = AABB<[f64; 2]>>` | `new`, `bulk_load`, `size`, `items`, `insert`, `locate_in_envelope`, `locate_in_envelope_intersecting` |
| `PointDistanceFactory` | above + `PointDistance` | all above plus `nearest_neighbor`, `nearest_neighbors`, `locate_all_at_point`, `locate_within_distance` |

## Usage

Register a tree snapshot type with the dynamic registry, prime the matching factory, then
instantiate the factory meta-tool:

```rust
use elicit_rstar::{RstarTree, prime_point_distance_tree, prime_rtree_object_tree};
use elicitation::{DynamicToolRegistry, RstarRectangle};

prime_rtree_object_tree::<RstarRectangle>();
prime_point_distance_tree::<RstarRectangle>();

let registry = DynamicToolRegistry::new()
    .register_type::<RstarTree<RstarRectangle>>("rect_tree");
```

Built-in tree aliases for the standard 2D rstar primitives:

| Alias | Item type |
|---|---|
| `RstarTree<T>` | Generic tree snapshot |
| `RectangleTree` | `RstarTree<RstarRectangle>` |
| `LineTree` | `RstarTree<RstarLine>` |

```toml
[dependencies]
elicit_rstar = "0.11"
```

[elicitation]: https://crates.io/crates/elicitation
[rstar]: https://crates.io/crates/rstar
