# elicit_nalgebra — Implementation Plan

> **Premise:** Expose nalgebra's linear algebra API as MCP tools for both runtime computation and code generation.
> **Approach:** Completionist harvesting using dual-mode tools (primary), fragment tools (generics), and minimal factory pattern.

---

## Why nalgebra is "More Straightforward"

Compared to previous shadow crates:

| Library | Challenge | nalgebra Advantage |
|---------|-----------|-------------------|
| num-traits | Trait-heavy, needs factory pattern | Concrete methods on types |
| Leptos | Macro-driven, attribute macro harvesting | No macros to wrap |
| Axum | Async/Service abstractions, trait bounds | Synchronous math operations |
| Polars | Large API surface (DataFrame, SQL, etc.) | Focused on linear algebra |

**Key simplifications:**
1. **Serialization is natural**: Matrices → nested arrays `[[1,2],[3,4]]`, Vectors → arrays `[1,2,3]`
2. **Operations are concrete**: `matrix.transpose()`, `vector.normalize()` - not trait method dispatch
3. **No lifetime complexity**: Most types are owned (`Matrix`, `Vector`, `Rotation`)
4. **Clear taxonomy**: Matrix ops, Vector ops, Geometric ops, Decompositions

---

## Core Constraint: Generic Dimensions

nalgebra uses const generics for compile-time dimensions:

```rust
pub struct Matrix<T, R, C, S>
where
    R: Dim,
    C: Dim,
    S: Storage<T, R, C>,
{ /* ... */ }
```

**Crossing the JSON boundary:**

| Category | Examples | MCP Strategy |
|----------|----------|--------------|
| Fixed-size types | `Matrix3<f64>`, `Vector4<f32>` | ✅ Dual-mode tools (serialize as JSON arrays) |
| Dynamic types | `DMatrix`, `DVector` | ✅ Dual-mode tools (dimensions in JSON) |
| Generic code | `fn compute<const N: usize>(v: SVector<f64, N>)` | ✅ Fragment tools |
| Scalar generics | `Matrix3<T> where T: RealField` | ✅ Fragment tools + minimal factory |
| Decompositions | `SVD`, `QR`, `Cholesky` | ✅ Dual-mode tools (return serializable results) |

---

## Tool Breakdown: 480 Total

### Runtime-Only Tools (60)
UUID-keyed handles for persistent matrices/vectors:

| Registry | Operations | Count |
|----------|-----------|-------|
| **MatrixRegistry** | `matrix_create`, `matrix_get`, `matrix_set`, `matrix_delete` | 15 |
| **VectorRegistry** | `vector_create`, `vector_get`, `vector_set`, `vector_delete` | 10 |
| **DecompositionRegistry** | `svd_create`, `qr_create`, `lu_create`, `decomp_query` | 20 |
| **GeometryRegistry** | `rotation_create`, `transform_create`, `compose`, `apply` | 15 |

**Why handles?** Long-lived matrices in workflows, chained decompositions, reusable transforms.

### Dual-Mode Tools (350)

Tools that both execute at runtime AND emit code via `CustomEmit`:

#### Matrix Operations (120)
- **Creation** (20): `matrix_zeros`, `matrix_identity`, `matrix_from_rows`, `matrix_from_fn`, etc.
- **Arithmetic** (25): `matrix_add`, `matrix_mul`, `matrix_sub`, `matrix_component_mul`, `matrix_scale`, etc.
- **Transformations** (20): `transpose`, `inverse`, `adjoint`, `normalize_columns`, etc.
- **Slicing/Views** (15): `get_row`, `get_column`, `slice`, `fixed_slice`, etc.
- **Properties** (15): `determinant`, `trace`, `rank`, `is_square`, `is_invertible`, etc.
- **Solvers** (15): `solve_lower_triangular`, `solve_upper_triangular`, `try_inverse`, `pseudo_inverse`, etc.
- **Norms** (10): `norm`, `norm_squared`, `magnitude`, `metric_distance`, etc.

#### Vector Operations (80)
- **Creation** (15): `vector_zeros`, `vector_from_iterator`, `vector_repeat`, `vector_from_fn`, etc.
- **Arithmetic** (20): `vector_add`, `vector_sub`, `vector_scale`, `vector_component_mul`, etc.
- **Geometric** (20): `dot`, `cross`, `normalize`, `angle`, `project`, `reflect`, etc.
- **Transformations** (10): `cap_magnitude`, `try_normalize`, `lerp`, `slerp`, etc.
- **Properties** (15): `norm`, `norm_squared`, `is_normalized`, `min`, `max`, `sum`, etc.

#### Geometric Types (80)
- **Rotations** (25): `rotation2_new`, `rotation3_from_euler`, `quaternion_slerp`, `axis_angle`, etc.
- **Translations** (10): `translation_new`, `translation_inverse`, `translate_point`, etc.
- **Isometries** (15): `isometry_new`, `isometry_inverse`, `transform_point`, `transform_vector`, etc.
- **Similarities** (10): `similarity_new`, `similarity_inverse`, `scaling`, etc.
- **Transforms** (20): `affine_new`, `projective_new`, `perspective_new`, `orthographic_new`, etc.

#### Decompositions (70)
- **SVD** (15): `svd_new`, `svd_solve`, `svd_pseudo_inverse`, `svd_rank`, `singular_values`, etc.
- **QR** (12): `qr_new`, `qr_solve`, `qr_q`, `qr_r`, `qr_determinant`, etc.
- **LU** (12): `lu_new`, `lu_solve`, `lu_inverse`, `lu_determinant`, `lu_l`, `lu_u`, etc.
- **Cholesky** (10): `cholesky_new`, `cholesky_solve`, `cholesky_inverse`, `cholesky_l`, etc.
- **Schur** (8): `schur_new`, `schur_eigenvalues`, `schur_unpack`, etc.
- **Eigen** (8): `symmetric_eigen_new`, `eigenvalues`, `eigenvectors`, etc.
- **Hessenberg** (5): `hessenberg_new`, `hessenberg_unpack`, etc.

### Fragment Tools (70)

Code generation for generic dimensions and scalar types:

#### Generic Dimension Code (30)
- `emit_matrix_type` - `SMatrix<T, R, C>` or `DMatrix<T>`
- `emit_vector_type` - `SVector<T, N>` or `DVector<T>`
- `emit_function_generic_dim` - Functions with `const N: usize` parameters
- `emit_macro_invocation` - `matrix!`, `vector!` macro calls
- `emit_array_initialization` - Compile-time array literals
- And 25 more for various generic dimension patterns

#### Generic Scalar Code (20)
- `emit_function_realfield` - Functions bounded by `T: RealField`
- `emit_function_complexfield` - Functions bounded by `T: ComplexField`
- `emit_function_simdrealfield` - SIMD-aware functions
- `emit_trait_bound` - Complex trait bound expressions
- And 16 more for scalar type patterns

#### Complete Code Assembly (20)
- `emit_module_linalg` - Complete linear algebra module
- `emit_struct_with_matrices` - Structs containing matrix fields
- `emit_impl_block_matrix_ops` - Impl blocks with matrix operations
- `emit_test_suite_linalg` - Complete test suite generation
- `assemble_nalgebra_binary` - Full executable with nalgebra computations
- And 15 more for code assembly patterns

---

## Serialization Strategy

### Matrix Representation

```rust
// Fixed-size Matrix3<f64>
{
  "type": "Matrix3<f64>",
  "data": [
    [1.0, 2.0, 3.0],
    [4.0, 5.0, 6.0],
    [7.0, 8.0, 9.0]
  ]
}

// Dynamic DMatrix
{
  "type": "DMatrix<f64>",
  "nrows": 3,
  "ncols": 3,
  "data": [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]
}
```

### Vector Representation

```rust
// Fixed-size Vector3<f64>
{
  "type": "Vector3<f64>",
  "data": [1.0, 2.0, 3.0]
}

// Dynamic DVector
{
  "type": "DVector<f64>",
  "len": 3,
  "data": [1.0, 2.0, 3.0]
}
```

### Geometric Types

```rust
// Rotation3<f64> (quaternion-based)
{
  "type": "Rotation3<f64>",
  "quaternion": { "w": 1.0, "i": 0.0, "j": 0.0, "k": 0.0 }
}

// Isometry3<f64>
{
  "type": "Isometry3<f64>",
  "rotation": { "quaternion": { "w": 1.0, "i": 0.0, "j": 0.0, "k": 0.0 } },
  "translation": { "vector": [0.0, 0.0, 0.0] }
}
```

### Decomposition Results

```rust
// SVD result
{
  "type": "SVD<f64>",
  "u": { "type": "Matrix3<f64>", "data": [[...]] },
  "singular_values": { "type": "Vector3<f64>", "data": [5.0, 3.0, 1.0] },
  "v_t": { "type": "Matrix3<f64>", "data": [[...]] }
}
```

---

## Phase 1: Core Matrix/Vector (Dual-Mode)

**Goal:** Establish dual-mode pattern with Matrix and Vector operations.

### Crate Structure

```
crates/elicit_nalgebra/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── matrix.rs       # Matrix dual-mode tools
    ├── vector.rs       # Vector dual-mode tools
    ├── handles.rs      # UUID registries
    └── serde_types.rs  # JSON serialization wrappers
```

### Cargo.toml

```toml
[package]
name = "elicit_nalgebra"
version = "0.1.0"
edition = "2021"

[dependencies]
elicitation = { workspace = true }
elicitation_derive.workspace = true
nalgebra = "0.34"
serde = { workspace = true, features = ["derive"] }
serde_json.workspace = true
uuid.workspace = true
rmcp.workspace = true
tracing.workspace = true

[features]
emit = ["dep:quote", "elicitation/emit"]
```

### Dual-Mode Tool Example: Matrix Multiplication

```rust
use elicitation_derive::elicit_tool;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixMulParams {
    pub lhs: MatrixJson,
    pub rhs: MatrixJson,
}

#[elicit_tool(
    plugin = "nalgebra_matrix",
    name = "matrix_mul",
    description = "Multiply two matrices",
    emit = Auto
)]
async fn matrix_mul(p: MatrixMulParams) -> Result<CallToolResult, ErrorData> {
    // Runtime: deserialize, multiply, serialize result
    let lhs = p.lhs.to_matrix()?;
    let rhs = p.rhs.to_matrix()?;
    let result = lhs * rhs;
    let result_json = MatrixJson::from_matrix(&result);

    Ok(CallToolResult::success(json!({ "result": result_json })))
}

// Auto-generated CustomEmit impl:
impl CustomEmit<MatrixMulParams> for MatrixMulEmit {
    fn emit_code(params: &MatrixMulParams) -> TokenStream {
        let lhs_code = emit_matrix_literal(&params.lhs);
        let rhs_code = emit_matrix_literal(&params.rhs);

        quote! {
            {
                let lhs = #lhs_code;
                let rhs = #rhs_code;
                lhs * rhs
            }
        }
    }
}
```

### MatrixJson Wrapper Type

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MatrixJson {
    Matrix2 { data: [[f64; 2]; 2] },
    Matrix3 { data: [[f64; 3]; 3] },
    Matrix4 { data: [[f64; 4]; 4] },
    DMatrix { nrows: usize, ncols: usize, data: Vec<Vec<f64>> },
}

impl MatrixJson {
    pub fn to_matrix(&self) -> Result<DMatrix<f64>, String> {
        match self {
            MatrixJson::Matrix2 { data } => {
                Ok(DMatrix::from_row_slice(2, 2, &data.concat()))
            }
            MatrixJson::Matrix3 { data } => {
                Ok(DMatrix::from_row_slice(3, 3, &data.concat()))
            }
            MatrixJson::DMatrix { nrows, ncols, data } => {
                let flat: Vec<f64> = data.iter().flatten().copied().collect();
                Ok(DMatrix::from_row_slice(*nrows, *ncols, &flat))
            }
            _ => todo!(),
        }
    }

    pub fn from_matrix(m: &DMatrix<f64>) -> Self {
        let (nrows, ncols) = m.shape();
        let data: Vec<Vec<f64>> = (0..nrows)
            .map(|r| (0..ncols).map(|c| m[(r, c)]).collect())
            .collect();

        MatrixJson::DMatrix { nrows, ncols, data }
    }
}
```

### Dual-Mode Tool Example: Vector Normalization

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorNormalizeParams {
    pub vector: VectorJson,
}

#[elicit_tool(
    plugin = "nalgebra_vector",
    name = "vector_normalize",
    description = "Normalize a vector to unit length",
    emit = Auto
)]
async fn vector_normalize(p: VectorNormalizeParams) -> Result<CallToolResult, ErrorData> {
    let vec = p.vector.to_vector()?;
    let normalized = vec.normalize();
    let result_json = VectorJson::from_vector(&normalized);

    Ok(CallToolResult::success(json!({ "result": result_json })))
}

impl CustomEmit<VectorNormalizeParams> for VectorNormalizeEmit {
    fn emit_code(params: &VectorNormalizeParams) -> TokenStream {
        let vec_code = emit_vector_literal(&params.vector);

        quote! {
            {
                let v = #vec_code;
                v.normalize()
            }
        }
    }
}
```

---

## Phase 2: Geometric Types (Dual-Mode)

**Goal:** Rotations, Translations, Isometries, Transforms.

### Rotation Tools (25 dual-mode)

```rust
#[elicit_tool(
    plugin = "nalgebra_geometry",
    name = "rotation3_from_euler",
    description = "Create 3D rotation from Euler angles (roll, pitch, yaw)",
    emit = Auto
)]
async fn rotation3_from_euler(p: Rotation3EulerParams) -> Result<CallToolResult, ErrorData> {
    let rot = Rotation3::from_euler_angles(p.roll, p.pitch, p.yaw);
    let result = RotationJson::from_rotation3(&rot);

    Ok(CallToolResult::success(json!({ "rotation": result })))
}

#[elicit_tool(
    plugin = "nalgebra_geometry",
    name = "quaternion_slerp",
    description = "Spherical linear interpolation between quaternions",
    emit = Auto
)]
async fn quaternion_slerp(p: QuaternionSlerpParams) -> Result<CallToolResult, ErrorData> {
    let q1 = p.start.to_unit_quaternion()?;
    let q2 = p.end.to_unit_quaternion()?;
    let result = q1.slerp(&q2, p.t);
    let result_json = QuaternionJson::from_unit_quaternion(&result);

    Ok(CallToolResult::success(json!({ "result": result_json })))
}
```

### Transform Composition (15 dual-mode)

```rust
#[elicit_tool(
    plugin = "nalgebra_geometry",
    name = "isometry_compose",
    description = "Compose two isometries (rotation + translation)",
    emit = Auto
)]
async fn isometry_compose(p: IsometryComposeParams) -> Result<CallToolResult, ErrorData> {
    let iso1 = p.first.to_isometry3()?;
    let iso2 = p.second.to_isometry3()?;
    let composed = iso1 * iso2;
    let result = IsometryJson::from_isometry3(&composed);

    Ok(CallToolResult::success(json!({ "result": result })))
}
```

---

## Phase 3: Decompositions (Dual-Mode)

**Goal:** SVD, QR, LU, Cholesky, Eigenvalue decompositions.

### SVD Tools (15 dual-mode)

```rust
#[elicit_tool(
    plugin = "nalgebra_decomposition",
    name = "svd_compute",
    description = "Compute Singular Value Decomposition",
    emit = Auto
)]
async fn svd_compute(p: SvdParams) -> Result<CallToolResult, ErrorData> {
    let matrix = p.matrix.to_matrix()?;
    let svd = matrix.svd(p.compute_u, p.compute_v);
    let result = SvdJson::from_svd(&svd);

    Ok(CallToolResult::success(json!({ "svd": result })))
}

#[elicit_tool(
    plugin = "nalgebra_decomposition",
    name = "svd_solve",
    description = "Solve linear system using SVD",
    emit = Auto
)]
async fn svd_solve(p: SvdSolveParams) -> Result<CallToolResult, ErrorData> {
    let matrix = p.matrix.to_matrix()?;
    let b = p.b.to_vector()?;
    let svd = matrix.svd(true, true);
    let solution = svd.solve(&b, p.epsilon)?;
    let result = VectorJson::from_vector(&solution);

    Ok(CallToolResult::success(json!({ "solution": result })))
}
```

### Eigenvalue Tools (8 dual-mode)

```rust
#[elicit_tool(
    plugin = "nalgebra_decomposition",
    name = "symmetric_eigen",
    description = "Compute eigenvalues/eigenvectors of symmetric matrix",
    emit = Auto
)]
async fn symmetric_eigen(p: SymmetricEigenParams) -> Result<CallToolResult, ErrorData> {
    let matrix = p.matrix.to_matrix()?;
    let eigen = matrix.symmetric_eigen();
    let eigenvalues = VectorJson::from_vector(&eigen.eigenvalues);
    let eigenvectors = MatrixJson::from_matrix(&eigen.eigenvectors);

    Ok(CallToolResult::success(json!({
        "eigenvalues": eigenvalues,
        "eigenvectors": eigenvectors
    })))
}
```

---

## Phase 4: Fragment Tools (Generic Code Generation)

**Goal:** Generate code with generic dimensions and scalar types.

### Generic Dimension Fragments (30)

```rust
#[elicit_tool(
    plugin = "nalgebra_fragments",
    name = "emit_matrix_type",
    description = "Emit matrix type with generic dimensions",
    emit = Auto
)]
async fn emit_matrix_type(p: EmitMatrixTypeParams) -> Result<CallToolResult, ErrorData> {
    let code = match (p.nrows, p.ncols) {
        (Some(r), Some(c)) => {
            // Static dimensions
            format!("SMatrix<{}, {}, {}>", p.scalar_type, r, c)
        }
        _ => {
            // Dynamic dimensions
            format!("DMatrix<{}>", p.scalar_type)
        }
    };

    Ok(CallToolResult::success(Content::text(code)))
}

#[elicit_tool(
    plugin = "nalgebra_fragments",
    name = "emit_function_generic_dim",
    description = "Emit function with const generic dimension parameter",
    emit = Auto
)]
async fn emit_function_generic_dim(p: EmitGenericDimParams) -> Result<CallToolResult, ErrorData> {
    let code = format!(
        r#"fn {}<const N: usize>(v: SVector<f64, N>) -> SVector<f64, N> {{
    {}
}}"#,
        p.function_name,
        p.body
    );

    Ok(CallToolResult::success(Content::text(code)))
}
```

### Generic Scalar Fragments (20)

```rust
#[elicit_tool(
    plugin = "nalgebra_fragments",
    name = "emit_function_realfield",
    description = "Emit function with RealField scalar bound",
    emit = Auto
)]
async fn emit_function_realfield(p: EmitRealFieldParams) -> Result<CallToolResult, ErrorData> {
    let code = format!(
        r#"fn {}<T: RealField>(matrix: Matrix3<T>) -> T {{
    {}
}}"#,
        p.function_name,
        p.body
    );

    Ok(CallToolResult::success(Content::text(code)))
}
```

### Complete Assembly (20)

```rust
#[elicit_tool(
    plugin = "nalgebra_fragments",
    name = "assemble_nalgebra_binary",
    description = "Generate complete executable with nalgebra computations",
    emit = Auto
)]
async fn assemble_nalgebra_binary(p: AssembleParams) -> Result<CallToolResult, ErrorData> {
    let cargo_toml = generate_cargo_toml(&p);
    let main_rs = generate_main_with_computations(&p);

    Ok(CallToolResult::success(json!({
        "Cargo.toml": cargo_toml,
        "src/main.rs": main_rs,
        "description": "Complete nalgebra binary project"
    })))
}

fn generate_main_with_computations(p: &AssembleParams) -> String {
    format!(
        r#"use nalgebra::{{Matrix3, Vector3, DMatrix, DVector}};

fn main() {{
    {}
}}

{}
"#,
        p.main_body,
        p.helper_functions.join("\n\n")
    )
}
```

---

## Phase 5: UUID-Keyed Handles (Runtime-Only)

**Goal:** Persistent matrix/vector handles for stateful workflows.

### MatrixRegistry (15 runtime tools)

```rust
pub struct NalgebraPlugin {
    matrices: Arc<Mutex<HashMap<Uuid, DMatrix<f64>>>>,
    vectors: Arc<Mutex<HashMap<Uuid, DVector<f64>>>>,
    decompositions: Arc<Mutex<HashMap<Uuid, DecompositionHandle>>>,
}

#[elicit_tool(
    plugin = "nalgebra_handles",
    name = "matrix_create_handle",
    description = "Create persistent matrix handle"
)]
async fn matrix_create_handle(p: MatrixCreateParams) -> Result<CallToolResult, ErrorData> {
    let matrix = p.matrix.to_matrix()?;
    let id = Uuid::new_v4();

    let plugin = get_plugin();
    plugin.matrices.lock().unwrap().insert(id, matrix);

    Ok(CallToolResult::success(json!({ "matrix_id": id })))
}

#[elicit_tool(
    plugin = "nalgebra_handles",
    name = "matrix_get",
    description = "Retrieve matrix by handle"
)]
async fn matrix_get(p: MatrixGetParams) -> Result<CallToolResult, ErrorData> {
    let plugin = get_plugin();
    let matrices = plugin.matrices.lock().unwrap();
    let matrix = matrices.get(&p.matrix_id)
        .ok_or_else(|| ErrorData::new("Matrix not found"))?;

    let result = MatrixJson::from_matrix(matrix);
    Ok(CallToolResult::success(json!({ "matrix": result })))
}

#[elicit_tool(
    plugin = "nalgebra_handles",
    name = "matrix_compose",
    description = "Multiply two matrix handles, store result"
)]
async fn matrix_compose(p: MatrixComposeParams) -> Result<CallToolResult, ErrorData> {
    let plugin = get_plugin();
    let matrices = plugin.matrices.lock().unwrap();

    let lhs = matrices.get(&p.lhs_id).ok_or_else(|| ErrorData::new("LHS not found"))?;
    let rhs = matrices.get(&p.rhs_id).ok_or_else(|| ErrorData::new("RHS not found"))?;

    let result = lhs * rhs;
    let result_id = Uuid::new_v4();

    drop(matrices);
    plugin.matrices.lock().unwrap().insert(result_id, result);

    Ok(CallToolResult::success(json!({ "result_id": result_id })))
}
```

### DecompositionRegistry (20 runtime tools)

```rust
pub enum DecompositionHandle {
    Svd { u: DMatrix<f64>, singular_values: DVector<f64>, v_t: DMatrix<f64> },
    Qr { q: DMatrix<f64>, r: DMatrix<f64> },
    Lu { l: DMatrix<f64>, u: DMatrix<f64>, p: Vec<usize> },
    Cholesky { l: DMatrix<f64> },
}

#[elicit_tool(
    plugin = "nalgebra_handles",
    name = "decomposition_svd",
    description = "Compute and store SVD decomposition"
)]
async fn decomposition_svd(p: DecompositionSvdParams) -> Result<CallToolResult, ErrorData> {
    let plugin = get_plugin();
    let matrices = plugin.matrices.lock().unwrap();
    let matrix = matrices.get(&p.matrix_id)
        .ok_or_else(|| ErrorData::new("Matrix not found"))?;

    let svd = matrix.clone().svd(true, true);
    let handle = DecompositionHandle::Svd {
        u: svd.u.unwrap(),
        singular_values: svd.singular_values,
        v_t: svd.v_t.unwrap(),
    };

    let decomp_id = Uuid::new_v4();
    drop(matrices);
    plugin.decompositions.lock().unwrap().insert(decomp_id, handle);

    Ok(CallToolResult::success(json!({ "decomposition_id": decomp_id })))
}

#[elicit_tool(
    plugin = "nalgebra_handles",
    name = "decomposition_solve",
    description = "Solve linear system using stored decomposition"
)]
async fn decomposition_solve(p: DecompositionSolveParams) -> Result<CallToolResult, ErrorData> {
    let plugin = get_plugin();
    let decomps = plugin.decompositions.lock().unwrap();
    let vectors = plugin.vectors.lock().unwrap();

    let decomp = decomps.get(&p.decomposition_id)
        .ok_or_else(|| ErrorData::new("Decomposition not found"))?;
    let b = vectors.get(&p.b_id)
        .ok_or_else(|| ErrorData::new("Vector not found"))?;

    let solution = match decomp {
        DecompositionHandle::Svd { u, singular_values, v_t } => {
            // Reconstruct SVD and solve
            todo!()
        }
        DecompositionHandle::Lu { l, u, p } => {
            // Solve using LU
            todo!()
        }
        _ => return Err(ErrorData::new("Unsupported decomposition type")),
    };

    let solution_json = VectorJson::from_vector(&solution);
    Ok(CallToolResult::success(json!({ "solution": solution_json })))
}
```

---

## Minimal Factory Pattern (If Needed)

**When:** Only if we need to expose trait methods from `RealField` / `ComplexField`.

Most operations are already concrete methods on types, so factory pattern is rarely needed.

**Example (if required):**

```rust
pub trait RealFieldJson: Sized + 'static {
    fn type_name() -> &'static str;
    fn sqrt_json(value: &str) -> Result<String, String>;
    fn sin_json(value: &str) -> Result<String, String>;
    fn cos_json(value: &str) -> Result<String, String>;
}

impl<T> RealFieldJson for T
where
    T: RealField + Serialize + DeserializeOwned + 'static,
{
    fn sqrt_json(value: &str) -> Result<String, String> {
        let v: T = serde_json::from_str(value).map_err(|e| e.to_string())?;
        let result = v.sqrt();
        serde_json::to_string(&result).map_err(|e| e.to_string())
    }
    // ...
}

#[reflect_trait(crate::RealFieldJson)]
pub trait RealFieldJsonTools: Sized + 'static {
    fn sqrt_json(value: &str) -> Result<String, String>;
    // ...
}
```

**Verdict:** Defer factory pattern to Phase 6+ unless explicitly needed.

---

## Implementation Order

1. **Phase 1a** — Crate scaffold: `Cargo.toml`, `lib.rs`, `serde_types.rs`
2. **Phase 1b** — Core dual-mode tools: `matrix.rs` (30 tools), `vector.rs` (30 tools)
3. **Phase 1c** — `just check elicit_nalgebra`; fix compilation
4. **Phase 2a** — Geometric dual-mode tools: rotations, transforms (80 tools)
5. **Phase 2b** — `just check elicit_nalgebra`
6. **Phase 3a** — Decomposition dual-mode tools: SVD, QR, LU, Cholesky (70 tools)
7. **Phase 3b** — `just check elicit_nalgebra`
8. **Phase 4a** — Fragment tools: generic dimensions, scalar types (50 tools)
9. **Phase 4b** — `just check elicit_nalgebra`
10. **Phase 5a** — UUID-keyed handles: MatrixRegistry, DecompositionRegistry (60 tools)
11. **Phase 5b** — `just check elicit_nalgebra`
12. **Phase 6** — Wire into `elicit_server` emit chain
13. **Phase 7** — (Optional) Factory pattern for RealField/ComplexField if needed

---

## Tool Count Summary

| Category | Count | Implementation Strategy |
|----------|-------|------------------------|
| Dual-Mode Matrix Ops | 120 | `emit = Auto` + CustomEmit |
| Dual-Mode Vector Ops | 80 | `emit = Auto` + CustomEmit |
| Dual-Mode Geometry | 80 | `emit = Auto` + CustomEmit |
| Dual-Mode Decompositions | 70 | `emit = Auto` + CustomEmit |
| Fragment Tools | 70 | Code generation only |
| Runtime Handles | 60 | UUID registries |
| **Total** | **480** | |

---

## Key Advantages

1. **Natural Serialization**: Matrices/vectors map directly to JSON arrays
2. **Concrete Methods**: Most API is concrete, not trait-dispatch heavy
3. **No Macro Harvesting**: No proc-macros to wrap (unlike Leptos)
4. **Dual-Mode Dominance**: 350/480 tools (73%) are dual-mode
5. **Clear Taxonomy**: Matrix, Vector, Geometry, Decomposition categories
6. **Proven Patterns**: Same factory/fragment/dual-mode patterns from previous crates

---

## Sources

- [nalgebra - Rust](https://docs.rs/nalgebra)
- [nalgebra.org](https://nalgebra.rs/)
- [GitHub - dimforge/nalgebra](https://github.com/dimforge/nalgebra)
- [nalgebra - crates.io](https://crates.io/crates/nalgebra)
