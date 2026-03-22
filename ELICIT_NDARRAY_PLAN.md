# elicit_ndarray — Implementation Plan

> **Premise:** Expose ndarray's N-dimensional array API as MCP tools for both runtime computation and code generation.
> **Approach:** Completionist harvesting using dual-mode tools (primary), fragment tools (generics/parallel), and minimal runtime handles.

---

## Why ndarray is "Similar but Also Widely Used"

**Similar to nalgebra:**
- Natural JSON serialization (arrays → nested JSON)
- Synchronous operations (no async)
- Concrete methods on types (not trait-heavy)
- Clear taxonomy (creation → indexing → operations → manipulation)

**Different from nalgebra:**
- **General N-D arrays** (not just 2D matrices) — up to 6 static dimensions (Ix0-Ix6) + dynamic (IxDyn)
- **Broadcasting semantics** — element-wise ops across different shapes (like NumPy)
- **Parallel operations** — rayon integration for data parallelism
- **View-based slicing** — zero-copy views with arbitrary strides
- **NumPy compatibility** — familiar API for Python users

**Wider adoption:**
- Foundation for scientific computing ecosystem (ndarray-linalg, ndarray-stats, ndarray-rand)
- Used by: polars, image processing, ML libraries, scientific simulations
- NumPy mental model attracts Python → Rust migrations

---

## Core Constraint: Generic Dimensions and Ownership

ndarray uses type-level dimensions and ownership modes:

```rust
pub struct ArrayBase<S, D>
where
    S: Data,
    D: Dimension,
{
    // S determines ownership: OwnedRepr<A>, ViewRepr<&A>, ArcArray<A>
    // D determines shape: Ix0, Ix1, Ix2, ..., Ix6, IxDyn
}
```

**Type aliases:**
```rust
type Array<A, D> = ArrayBase<OwnedRepr<A>, D>;        // Owned
type ArrayView<'a, A, D> = ArrayBase<ViewRepr<&'a A>, D>; // Borrowed (lifetime!)
type ArcArray<A, D> = ArrayBase<ArcArray<A>, D>;      // Shared
```

**Crossing the JSON boundary:**

| Category | Examples | MCP Strategy |
|----------|----------|--------------|
| Fixed-dim arrays | `Array1<f64>`, `Array2<i32>`, `Array3<u8>` | ✅ Dual-mode (serialize as nested JSON) |
| Dynamic arrays | `ArrayD<f64>` (IxDyn) | ✅ Dual-mode (shape in JSON metadata) |
| Views (lifetimes) | `ArrayView2<'a, f64>` | ❌ Cannot serialize (use owned/Arc instead) |
| ArcArray | `ArcArray2<f64>` | ✅ Runtime handles (UUID → Arc) |
| Generic dim code | `fn compute<D: Dimension>(arr: Array<f64, D>)` | ✅ Fragment tools |
| Parallel ops | `arr.par_map_inplace(f)` | ✅ Fragment tools (rayon code gen) |
| Broadcasting | Automatic shape alignment | ✅ Dual-mode (runtime + emit rules) |

---

## Tool Breakdown: 520 Total

### Dual-Mode Tools (400)

Tools that both execute at runtime AND emit code via `CustomEmit`:

#### Array Creation (60)
- **From data** (15): `array_from_vec`, `array_from_shape_vec`, `array_from_elem`, `array_from_fn`, etc.
- **Ranges** (10): `array_range`, `array_linspace`, `array_logspace`, `array_geomspace`, etc.
- **Special values** (10): `array_zeros`, `array_ones`, `array_eye`, `array_full`, etc.
- **Random** (10): `array_rand`, `array_rand_uniform`, `array_rand_normal`, `array_rand_exponential`, etc.
- **From iterators** (10): `array_from_iter`, `array_from_iter_2d`, `collect_rows`, `collect_columns`, etc.
- **Type conversion** (5): `array_from_diag`, `array_from_shape_fn`, etc.

#### Indexing & Slicing (50)
- **Element access** (10): `array_get`, `array_index`, `array_get_mut`, `array_uget` (unchecked), etc.
- **Slicing** (20): `array_slice`, `array_slice_mut`, `slice_axis`, `slice_collapse`, `slice_each_axis`, etc.
- **Views** (10): `array_view`, `array_view_mut`, `into_slice`, `as_slice_memory_order`, etc.
- **Iteration** (10): `array_iter`, `iter_mut`, `indexed_iter`, `axis_iter`, `outer_iter`, etc.

#### Arithmetic Operations (50)
- **Element-wise binary** (15): `array_add`, `array_sub`, `array_mul`, `array_div`, `array_rem`, `array_bitand`, etc.
- **Element-wise unary** (10): `array_neg`, `array_abs`, `array_recip`, `array_mapv`, `array_mapv_inplace`, etc.
- **Scalar ops** (10): `array_add_scalar`, `array_mul_scalar`, `array_pow_scalar`, etc.
- **Comparison** (10): `array_eq`, `array_ne`, `array_lt`, `array_le`, `array_gt`, `array_ge`, etc.
- **Logical** (5): `array_and`, `array_or`, `array_not`, `array_xor`, etc.

#### Broadcasting (30)
- **Auto broadcast** (15): `broadcast_add`, `broadcast_mul`, `broadcast_to`, `broadcast_to_shape`, etc.
- **Manual broadcast** (10): `insert_axis`, `broadcast_axis`, `remove_axis`, `expand_dims`, etc.
- **Shape ops** (5): `broadcast_with`, `broadcast_iter`, etc.

#### Aggregations (40)
- **Full reductions** (15): `sum`, `mean`, `var`, `std`, `min`, `max`, `product`, `all`, `any`, etc.
- **Axis reductions** (15): `sum_axis`, `mean_axis`, `var_axis`, `min_axis`, `max_axis`, etc.
- **Cumulative** (10): `accumulate_axis_inplace`, `scan_axis`, etc.

#### Linear Algebra (40)
- **Matrix ops** (15): `dot`, `matrix_mul`, `outer`, `inner`, `kron`, `vdot`, etc.
- **Transpose/reshape** (10): `transpose`, `t`, `reversed_axes`, `permuted_axes`, `swap_axes`, etc.
- **Norms** (10): `norm`, `norm_l1`, `norm_l2`, `norm_linf`, `norm_axis`, etc.
- **Decompositions** (5): References to ndarray-linalg (SVD, QR, etc. — separate shadow crate)

#### Manipulation (60)
- **Concatenation** (10): `concatenate`, `stack`, `append`, `append_axis`, etc.
- **Splitting** (10): `split_at`, `split_axis`, `split_complex`, etc.
- **Reshape** (15): `reshape`, `into_shape`, `reshape_with_order`, `into_dyn`, `into_dimensionality`, etc.
- **Axis ops** (10): `insert_axis`, `remove_axis`, `merge_axes`, `move_axis`, etc.
- **Flipping** (10): `invert_axis`, `slice_each_axis_inplace`, `reversed_axes`, etc.
- **Cloning** (5): `clone`, `to_owned`, `into_shared`, `into_owned`, etc.

#### Element-wise Functions (40)
- **Math** (20): `map`, `mapv`, `mapv_inplace`, `zip_mut_with`, `fold`, `fold_axis`, etc.
- **Apply** (10): `map_axis`, `map_axis_mut`, `accumulate_axis_inplace`, etc.
- **Zip** (10): `zip`, `azip`, `par_azip` (parallel), `zip_mut_with`, etc.

#### I/O and Serialization (30)
- **CSV** (10): `read_csv`, `write_csv`, `from_csv_string`, `to_csv_string`, etc.
- **Binary** (10): `serialize`, `deserialize`, `to_bytes`, `from_bytes`, etc.
- **Display** (10): `to_string`, `fmt_table`, `display_shape`, etc.

### Fragment Tools (80)

Code generation for generic dimensions, parallel operations, and complex patterns:

#### Generic Dimension Code (30)
- `emit_array_type` — `Array<T, Ix2>`, `ArrayD<f64>`, generic `Array<T, D>`
- `emit_function_generic_dim` — Functions with `D: Dimension` parameter
- `emit_fixed_dim_function` — Functions for specific Ix1, Ix2, etc.
- `emit_dyn_dim_function` — Functions using IxDyn (dynamic dims)
- `emit_const_dim_bounds` — Where clauses for dimension constraints
- And 25 more for dimension-generic patterns

#### Parallel Operation Code (20)
- `emit_par_map_inplace` — Parallel element-wise modification
- `emit_par_azip` — Parallel zip macro invocations
- `emit_par_chunks` — Parallel chunk processing
- `emit_rayon_iterator` — Rayon parallel iterator chains
- And 16 more for parallel patterns

#### Broadcasting Code (15)
- `emit_broadcast_binary_op` — Generate broadcasting arithmetic
- `emit_explicit_broadcast` — Manual broadcast with shape checks
- `emit_broadcast_assign` — Broadcasted assignment
- And 12 more for broadcast patterns

#### Complete Assembly (15)
- `emit_module_ndarray` — Complete ndarray module
- `emit_struct_with_arrays` — Structs containing array fields
- `emit_impl_block_array_ops` — Impl blocks with array operations
- `emit_test_suite_ndarray` — Test suite generation
- `assemble_ndarray_binary` — Full executable with ndarray
- And 10 more for assembly patterns

### Runtime-Only Tools (40)

UUID-keyed handles for persistent arrays and workflows:

| Registry | Operations | Count |
|----------|-----------|-------|
| **ArrayRegistry** | `array_create_handle`, `array_get`, `array_set`, `array_delete`, `array_clone_handle` | 15 |
| **ViewRegistry** | `view_create`, `view_slice`, `view_reshape`, `view_delete` | 10 |
| **IteratorRegistry** | `iterator_create`, `iterator_next`, `iterator_collect` | 10 |
| **ParallelRegistry** | `parallel_map`, `parallel_reduce`, `parallel_zip` | 5 |

**Why handles?**
- Long-lived arrays in agent workflows
- Chained transformations without full serialization
- Shared arrays across multiple operations (ArcArray)
- Lazy iteration and chunking

---

## Serialization Strategy

### 1D Array

```json
{
  "type": "Array1<f64>",
  "shape": [5],
  "data": [1.0, 2.0, 3.0, 4.0, 5.0]
}
```

### 2D Array (Matrix)

```json
{
  "type": "Array2<f64>",
  "shape": [3, 3],
  "data": [
    [1.0, 2.0, 3.0],
    [4.0, 5.0, 6.0],
    [7.0, 8.0, 9.0]
  ]
}
```

### 3D Array

```json
{
  "type": "Array3<f64>",
  "shape": [2, 3, 4],
  "data": [
    [[1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0], [9.0, 10.0, 11.0, 12.0]],
    [[13.0, 14.0, 15.0, 16.0], [17.0, 18.0, 19.0, 20.0], [21.0, 22.0, 23.0, 24.0]]
  ]
}
```

### Dynamic Dimensions (IxDyn)

```json
{
  "type": "ArrayD<f64>",
  "ndim": 4,
  "shape": [2, 3, 4, 5],
  "data": [...],  // Flattened row-major
  "layout": "row_major"  // or "column_major"
}
```

### Slicing Info

```json
{
  "type": "ArrayView2<f64>",
  "base_array_id": "uuid-of-parent",
  "slice": {
    "axis_0": { "start": 0, "end": 5, "step": 1 },
    "axis_1": { "start": 2, "end": 8, "step": 2 }
  }
}
```

---

## Phase 1: Core Array Creation & Indexing (Dual-Mode)

**Goal:** Establish dual-mode pattern for array creation and basic access.

### Crate Structure

```
crates/elicit_ndarray/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── creation.rs      # Array creation dual-mode tools
    ├── indexing.rs      # Indexing/slicing dual-mode tools
    ├── handles.rs       # UUID registries
    └── serde_types.rs   # JSON serialization wrappers
```

### Cargo.toml

```toml
[package]
name = "elicit_ndarray"
version = "0.1.0"
edition = "2021"

[dependencies]
elicitation = { workspace = true }
elicitation_derive.workspace = true
ndarray = { version = "0.17", features = ["serde"] }
serde = { workspace = true, features = ["derive"] }
serde_json.workspace = true
uuid.workspace = true
rmcp.workspace = true
tracing.workspace = true

[features]
emit = ["dep:quote", "elicitation/emit"]
rayon = ["ndarray/rayon"]
```

### Dual-Mode Tool Example: Array Creation

```rust
use elicitation_derive::elicit_tool;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrayZerosParams {
    pub shape: Vec<usize>,
    pub element_type: String,  // "f64", "i32", etc.
}

#[elicit_tool(
    plugin = "ndarray_creation",
    name = "array_zeros",
    description = "Create array filled with zeros",
    emit = Auto
)]
async fn array_zeros(p: ArrayZerosParams) -> Result<CallToolResult, ErrorData> {
    // Runtime: create array, serialize
    let shape = IxDyn(&p.shape);
    let array = Array::<f64, _>::zeros(shape);
    let result = ArrayJson::from_array(&array);

    Ok(CallToolResult::success(json!({ "array": result })))
}

// Auto-generated CustomEmit impl:
impl CustomEmit<ArrayZerosParams> for ArrayZerosEmit {
    fn emit_code(params: &ArrayZerosParams) -> TokenStream {
        let shape_dims = &params.shape;
        let element_type: TokenStream = params.element_type.parse().unwrap();

        match params.shape.len() {
            1 => quote! {
                Array1::<#element_type>::zeros(#(#shape_dims),*)
            },
            2 => quote! {
                Array2::<#element_type>::zeros((#(#shape_dims),*))
            },
            _ => quote! {
                ArrayD::<#element_type>::zeros(IxDyn(&[#(#shape_dims),*]))
            },
        }
    }
}
```

### ArrayJson Wrapper Type

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ArrayJson {
    Array1 { shape: [usize; 1], data: Vec<f64> },
    Array2 { shape: [usize; 2], data: Vec<Vec<f64>> },
    Array3 { shape: [usize; 3], data: Vec<Vec<Vec<f64>>> },
    ArrayD { ndim: usize, shape: Vec<usize>, data: Vec<f64> },
}

impl ArrayJson {
    pub fn from_array<D: Dimension>(arr: &Array<f64, D>) -> Self {
        let shape = arr.shape();
        match arr.ndim() {
            1 => ArrayJson::Array1 {
                shape: [shape[0]],
                data: arr.iter().copied().collect(),
            },
            2 => ArrayJson::Array2 {
                shape: [shape[0], shape[1]],
                data: arr.outer_iter()
                    .map(|row| row.iter().copied().collect())
                    .collect(),
            },
            _ => {
                let data = arr.iter().copied().collect();
                ArrayJson::ArrayD {
                    ndim: arr.ndim(),
                    shape: shape.to_vec(),
                    data,
                }
            }
        }
    }

    pub fn to_array_dyn(&self) -> Result<ArrayD<f64>, String> {
        match self {
            ArrayJson::Array1 { shape, data } => {
                Ok(Array::from_vec(data.clone()).into_dyn())
            }
            ArrayJson::Array2 { shape, data } => {
                let flat: Vec<f64> = data.iter().flatten().copied().collect();
                Array::from_shape_vec(IxDyn(&[shape[0], shape[1]]), flat)
                    .map_err(|e| e.to_string())
            }
            ArrayJson::ArrayD { shape, data, .. } => {
                Array::from_shape_vec(IxDyn(shape), data.clone())
                    .map_err(|e| e.to_string())
            }
            _ => todo!(),
        }
    }
}
```

### Dual-Mode Tool Example: Slicing

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArraySliceParams {
    pub array: ArrayJson,
    pub slices: Vec<SliceInfo>,  // Per-axis slice info
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SliceInfo {
    pub start: Option<isize>,
    pub end: Option<isize>,
    pub step: isize,
}

#[elicit_tool(
    plugin = "ndarray_indexing",
    name = "array_slice",
    description = "Slice array along axes",
    emit = Auto
)]
async fn array_slice(p: ArraySliceParams) -> Result<CallToolResult, ErrorData> {
    let array = p.array.to_array_dyn()?;

    // Build slice info for each axis
    let sliced = array.slice(s![..;2, 1..5]);  // Example
    let result = ArrayJson::from_array(&sliced.to_owned());

    Ok(CallToolResult::success(json!({ "array": result })))
}

impl CustomEmit<ArraySliceParams> for ArraySliceEmit {
    fn emit_code(params: &ArraySliceParams) -> TokenStream {
        let array_code = emit_array_literal(&params.array);
        let slice_args = params.slices.iter().map(emit_slice_arg);

        quote! {
            {
                let arr = #array_code;
                arr.slice(s![#(#slice_args),*]).to_owned()
            }
        }
    }
}

fn emit_slice_arg(info: &SliceInfo) -> TokenStream {
    match (info.start, info.end, info.step) {
        (None, None, 1) => quote! { .. },
        (Some(s), None, 1) => quote! { #s.. },
        (None, Some(e), 1) => quote! { ..#e },
        (Some(s), Some(e), 1) => quote! { #s..#e },
        (Some(s), Some(e), step) => quote! { #s..#e; #step },
        _ => panic!("Invalid slice info"),
    }
}
```

---

## Phase 2: Arithmetic & Broadcasting (Dual-Mode)

**Goal:** Element-wise operations and broadcasting semantics.

### Element-wise Binary Operations (15 dual-mode)

```rust
#[elicit_tool(
    plugin = "ndarray_arithmetic",
    name = "array_add",
    description = "Element-wise addition with broadcasting",
    emit = Auto
)]
async fn array_add(p: BinaryOpParams) -> Result<CallToolResult, ErrorData> {
    let lhs = p.lhs.to_array_dyn()?;
    let rhs = p.rhs.to_array_dyn()?;

    // Broadcasting happens automatically via ndarray
    let result = &lhs + &rhs;
    let result_json = ArrayJson::from_array(&result);

    Ok(CallToolResult::success(json!({ "result": result_json })))
}

impl CustomEmit<BinaryOpParams> for ArrayAddEmit {
    fn emit_code(params: &BinaryOpParams) -> TokenStream {
        let lhs_code = emit_array_literal(&params.lhs);
        let rhs_code = emit_array_literal(&params.rhs);

        quote! {
            {
                let lhs = #lhs_code;
                let rhs = #rhs_code;
                &lhs + &rhs
            }
        }
    }
}
```

### Broadcasting Explicit (15 dual-mode)

```rust
#[elicit_tool(
    plugin = "ndarray_broadcasting",
    name = "broadcast_to_shape",
    description = "Broadcast array to target shape",
    emit = Auto
)]
async fn broadcast_to_shape(p: BroadcastParams) -> Result<CallToolResult, ErrorData> {
    let array = p.array.to_array_dyn()?;
    let target_shape = IxDyn(&p.target_shape);

    let broadcasted = array.broadcast(target_shape)
        .ok_or_else(|| ErrorData::new("Cannot broadcast to target shape"))?;
    let result = ArrayJson::from_array(&broadcasted.to_owned());

    Ok(CallToolResult::success(json!({ "result": result })))
}
```

---

## Phase 3: Aggregations & Linear Algebra (Dual-Mode)

**Goal:** Reductions, matrix multiplication, norms.

### Aggregation Tools (40 dual-mode)

```rust
#[elicit_tool(
    plugin = "ndarray_aggregation",
    name = "array_sum",
    description = "Sum all elements",
    emit = Auto
)]
async fn array_sum(p: ArraySumParams) -> Result<CallToolResult, ErrorData> {
    let array = p.array.to_array_dyn()?;
    let sum: f64 = array.sum();

    Ok(CallToolResult::success(json!({ "sum": sum })))
}

#[elicit_tool(
    plugin = "ndarray_aggregation",
    name = "array_sum_axis",
    description = "Sum along specific axis",
    emit = Auto
)]
async fn array_sum_axis(p: ArraySumAxisParams) -> Result<CallToolResult, ErrorData> {
    let array = p.array.to_array_dyn()?;
    let result = array.sum_axis(Axis(p.axis));
    let result_json = ArrayJson::from_array(&result);

    Ok(CallToolResult::success(json!({ "result": result_json })))
}
```

### Linear Algebra Tools (40 dual-mode)

```rust
#[elicit_tool(
    plugin = "ndarray_linalg",
    name = "array_dot",
    description = "Matrix/vector dot product",
    emit = Auto
)]
async fn array_dot(p: DotParams) -> Result<CallToolResult, ErrorData> {
    let lhs = p.lhs.to_array_dyn()?;
    let rhs = p.rhs.to_array_dyn()?;

    // Use general_mat_mul or dot depending on dimensions
    let result = lhs.dot(&rhs);
    let result_json = ArrayJson::from_array(&result);

    Ok(CallToolResult::success(json!({ "result": result_json })))
}

#[elicit_tool(
    plugin = "ndarray_linalg",
    name = "array_transpose",
    description = "Transpose (reverse axes)",
    emit = Auto
)]
async fn array_transpose(p: TransposeParams) -> Result<CallToolResult, ErrorData> {
    let array = p.array.to_array_dyn()?;
    let result = array.t().to_owned();
    let result_json = ArrayJson::from_array(&result);

    Ok(CallToolResult::success(json!({ "result": result_json })))
}
```

---

## Phase 4: Manipulation & I/O (Dual-Mode)

**Goal:** Concatenation, reshaping, serialization.

### Concatenation Tools (10 dual-mode)

```rust
#[elicit_tool(
    plugin = "ndarray_manipulation",
    name = "array_concatenate",
    description = "Concatenate arrays along axis",
    emit = Auto
)]
async fn array_concatenate(p: ConcatenateParams) -> Result<CallToolResult, ErrorData> {
    let arrays: Vec<ArrayD<f64>> = p.arrays.iter()
        .map(|a| a.to_array_dyn())
        .collect::<Result<_, _>>()?;

    let views: Vec<_> = arrays.iter().map(|a| a.view()).collect();
    let result = ndarray::concatenate(Axis(p.axis), &views)
        .map_err(|e| ErrorData::new(e.to_string()))?;

    let result_json = ArrayJson::from_array(&result);
    Ok(CallToolResult::success(json!({ "result": result_json })))
}
```

### Reshape Tools (15 dual-mode)

```rust
#[elicit_tool(
    plugin = "ndarray_manipulation",
    name = "array_reshape",
    description = "Reshape array to new dimensions",
    emit = Auto
)]
async fn array_reshape(p: ReshapeParams) -> Result<CallToolResult, ErrorData> {
    let array = p.array.to_array_dyn()?;
    let new_shape = IxDyn(&p.new_shape);

    let reshaped = array.into_shape(new_shape)
        .map_err(|e| ErrorData::new(e.to_string()))?;

    let result_json = ArrayJson::from_array(&reshaped);
    Ok(CallToolResult::success(json!({ "result": result_json })))
}
```

---

## Phase 5: Fragment Tools (Generic & Parallel)

**Goal:** Generate code with generic dimensions and parallel operations.

### Generic Dimension Fragments (30)

```rust
#[elicit_tool(
    plugin = "ndarray_fragments",
    name = "emit_array_type",
    description = "Emit array type with generic or fixed dimensions",
    emit = Auto
)]
async fn emit_array_type(p: EmitArrayTypeParams) -> Result<CallToolResult, ErrorData> {
    let code = match p.dim_type.as_str() {
        "Array1" => format!("Array1<{}>", p.element_type),
        "Array2" => format!("Array2<{}>", p.element_type),
        "ArrayD" => format!("ArrayD<{}>", p.element_type),
        "generic" => format!("Array<{}, D> where D: Dimension", p.element_type),
        _ => return Err(ErrorData::new("Unknown dimension type")),
    };

    Ok(CallToolResult::success(Content::text(code)))
}

#[elicit_tool(
    plugin = "ndarray_fragments",
    name = "emit_function_generic_dim",
    description = "Emit function with Dimension bound",
    emit = Auto
)]
async fn emit_function_generic_dim(p: EmitGenericDimParams) -> Result<CallToolResult, ErrorData> {
    let code = format!(
        r#"fn {}<D: Dimension>(arr: Array<f64, D>) -> Array<f64, D> {{
    {}
}}"#,
        p.function_name,
        p.body
    );

    Ok(CallToolResult::success(Content::text(code)))
}
```

### Parallel Operation Fragments (20)

```rust
#[elicit_tool(
    plugin = "ndarray_fragments",
    name = "emit_par_map_inplace",
    description = "Emit parallel element-wise modification code",
    emit = Auto
)]
async fn emit_par_map_inplace(p: EmitParMapParams) -> Result<CallToolResult, ErrorData> {
    let code = format!(
        r#"arr.par_map_inplace(|x| {});"#,
        p.closure_body
    );

    Ok(CallToolResult::success(Content::text(code)))
}

#[elicit_tool(
    plugin = "ndarray_fragments",
    name = "emit_par_azip",
    description = "Emit parallel azip! macro invocation",
    emit = Auto
)]
async fn emit_par_azip(p: EmitParAzipParams) -> Result<CallToolResult, ErrorData> {
    let bindings = p.bindings.iter()
        .map(|b| format!("{} {}", b.mode, b.name))
        .collect::<Vec<_>>()
        .join(", ");

    let code = format!(
        r#"par_azip!(({}) {});"#,
        bindings,
        p.body
    );

    Ok(CallToolResult::success(Content::text(code)))
}
```

### Complete Assembly (15)

```rust
#[elicit_tool(
    plugin = "ndarray_fragments",
    name = "assemble_ndarray_binary",
    description = "Generate complete executable with ndarray computations",
    emit = Auto
)]
async fn assemble_ndarray_binary(p: AssembleParams) -> Result<CallToolResult, ErrorData> {
    let cargo_toml = generate_cargo_toml(&p);
    let main_rs = generate_main_with_arrays(&p);

    Ok(CallToolResult::success(json!({
        "Cargo.toml": cargo_toml,
        "src/main.rs": main_rs,
        "description": "Complete ndarray binary project"
    })))
}

fn generate_main_with_arrays(p: &AssembleParams) -> String {
    let features = if p.use_rayon {
        r#"ndarray = { version = "0.17", features = ["rayon", "serde"] }"#
    } else {
        r#"ndarray = { version = "0.17", features = ["serde"] }"#
    };

    format!(
        r#"use ndarray::{{Array, Array1, Array2, ArrayD, IxDyn, Axis, s}};
{}

fn main() {{
    {}
}}

{}
"#,
        if p.use_rayon { "use ndarray::parallel::prelude::*;" } else { "" },
        p.main_body,
        p.helper_functions.join("\n\n")
    )
}
```

---

## Phase 6: UUID-Keyed Handles (Runtime-Only)

**Goal:** Persistent array handles for stateful workflows.

### ArrayRegistry (15 runtime tools)

```rust
pub struct NdarrayPlugin {
    arrays: Arc<Mutex<HashMap<Uuid, ArcArray<f64, IxDyn>>>>,
    views: Arc<Mutex<HashMap<Uuid, ViewHandle>>>,
    iterators: Arc<Mutex<HashMap<Uuid, IteratorHandle>>>,
}

#[elicit_tool(
    plugin = "ndarray_handles",
    name = "array_create_handle",
    description = "Create persistent array handle (ArcArray for sharing)"
)]
async fn array_create_handle(p: ArrayCreateParams) -> Result<CallToolResult, ErrorData> {
    let array = p.array.to_array_dyn()?;
    let arc_array = array.into_shared();
    let id = Uuid::new_v4();

    let plugin = get_plugin();
    plugin.arrays.lock().unwrap().insert(id, arc_array);

    Ok(CallToolResult::success(json!({ "array_id": id })))
}

#[elicit_tool(
    plugin = "ndarray_handles",
    name = "array_get_handle",
    description = "Retrieve array by handle"
)]
async fn array_get_handle(p: ArrayGetParams) -> Result<CallToolResult, ErrorData> {
    let plugin = get_plugin();
    let arrays = plugin.arrays.lock().unwrap();
    let array = arrays.get(&p.array_id)
        .ok_or_else(|| ErrorData::new("Array not found"))?;

    let result = ArrayJson::from_array(&array.view());
    Ok(CallToolResult::success(json!({ "array": result })))
}

#[elicit_tool(
    plugin = "ndarray_handles",
    name = "array_compose_handles",
    description = "Perform operation on two array handles, store result"
)]
async fn array_compose_handles(p: ArrayComposeParams) -> Result<CallToolResult, ErrorData> {
    let plugin = get_plugin();
    let arrays = plugin.arrays.lock().unwrap();

    let lhs = arrays.get(&p.lhs_id).ok_or_else(|| ErrorData::new("LHS not found"))?;
    let rhs = arrays.get(&p.rhs_id).ok_or_else(|| ErrorData::new("RHS not found"))?;

    let result = match p.operation.as_str() {
        "add" => (lhs.view() + rhs.view()).into_shared(),
        "mul" => (lhs.view() * rhs.view()).into_shared(),
        "dot" => lhs.dot(&rhs.view()).into_shared(),
        _ => return Err(ErrorData::new("Unknown operation")),
    };

    let result_id = Uuid::new_v4();
    drop(arrays);
    plugin.arrays.lock().unwrap().insert(result_id, result);

    Ok(CallToolResult::success(json!({ "result_id": result_id })))
}
```

### ViewRegistry (10 runtime tools)

```rust
pub enum ViewHandle {
    Slice { parent_id: Uuid, slice_info: Vec<SliceInfo> },
    Transpose { parent_id: Uuid },
    Reshape { parent_id: Uuid, shape: Vec<usize> },
}

#[elicit_tool(
    plugin = "ndarray_handles",
    name = "view_create_slice",
    description = "Create zero-copy slice view of array"
)]
async fn view_create_slice(p: ViewSliceParams) -> Result<CallToolResult, ErrorData> {
    let plugin = get_plugin();
    let arrays = plugin.arrays.lock().unwrap();

    // Verify parent exists
    arrays.get(&p.parent_id)
        .ok_or_else(|| ErrorData::new("Parent array not found"))?;

    let view_handle = ViewHandle::Slice {
        parent_id: p.parent_id,
        slice_info: p.slices,
    };

    let view_id = Uuid::new_v4();
    drop(arrays);
    plugin.views.lock().unwrap().insert(view_id, view_handle);

    Ok(CallToolResult::success(json!({ "view_id": view_id })))
}
```

---

## Relationship to ndarray-linalg

**ndarray-linalg** is a separate crate providing advanced linear algebra (LAPACK/OpenBLAS bindings):
- SVD, QR, LU, Cholesky decompositions
- Eigenvalue/eigenvector computations
- Matrix inverse, determinant, rank

**Strategy:**
- **elicit_ndarray** focuses on core array operations (this plan)
- **elicit_ndarray_linalg** would be a separate shadow crate (future plan)
- Reference ndarray-linalg in Phase 3 tools, but defer advanced decompositions

**Example reference tool:**

```rust
#[elicit_tool(
    plugin = "ndarray_linalg",
    name = "array_svd",
    description = "Compute SVD (requires elicit_ndarray_linalg plugin)"
)]
async fn array_svd(p: SvdParams) -> Result<CallToolResult, ErrorData> {
    Err(ErrorData::new(
        "SVD requires elicit_ndarray_linalg plugin. \
         Use `nalgebra` or install `elicit_ndarray_linalg` for advanced linear algebra."
    ))
}
```

---

## Implementation Order

1. **Phase 1a** — Crate scaffold: `Cargo.toml`, `lib.rs`, `serde_types.rs`
2. **Phase 1b** — Creation dual-mode tools: `creation.rs` (60 tools)
3. **Phase 1c** — Indexing dual-mode tools: `indexing.rs` (50 tools)
4. **Phase 1d** — `just check elicit_ndarray`; fix compilation
5. **Phase 2a** — Arithmetic dual-mode tools: element-wise ops (50 tools)
6. **Phase 2b** — Broadcasting dual-mode tools: (30 tools)
7. **Phase 2c** — `just check elicit_ndarray`
8. **Phase 3a** — Aggregation dual-mode tools: (40 tools)
9. **Phase 3b** — Linear algebra dual-mode tools: (40 tools)
10. **Phase 3c** — `just check elicit_ndarray`
11. **Phase 4a** — Manipulation dual-mode tools: (60 tools)
12. **Phase 4b** — I/O dual-mode tools: (30 tools)
13. **Phase 4c** — `just check elicit_ndarray`
14. **Phase 5a** — Fragment tools: generic dimensions (30 tools)
15. **Phase 5b** — Fragment tools: parallel ops (20 tools)
16. **Phase 5c** — Fragment tools: assembly (15 tools)
17. **Phase 5d** — `just check elicit_ndarray`
18. **Phase 6a** — UUID handles: ArrayRegistry, ViewRegistry (40 tools)
19. **Phase 6b** — `just check elicit_ndarray`
20. **Phase 7** — Wire into `elicit_server` emit chain

---

## Tool Count Summary

| Category | Count | Implementation Strategy |
|----------|-------|------------------------|
| Dual-Mode Creation | 60 | `emit = Auto` + CustomEmit |
| Dual-Mode Indexing/Slicing | 50 | `emit = Auto` + CustomEmit |
| Dual-Mode Arithmetic | 50 | `emit = Auto` + CustomEmit |
| Dual-Mode Broadcasting | 30 | `emit = Auto` + CustomEmit |
| Dual-Mode Aggregations | 40 | `emit = Auto` + CustomEmit |
| Dual-Mode Linear Algebra | 40 | `emit = Auto` + CustomEmit |
| Dual-Mode Manipulation | 60 | `emit = Auto` + CustomEmit |
| Dual-Mode I/O | 30 | `emit = Auto` + CustomEmit |
| Fragment Generic Dims | 30 | Code generation only |
| Fragment Parallel Ops | 20 | Code generation only |
| Fragment Assembly | 15 | Code generation only |
| Fragment Broadcasting | 15 | Code generation only |
| Runtime Handles | 40 | UUID registries |
| **Total** | **520** | |

---

## Key Advantages

1. **NumPy Familiarity**: Python users recognize the API immediately
2. **Natural Serialization**: N-D arrays → nested JSON, shape metadata
3. **Dual-Mode Dominance**: 400/520 tools (77%) are dual-mode
4. **Broadcasting Support**: Element-wise ops automatically broadcast (like NumPy)
5. **Zero-Copy Views**: UUID handles enable efficient slicing workflows
6. **Parallel Ready**: Fragment tools generate rayon parallel code
7. **Generic Dimensions**: Support both static (Ix2) and dynamic (IxDyn) arrays
8. **Wide Ecosystem**: Foundation for ndarray-linalg, ndarray-stats, ndarray-rand

---

## Comparison to nalgebra

| Aspect | nalgebra | ndarray |
|--------|----------|---------|
| **Focus** | Linear algebra + geometry | General N-D arrays |
| **Use case** | Graphics, physics, robotics | Scientific computing, data analysis |
| **Dimensions** | 2D matrices primary | N-D arrays (up to 6+ dimensions) |
| **Special types** | Rotations, quaternions, transforms | Slicing, broadcasting, parallel ops |
| **Tool count** | 480 tools | 520 tools |
| **Dual-mode %** | 73% | 77% |
| **Key feature** | Geometric types + decompositions | Broadcasting + NumPy compatibility |

Both are "straightforward" because:
- Natural JSON serialization
- Synchronous operations
- Concrete methods (not trait-heavy)
- Clear API taxonomy

---

## Sources

- [ndarray - Rust (docs.rs)](https://docs.rs/ndarray/latest/ndarray/)
- [GitHub - rust-ndarray/ndarray](https://github.com/rust-ndarray/ndarray)
- [ndarray for NumPy users](https://docs.rs/ndarray/latest/ndarray/doc/ndarray_for_numpy_users/index.html)
- [ndarray - crates.io](https://crates.io/crates/ndarray)
- [ndarray Quick Start](https://github.com/rust-ndarray/ndarray/blob/master/README-quick-start.md)
