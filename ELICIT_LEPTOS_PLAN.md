# elicit_leptos — Complete Reactive Web Framework Harvesting Plan

> **Completionist mandate:** Expose the entire Leptos framework (macros + runtime + SSR) as MCP tools.
> **Three-pronged approach:** Runtime tools (reactive primitives) + Fragment tools (macro wrappers + code generation) + Dual-mode tools (view composition).
> **Key insight:** Macros are thin wrappers - we expose them as fragment tools for verified workflow composition.

---

## Executive Summary

**Scope:** Leptos 0.7+ complete public API (reactive system + components + server functions + routing + SSR)
**Strategy:** Harvest 100% using Runtime + Fragment + Dual patterns
**Estimated tools:** 700-900 MCP tools
**Challenge:** Leptos is macro-heavy. Solution: Wrap every macro as a fragment tool, expose runtime primitives separately.

---

## The Three Patterns Applied to Leptos

### Pattern 1: Runtime Tools (Reactive Primitives)

**What works at runtime:**
- Signal creation and reading (create_signal, create_memo, create_effect)
- Resource fetching (create_resource)
- Context API (provide_context, use_context)
- Action dispatching (create_action)
- Router state queries (use_params, use_location)

**Example:**
```rust
#[elicit_tool(plugin = "leptos_reactive", name = "create_signal")]
async fn create_signal_tool(p: CreateSignalParams) -> Result<CallToolResult, ErrorData> {
    // Runtime: Create signal in current reactive scope
    // Returns: Signal handle (UUID-keyed)
    let signal_id = Uuid::new_v4();
    let (getter, setter) = create_signal(p.initial_value);
    SIGNALS.lock().insert(signal_id, (getter, setter));
    Ok(CallToolResult::success(json!({ "signal_id": signal_id })))
}
```

### Pattern 2: Fragment Tools (Macro Wrappers)

**What becomes fragments:**
- `#[component]` - component definition attribute
- `#[server]` - server function attribute
- `#[island]` - island component attribute
- `view!` - view macro for HTML templates
- Route definitions
- Complete app scaffolding

**Example:**
```rust
#[elicit_tool(
    plugin = "leptos_fragments",
    name = "emit_component",
    description = "Emit Leptos component with #[component] attribute",
    emit = Auto
)]
async fn emit_component(p: EmitComponentParams) -> Result<CallToolResult, ErrorData> {
    // Emits:
    // #[component]
    // pub fn ComponentName(props...) -> impl IntoView { view! { ... } }
    let code = format!(
        r#"#[component]
pub fn {}({}) -> impl IntoView {{
    {}
}}"#,
        p.name,
        generate_props_signature(&p.props),
        p.body
    );
    Ok(CallToolResult::success(Content::text(code)))
}
```

### Pattern 3: Dual-Mode Tools (View Composition)

**Operations that do both:**
- View building (create view tree at runtime, emit view! code)
- Component instantiation (render now, or emit component call)
- Event handler attachment (register runtime handler, or emit closure code)

**Example:**
```rust
#[elicit_tool(
    plugin = "leptos_view",
    name = "div",
    description = "Create div element with children",
    emit = Auto
)]
async fn view_div(p: ViewDivParams) -> Result<CallToolResult, ErrorData> {
    // Runtime: Build View node
    let view_id = create_div_view(&p.attrs, &p.children);
    Ok(CallToolResult::success(json!({ "view_id": view_id })))
}

// Auto-generated CustomEmit impl:
impl CustomEmit<ViewDivParams> for ViewDivEmit {
    fn emit_code(params: &ViewDivParams) -> TokenStream {
        let attrs = emit_attributes(&params.attrs);
        let children = emit_children(&params.children);
        quote! {
            view! {
                <div #attrs>
                    #children
                </div>
            }
        }
    }
}
```

---

## Architecture: Single Shadow Crate

### elicit_leptos

**Purpose:** Complete Leptos framework exposure
**Patterns:** All three (Runtime + Fragment + Dual)

**Module structure:**
```
crates/elicit_leptos/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── reactive/
    │   ├── mod.rs
    │   ├── signal.rs          // Signals (runtime)
    │   ├── effect.rs          // Effects (runtime)
    │   ├── memo.rs            // Memos (runtime)
    │   ├── resource.rs        // Resources (runtime)
    │   └── context.rs         // Context (runtime)
    ├── components/
    │   ├── mod.rs
    │   ├── component.rs       // #[component] wrapper (fragment)
    │   ├── props.rs           // Props types (fragment)
    │   └── children.rs        // Children handling (dual)
    ├── view/
    │   ├── mod.rs
    │   ├── elements.rs        // HTML elements (dual)
    │   ├── attributes.rs      // Attributes/props (dual)
    │   ├── events.rs          // Event handlers (dual)
    │   └── view_macro.rs      // view! wrapper (fragment)
    ├── server/
    │   ├── mod.rs
    │   ├── server_fn.rs       // #[server] wrapper (fragment)
    │   ├── actions.rs         // Actions (runtime + dual)
    │   └── resources.rs       // Server resources (runtime + dual)
    ├── routing/
    │   ├── mod.rs
    │   ├── router.rs          // Router (runtime + fragment)
    │   ├── routes.rs          // Route definitions (fragment)
    │   ├── params.rs          // use_params (runtime)
    │   └── navigate.rs        // Navigation (runtime)
    ├── ssr/
    │   ├── mod.rs
    │   ├── hydration.rs       // Hydration (runtime + fragment)
    │   └── islands.rs         // #[island] wrapper (fragment)
    └── fragments/
        ├── mod.rs
        ├── app.rs             // Complete app assembly
        ├── project.rs         // Project scaffolding
        └── templates.rs       // Common templates
```

---

## Phase 1: Reactive Primitives (Runtime)

### 1.1 Signals (Runtime Registry)

**UUID-keyed signal storage:**
```rust
pub struct LeptosReactivePlugin {
    signals: Arc<Mutex<HashMap<Uuid, (ReadSignal<String>, WriteSignal<String>)>>>,
    effects: Arc<Mutex<HashMap<Uuid, /* Effect handle */>>>,
    memos: Arc<Mutex<HashMap<Uuid, /* Memo handle */>>>,
}

#[elicit_tool(plugin = "leptos_reactive", name = "create_signal")]
async fn create_signal_tool(p: CreateSignalParams) -> Result<CallToolResult, ErrorData> {
    let (getter, setter) = create_signal(p.initial_value);
    let id = Uuid::new_v4();
    SIGNALS.lock().insert(id, (getter, setter));
    Ok(CallToolResult::success(json!({
        "signal_id": id,
        "initial_value": p.initial_value
    })))
}

#[elicit_tool(plugin = "leptos_reactive", name = "get_signal")]
async fn get_signal_tool(p: GetSignalParams) -> Result<CallToolResult, ErrorData> {
    let signals = SIGNALS.lock();
    let (getter, _) = signals.get(&p.signal_id)
        .ok_or_else(|| ErrorData::new("Signal not found"))?;

    let value = getter.get();
    Ok(CallToolResult::success(json!({ "value": value })))
}

#[elicit_tool(plugin = "leptos_reactive", name = "set_signal")]
async fn set_signal_tool(p: SetSignalParams) -> Result<CallToolResult, ErrorData> {
    let signals = SIGNALS.lock();
    let (_, setter) = signals.get(&p.signal_id)
        .ok_or_else(|| ErrorData::new("Signal not found"))?;

    setter.set(p.value.clone());
    Ok(CallToolResult::success(json!({ "new_value": p.value })))
}

#[elicit_tool(plugin = "leptos_reactive", name = "update_signal")]
async fn update_signal_tool(p: UpdateSignalParams) -> Result<CallToolResult, ErrorData> {
    // Apply function to current value
    let signals = SIGNALS.lock();
    let (getter, setter) = signals.get(&p.signal_id)
        .ok_or_else(|| ErrorData::new("Signal not found"))?;

    let current = getter.get();
    let new_value = apply_update_fn(&current, &p.update_fn);
    setter.set(new_value.clone());

    Ok(CallToolResult::success(json!({ "new_value": new_value })))
}
```

**Signal tools (~15):**
- `create_signal` - Create signal
- `get_signal` - Read signal value
- `set_signal` - Write signal value
- `update_signal` - Update with function
- `with_signal` - Run closure with value
- `track_signal` - Subscribe to changes
- `untrack_signal` - Read without tracking
- `create_rw_signal` - Read-write signal
- `signal_get_untracked` - Untracked read
- `signal_with_untracked` - Untracked with
- `create_trigger` - Trigger signal
- `trigger_notify` - Notify trigger
- `batch` - Batch updates
- `untrack` - Untrack scope
- `on_cleanup` - Register cleanup

### 1.2 Effects (Runtime)

```rust
#[elicit_tool(plugin = "leptos_reactive", name = "create_effect")]
async fn create_effect_tool(p: CreateEffectParams) -> Result<CallToolResult, ErrorData> {
    let effect_id = Uuid::new_v4();

    // Parse effect function from params
    let effect_fn = parse_effect_closure(&p.effect_body)?;

    let effect_handle = create_effect(effect_fn);
    EFFECTS.lock().insert(effect_id, effect_handle);

    Ok(CallToolResult::success(json!({ "effect_id": effect_id })))
}

#[elicit_tool(plugin = "leptos_reactive", name = "stop_effect")]
async fn stop_effect_tool(p: StopEffectParams) -> Result<CallToolResult, ErrorData> {
    let mut effects = EFFECTS.lock();
    let effect = effects.remove(&p.effect_id)
        .ok_or_else(|| ErrorData::new("Effect not found"))?;

    effect.stop();
    Ok(CallToolResult::success(json!({ "stopped": true })))
}
```

**Effect tools (~10):**
- `create_effect` - Create effect
- `create_isomorphic_effect` - SSR-compatible effect
- `create_render_effect` - Render-phase effect
- `stop_effect` - Stop effect
- `watch` - Watch specific signals
- `watch_with_options` - Watch with config
- `on` - Create conditional watcher
- `queue_microtask` - Schedule microtask
- `request_animation_frame` - Schedule RAF
- `set_timeout` - Schedule timeout

### 1.3 Memos (Runtime)

```rust
#[elicit_tool(plugin = "leptos_reactive", name = "create_memo")]
async fn create_memo_tool(p: CreateMemoParams) -> Result<CallToolResult, ErrorData> {
    let memo_id = Uuid::new_v4();

    let memo_fn = parse_memo_closure(&p.memo_body)?;
    let memo = create_memo(memo_fn);

    MEMOS.lock().insert(memo_id, memo);

    Ok(CallToolResult::success(json!({ "memo_id": memo_id })))
}

#[elicit_tool(plugin = "leptos_reactive", name = "get_memo")]
async fn get_memo_tool(p: GetMemoParams) -> Result<CallToolResult, ErrorData> {
    let memos = MEMOS.lock();
    let memo = memos.get(&p.memo_id)
        .ok_or_else(|| ErrorData::new("Memo not found"))?;

    let value = memo.get();
    Ok(CallToolResult::success(json!({ "value": value })))
}
```

**Memo tools (~5):**
- `create_memo` - Create memo
- `get_memo` - Read memo value
- `create_selector` - Create selector
- `create_selector_with_fn` - Selector with custom equality
- `with_memo` - Run closure with memo value

### 1.4 Resources (Runtime)

```rust
#[elicit_tool(plugin = "leptos_reactive", name = "create_resource")]
async fn create_resource_tool(p: CreateResourceParams) -> Result<CallToolResult, ErrorData> {
    let resource_id = Uuid::new_v4();

    // Parse fetcher function
    let fetcher = parse_fetcher_closure(&p.fetcher_body)?;

    let resource = create_resource(|| (), fetcher);
    RESOURCES.lock().insert(resource_id, resource);

    Ok(CallToolResult::success(json!({ "resource_id": resource_id })))
}

#[elicit_tool(plugin = "leptos_reactive", name = "read_resource")]
async fn read_resource_tool(p: ReadResourceParams) -> Result<CallToolResult, ErrorData> {
    let resources = RESOURCES.lock();
    let resource = resources.get(&p.resource_id)
        .ok_or_else(|| ErrorData::new("Resource not found"))?;

    let value = resource.get();
    Ok(CallToolResult::success(json!({
        "loading": value.is_none(),
        "value": value
    })))
}

#[elicit_tool(plugin = "leptos_reactive", name = "refetch_resource")]
async fn refetch_resource_tool(p: RefetchParams) -> Result<CallToolResult, ErrorData> {
    let resources = RESOURCES.lock();
    let resource = resources.get(&p.resource_id)
        .ok_or_else(|| ErrorData::new("Resource not found"))?;

    resource.refetch();
    Ok(CallToolResult::success(json!({ "refetched": true })))
}
```

**Resource tools (~10):**
- `create_resource` - Create async resource
- `create_local_resource` - Local-only resource
- `create_blocking_resource` - Blocking resource
- `read_resource` - Read current value
- `with_resource` - Run closure with value
- `refetch_resource` - Trigger refetch
- `resource_loading` - Check loading state
- `resource_error` - Get error state
- `use_transition` - Create transition
- `Suspense` boundary (fragment tool)

### 1.5 Context (Runtime)

```rust
#[elicit_tool(plugin = "leptos_reactive", name = "provide_context")]
async fn provide_context_tool(p: ProvideContextParams) -> Result<CallToolResult, ErrorData> {
    provide_context(p.value);
    Ok(CallToolResult::success(json!({ "provided": true })))
}

#[elicit_tool(plugin = "leptos_reactive", name = "use_context")]
async fn use_context_tool(p: UseContextParams) -> Result<CallToolResult, ErrorData> {
    let value: Option<String> = use_context();
    Ok(CallToolResult::success(json!({
        "found": value.is_some(),
        "value": value
    })))
}
```

**Context tools (~5):**
- `provide_context` - Provide context value
- `use_context` - Consume context value
- `expect_context` - Consume or panic
- `with_owner` - Run with specific owner
- `Owner::current` - Get current owner

**Total Reactive tools:** ~50

---

## Phase 2: Component Macros (Fragment Tools)

### 2.1 #[component] Attribute Macro Wrapper

**Fragment tool:**
```rust
#[elicit_tool(
    plugin = "leptos_components",
    name = "emit_component",
    description = "Emit #[component] attribute on function",
    emit = Auto
)]
async fn emit_component(p: EmitComponentParams) -> Result<CallToolResult, ErrorData> {
    let props_signature = p.props.iter()
        .map(|prop| format!("{}: {}", prop.name, prop.type_annotation))
        .collect::<Vec<_>>()
        .join(", ");

    let code = format!(
        r#"#[component]
pub fn {}({}) -> impl IntoView {{
    {}
}}"#,
        p.component_name,
        props_signature,
        p.body
    );

    Ok(CallToolResult::success(Content::text(code)))
}

#[elicit_tool(
    plugin = "leptos_components",
    name = "emit_component_with_generics",
    emit = Auto
)]
async fn emit_component_generic(p: EmitComponentGenericParams) -> Result<CallToolResult, ErrorData> {
    let code = format!(
        r#"#[component]
pub fn {}<{}>({}) -> impl IntoView
where
    {}
{{
    {}
}}"#,
        p.component_name,
        p.generic_params.join(", "),
        generate_props_signature(&p.props),
        p.where_clauses.join(",\n    "),
        p.body
    );

    Ok(CallToolResult::success(Content::text(code)))
}
```

**Component fragment tools (~20):**
- `emit_component` - Basic component
- `emit_component_with_generics` - Generic component
- `emit_component_with_children` - Component accepting children
- `emit_component_with_slot` - Slot-based component
- `emit_props_struct` - Props struct definition
- `emit_prop_default` - Default prop value
- `emit_prop_optional` - Optional prop
- `emit_prop_into` - Into conversion prop
- `emit_prop_strip_option` - Strip option wrapper
- `emit_transparent_prop` - Transparent prop
- `emit_builder_prop` - Builder pattern prop
- `emit_children_prop` - Children prop type
- `emit_typed_children` - Typed children
- `emit_component_ref` - Component with ref
- `emit_untracked_component` - Untracked component
- `emit_scope_component` - Component with explicit scope
- `emit_inline_component` - Inline component
- `emit_component_call` - Component instantiation
- `emit_spread_props` - Spread props syntax
- `emit_dynamic_component` - Dynamic component switching

### 2.2 Props Builder Tools

**Generate props type definitions:**
```rust
#[elicit_tool(
    plugin = "leptos_components",
    name = "emit_props_type",
    emit = Auto
)]
async fn emit_props_type(p: EmitPropsTypeParams) -> Result<CallToolResult, ErrorData> {
    let fields = p.props.iter()
        .map(|prop| {
            let attrs = generate_prop_attributes(prop);
            format!("    {}\n    pub {}: {}", attrs, prop.name, prop.prop_type)
        })
        .collect::<Vec<_>>()
        .join(",\n");

    let code = format!(
        r#"#[derive(Props, PartialEq)]
pub struct {}Props {{
{}
}}"#,
        p.component_name,
        fields
    );

    Ok(CallToolResult::success(Content::text(code)))
}
```

---

## Phase 3: View Macro (Dual-Mode)

### 3.1 HTML Elements (Dual-Mode)

**All HTML5 elements as dual-mode tools:**
```rust
#[elicit_tool(
    plugin = "leptos_view",
    name = "div",
    description = "Create div element",
    emit = Auto
)]
async fn view_div(p: ViewElementParams) -> Result<CallToolResult, ErrorData> {
    // Runtime: Build View
    let view_id = create_element_view("div", &p.attrs, &p.children);
    Ok(CallToolResult::success(json!({ "view_id": view_id })))
}

// CustomEmit generates:
impl CustomEmit<ViewElementParams> for ViewDivEmit {
    fn emit_code(params: &ViewElementParams) -> TokenStream {
        let attrs = emit_attrs(&params.attrs);
        let children = emit_children(&params.children);
        quote! {
            view! {
                <div #attrs>
                    #children
                </div>
            }
        }
    }
}
```

**HTML element tools (~140 elements):**

**Document structure:** html, head, title, base, link, meta, style, body
**Sections:** article, section, nav, aside, h1-h6, header, footer, address
**Grouping:** p, hr, pre, blockquote, ol, ul, li, dl, dt, dd, figure, figcaption, main, div
**Text:** a, em, strong, small, s, cite, q, dfn, abbr, data, time, code, var, samp, kbd, sub, sup, i, b, u, mark, ruby, rt, rp, bdi, bdo, span, br, wbr
**Edits:** ins, del
**Embedded:** picture, source, img, iframe, embed, object, param, video, audio, track, map, area, svg (+ all SVG elements)
**Tables:** table, caption, colgroup, col, tbody, thead, tfoot, tr, td, th
**Forms:** form, label, input, button, select, datalist, optgroup, option, textarea, output, progress, meter, fieldset, legend
**Interactive:** details, summary, dialog
**Scripting:** script, noscript, template, slot, canvas

### 3.2 Attributes (Dual-Mode)

```rust
#[elicit_tool(
    plugin = "leptos_view",
    name = "attr_class",
    description = "Add class attribute",
    emit = Auto
)]
async fn attr_class(p: AttrClassParams) -> Result<CallToolResult, ErrorData> {
    // Runtime or emit
    Ok(CallToolResult::success(json!({
        "attr": "class",
        "value": p.classes
    })))
}

#[elicit_tool(
    plugin = "leptos_view",
    name = "attr_class_list",
    description = "Dynamic class list with conditions",
    emit = Auto
)]
async fn attr_class_list(p: AttrClassListParams) -> Result<CallToolResult, ErrorData> {
    // Runtime: Evaluate conditions and build class string
    // Emit: Generate class:list=[(class, condition), ...] syntax
    Ok(CallToolResult::success(json!({
        "attr": "class:list",
        "items": p.class_conditions
    })))
}

#[elicit_tool(
    plugin = "leptos_view",
    name = "attr_style",
    description = "Add style attribute",
    emit = Auto
)]
async fn attr_style(p: AttrStyleParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(json!({
        "attr": "style",
        "value": p.styles
    })))
}
```

**Attribute tools (~50):**
- Global attributes: `class`, `id`, `style`, `title`, `lang`, `dir`, `tabindex`, `accesskey`, `contenteditable`, `draggable`, `hidden`, `spellcheck`, `translate`
- `class:` directive - conditional class
- `style:` directive - conditional style
- `prop:` directive - property binding
- `attr:` directive - attribute binding
- `on:` directive - event handler
- `use:` directive - directive application
- `node_ref` - element reference
- `_ref` - ref binding
- Data attributes: `data-*`
- ARIA attributes: `aria-*` (30+ attributes)

### 3.3 Event Handlers (Dual-Mode)

```rust
#[elicit_tool(
    plugin = "leptos_view",
    name = "on_click",
    description = "Attach click event handler",
    emit = Auto
)]
async fn on_click(p: OnClickParams) -> Result<CallToolResult, ErrorData> {
    // Runtime: Register handler
    // Emit: Generate on:click= syntax
    Ok(CallToolResult::success(json!({
        "event": "click",
        "handler": p.handler_body
    })))
}
```

**Event handler tools (~50):**

**Mouse events:** click, dblclick, mousedown, mouseup, mousemove, mouseenter, mouseleave, mouseover, mouseout, contextmenu, wheel
**Keyboard events:** keydown, keyup, keypress
**Focus events:** focus, blur, focusin, focusout
**Form events:** submit, change, input, invalid, reset, select
**Drag events:** drag, dragstart, dragend, dragenter, dragleave, dragover, drop
**Clipboard events:** copy, cut, paste
**Composition events:** compositionstart, compositionupdate, compositionend
**Touch events:** touchstart, touchmove, touchend, touchcancel
**Pointer events:** pointerdown, pointerup, pointermove, pointerenter, pointerleave, pointerover, pointerout, pointercancel
**Animation events:** animationstart, animationend, animationiteration
**Transition events:** transitionstart, transitionend, transitionrun, transitioncancel
**Other:** scroll, resize, load, error, abort

### 3.4 View Macro Wrapper (Fragment)

```rust
#[elicit_tool(
    plugin = "leptos_view",
    name = "emit_view",
    description = "Emit view! macro invocation",
    emit = Auto
)]
async fn emit_view(p: EmitViewParams) -> Result<CallToolResult, ErrorData> {
    let code = format!(
        r#"view! {{
    {}
}}"#,
        p.view_body
    );

    Ok(CallToolResult::success(Content::text(code)))
}

#[elicit_tool(
    plugin = "leptos_view",
    name = "emit_fragment",
    description = "Emit view fragment (<>...</>)",
    emit = Auto
)]
async fn emit_fragment(p: EmitFragmentParams) -> Result<CallToolResult, ErrorData> {
    let children = p.children.join("\n    ");
    let code = format!(
        r#"view! {{
    <>
        {}
    </>
}}"#,
        children
    );

    Ok(CallToolResult::success(Content::text(code)))
}

#[elicit_tool(
    plugin = "leptos_view",
    name = "emit_show",
    description = "Emit <Show> component for conditional rendering",
    emit = Auto
)]
async fn emit_show(p: EmitShowParams) -> Result<CallToolResult, ErrorData> {
    let code = format!(
        r#"view! {{
    <Show
        when=move || {}
        fallback=|| view! {{ {} }}
    >
        {}
    </Show>
}}"#,
        p.condition,
        p.fallback.unwrap_or_default(),
        p.children
    );

    Ok(CallToolResult::success(Content::text(code)))
}

#[elicit_tool(
    plugin = "leptos_view",
    name = "emit_for",
    description = "Emit <For> component for list rendering",
    emit = Auto
)]
async fn emit_for(p: EmitForParams) -> Result<CallToolResult, ErrorData> {
    let code = format!(
        r#"view! {{
    <For
        each=move || {}
        key=|item| {}
        children=|item| view! {{
            {}
        }}
    />
}}"#,
        p.items_expr,
        p.key_fn,
        p.children_template
    );

    Ok(CallToolResult::success(Content::text(code)))
}
```

**View composition tools (~30):**
- `emit_view` - view! macro wrapper
- `emit_fragment` - Fragment wrapper
- `emit_show` - Conditional rendering
- `emit_for` - List rendering
- `emit_each` - Alternative list syntax
- `emit_suspense` - Suspense boundary
- `emit_transition` - Transition wrapper
- `emit_error_boundary` - Error boundary
- `emit_portal` - Portal to element
- `emit_dynamic_tag` - Dynamic element
- `emit_inner_html` - Raw HTML
- `emit_text_node` - Text interpolation
- `emit_raw_html` - Dangerous HTML
- Component embedding (20+ control flow components)

**Total View tools:** ~270

---

## Phase 4: Server Functions (Fragment Tools)

### 4.1 #[server] Attribute Macro Wrapper

```rust
#[elicit_tool(
    plugin = "leptos_server",
    name = "emit_server_function",
    description = "Emit #[server] attribute on async function",
    emit = Auto
)]
async fn emit_server_fn(p: EmitServerFnParams) -> Result<CallToolResult, ErrorData> {
    let params_sig = p.params.iter()
        .map(|param| format!("{}: {}", param.name, param.param_type))
        .collect::<Vec<_>>()
        .join(", ");

    let code = format!(
        r#"#[server({}, "{}")]
pub async fn {}({}) -> Result<{}, ServerFnError> {{
    {}
}}"#,
        p.server_fn_name,
        p.endpoint.unwrap_or_default(),
        p.function_name,
        params_sig,
        p.return_type,
        p.body
    );

    Ok(CallToolResult::success(Content::text(code)))
}

#[elicit_tool(
    plugin = "leptos_server",
    name = "emit_server_with_encoding",
    emit = Auto
)]
async fn emit_server_with_encoding(p: EmitServerEncodingParams) -> Result<CallToolResult, ErrorData> {
    let code = format!(
        r#"#[server(
    {},
    endpoint = "{}",
    encoding = "{}",
    {}
)]
pub async fn {}({}) -> Result<{}, ServerFnError> {{
    {}
}}"#,
        p.server_fn_name,
        p.endpoint,
        p.encoding, // "Url", "Cbor", "GetJson", "PostJson", "Rkyv", "StreamingText"
        generate_server_options(&p.options),
        p.function_name,
        generate_params_sig(&p.params),
        p.return_type,
        p.body
    );

    Ok(CallToolResult::success(Content::text(code)))
}
```

**Server function tools (~15):**
- `emit_server_function` - Basic #[server]
- `emit_server_with_encoding` - Custom encoding
- `emit_server_with_middleware` - Middleware chain
- `emit_server_with_prefix` - Custom prefix
- `emit_server_streaming` - Streaming response
- `emit_server_multipart` - Multipart form
- `emit_server_cbor` - CBOR encoding
- `emit_server_rkyv` - Rkyv encoding
- `emit_server_get_json` - GET with JSON
- `emit_server_post_json` - POST with JSON
- `emit_server_url_encoded` - URL encoded
- `emit_server_custom_client` - Custom client
- `emit_server_custom_error` - Custom error type
- `emit_call_server_fn` - Client-side call
- `emit_create_server_action` - Action creation

### 4.2 Actions (Runtime + Dual)

```rust
#[elicit_tool(
    plugin = "leptos_server",
    name = "create_action",
    description = "Create server action",
    emit = Auto
)]
async fn create_action_tool(p: CreateActionParams) -> Result<CallToolResult, ErrorData> {
    // Runtime: Register action
    let action_id = Uuid::new_v4();
    let action = create_action(parse_action_fn(&p.action_body)?);
    ACTIONS.lock().insert(action_id, action);

    Ok(CallToolResult::success(json!({ "action_id": action_id })))
}

#[elicit_tool(
    plugin = "leptos_server",
    name = "dispatch_action",
    emit = Auto
)]
async fn dispatch_action_tool(p: DispatchActionParams) -> Result<CallToolResult, ErrorData> {
    let actions = ACTIONS.lock();
    let action = actions.get(&p.action_id)
        .ok_or_else(|| ErrorData::new("Action not found"))?;

    action.dispatch(p.input);
    Ok(CallToolResult::success(json!({ "dispatched": true })))
}
```

**Action tools (~15):**
- `create_action` - Create action
- `create_server_action` - Server action
- `create_multi_action` - Multi action
- `create_server_multi_action` - Server multi action
- `dispatch_action` - Dispatch action
- `action_input` - Get action input
- `action_value` - Get action value
- `action_version` - Get action version
- `action_pending` - Check if pending
- `use_action` - Use existing action
- `use_server_action` - Use server action
- `with_action_value` - Run with value
- `action_set_pending` - Set pending state
- `action_cancel` - Cancel action
- `action_clear` - Clear action state

---

## Phase 5: Routing (Runtime + Fragment)

### 5.1 Router (Runtime + Fragment)

**Runtime tools:**
```rust
#[elicit_tool(plugin = "leptos_routing", name = "use_location")]
async fn use_location_tool() -> Result<CallToolResult, ErrorData> {
    let location = use_location();
    Ok(CallToolResult::success(json!({
        "pathname": location.pathname.get(),
        "search": location.search.get(),
        "hash": location.hash.get(),
        "state": location.state.get()
    })))
}

#[elicit_tool(plugin = "leptos_routing", name = "use_navigate")]
async fn use_navigate_tool(p: NavigateParams) -> Result<CallToolResult, ErrorData> {
    let navigate = use_navigate();
    navigate(&p.path, Default::default());
    Ok(CallToolResult::success(json!({ "navigated": true })))
}

#[elicit_tool(plugin = "leptos_routing", name = "use_params")]
async fn use_params_tool() -> Result<CallToolResult, ErrorData> {
    let params = use_params_map();
    let params_map: HashMap<String, String> = params.get()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    Ok(CallToolResult::success(json!({ "params": params_map })))
}

#[elicit_tool(plugin = "leptos_routing", name = "use_query")]
async fn use_query_tool() -> Result<CallToolResult, ErrorData> {
    let query = use_query_map();
    let query_map: HashMap<String, String> = query.get()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    Ok(CallToolResult::success(json!({ "query": query_map })))
}
```

**Fragment tools:**
```rust
#[elicit_tool(
    plugin = "leptos_routing",
    name = "emit_router",
    description = "Emit Router component",
    emit = Auto
)]
async fn emit_router(p: EmitRouterParams) -> Result<CallToolResult, ErrorData> {
    let code = format!(
        r#"view! {{
    <Router>
        {}
    </Router>
}}"#,
        p.routes
    );

    Ok(CallToolResult::success(Content::text(code)))
}

#[elicit_tool(
    plugin = "leptos_routing",
    name = "emit_routes",
    description = "Emit Routes component with route definitions",
    emit = Auto
)]
async fn emit_routes(p: EmitRoutesParams) -> Result<CallToolResult, ErrorData> {
    let routes = p.routes.iter()
        .map(|route| emit_route_definition(route))
        .collect::<Vec<_>>()
        .join("\n            ");

    let code = format!(
        r#"view! {{
    <Routes>
        {}
    </Routes>
}}"#,
        routes
    );

    Ok(CallToolResult::success(Content::text(code)))
}

#[elicit_tool(
    plugin = "leptos_routing",
    name = "emit_route",
    description = "Emit single Route definition",
    emit = Auto
)]
async fn emit_route(p: EmitRouteParams) -> Result<CallToolResult, ErrorData> {
    let code = format!(
        r#"<Route path="{}" view=|| view! {{ <{}/> }} />"#,
        p.path,
        p.component
    );

    Ok(CallToolResult::success(Content::text(code)))
}

#[elicit_tool(
    plugin = "leptos_routing",
    name = "emit_protected_route",
    description = "Emit route with auth protection",
    emit = Auto
)]
async fn emit_protected_route(p: EmitProtectedRouteParams) -> Result<CallToolResult, ErrorData> {
    let code = format!(
        r#"<ProtectedRoute
    path="{}"
    view=|| view! {{ <{}/> }}
    condition=move || {}
    redirect_path="{}"
/>"#,
        p.path,
        p.component,
        p.auth_check,
        p.redirect_to
    );

    Ok(CallToolResult::success(Content::text(code)))
}

#[elicit_tool(
    plugin = "leptos_routing",
    name = "emit_link",
    description = "Emit <A> link component",
    emit = Auto
)]
async fn emit_link(p: EmitLinkParams) -> Result<CallToolResult, ErrorData> {
    let code = format!(
        r#"<A href="{}">{}</A>"#,
        p.href,
        p.children
    );

    Ok(CallToolResult::success(Content::text(code)))
}
```

**Routing tools (~25):**

Runtime:
- `use_location` - Get current location
- `use_navigate` - Navigate function
- `use_params` - Route params
- `use_params_map` - Params map
- `use_query` - Query params
- `use_query_map` - Query map
- `use_route` - Current route
- `use_resolved_path` - Resolve path
- `use_is_routing` - Routing state
- `use_router` - Router instance

Fragment:
- `emit_router` - Router component
- `emit_routes` - Routes component
- `emit_route` - Single route
- `emit_nested_route` - Nested route
- `emit_parent_route` - Parent route
- `emit_protected_route` - Protected route
- `emit_redirect` - Redirect
- `emit_link` - Link component
- `emit_outlet` - Outlet component
- `emit_route_definitions` - Route macro
- `emit_static_route` - Static route
- `emit_dynamic_route` - Dynamic route
- `emit_catch_all_route` - Catch-all route
- `emit_hash_router` - Hash router
- `emit_static_router` - Static router (SSR)

---

## Phase 6: SSR & Islands (Fragment Tools)

### 6.1 #[island] Attribute Macro Wrapper

```rust
#[elicit_tool(
    plugin = "leptos_ssr",
    name = "emit_island",
    description = "Emit #[island] attribute for partial hydration",
    emit = Auto
)]
async fn emit_island(p: EmitIslandParams) -> Result<CallToolResult, ErrorData> {
    let code = format!(
        r#"#[island]
pub fn {}({}) -> impl IntoView {{
    {}
}}"#,
        p.component_name,
        generate_props_signature(&p.props),
        p.body
    );

    Ok(CallToolResult::success(Content::text(code)))
}
```

**SSR/Islands tools (~15):**
- `emit_island` - Island component
- `emit_island_with_ssr` - SSR+Island hybrid
- `emit_hydrate` - Client hydration entry
- `emit_ssr_render` - Server render entry
- `emit_leptos_routes` - SSR routes
- `emit_generate_route_list` - Route list generation
- `emit_ssr_modes` - SSR mode config
- `emit_out_of_order_streaming` - Streaming SSR
- `emit_in_order_streaming` - Ordered streaming
- `emit_async_rendering` - Async SSR
- `emit_ssr_context` - SSR context setup
- `emit_extract_meta` - Meta extraction
- `emit_hydration_scripts` - Hydration scripts
- `emit_islands_router` - Islands router
- `emit_server_fn_handler` - Server fn handler

---

## Phase 7: Complete App Assembly (Fragment Tools)

### 7.1 Project Scaffolding

```rust
#[elicit_tool(
    plugin = "leptos_fragments",
    name = "assemble_leptos_app",
    description = "Generate complete Leptos application",
    emit = Auto
)]
async fn assemble_app(p: AssembleAppParams) -> Result<CallToolResult, ErrorData> {
    let cargo_toml = generate_cargo_toml(&p);
    let main_rs = generate_main_rs(&p);
    let lib_rs = generate_lib_rs(&p);
    let app_rs = generate_app_component(&p);
    let index_html = generate_index_html(&p);
    let config_toml = generate_leptos_config(&p);

    Ok(CallToolResult::success(json!({
        "Cargo.toml": cargo_toml,
        "src/main.rs": main_rs,
        "src/lib.rs": lib_rs,
        "src/app.rs": app_rs,
        "index.html": index_html,
        "Leptos.toml": config_toml,
        "components": generate_component_files(&p.components)
    })))
}

fn generate_main_rs(p: &AssembleAppParams) -> String {
    format!(r#"
use leptos::*;
use {package_name}::App;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {{
    use axum::{{Router, routing::get}};
    use leptos_axum::{{LeptosRoutes, generate_route_list}};

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(leptos_axum::file_and_error_handler(App))
        .with_state(leptos_options);

    println!("Listening on http://{{}}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}}

#[cfg(not(feature = "ssr"))]
pub fn main() {{
    use leptos::*;
    mount_to_body(App);
}}
"#, package_name = p.package_name)
}

fn generate_app_component(p: &AssembleAppParams) -> String {
    format!(r#"
use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {{
    view! {{
        <Router>
            <Routes>
                {}
            </Routes>
        </Router>
    }}
}}
"#, generate_route_list(&p.routes))
}
```

**App assembly tools (~10):**
- `assemble_leptos_app` - Complete app
- `emit_cargo_toml` - Cargo.toml
- `emit_leptos_config` - Leptos.toml
- `emit_main_entry` - main.rs
- `emit_lib_entry` - lib.rs
- `emit_app_root` - App component
- `emit_index_html` - index.html
- `emit_ssr_setup` - SSR server
- `emit_client_setup` - Client entry
- `emit_workspace_config` - Workspace setup

---

## Estimated Tool Count

| Category | Runtime | Dual-Mode | Fragment | Total |
|---|---|---|---|---|
| **Reactive Primitives** | 50 | 0 | 0 | 50 |
| **Component Macros** | 0 | 0 | 20 | 20 |
| **View Elements** | 0 | 140 | 0 | 140 |
| **View Attributes** | 0 | 50 | 0 | 50 |
| **View Events** | 0 | 50 | 0 | 50 |
| **View Composition** | 0 | 0 | 30 | 30 |
| **Server Functions** | 0 | 15 | 15 | 30 |
| **Routing** | 10 | 0 | 15 | 25 |
| **SSR/Islands** | 0 | 0 | 15 | 15 |
| **App Assembly** | 0 | 0 | 10 | 10 |
| **Control Flow** | 0 | 30 | 0 | 30 |
| **Directives** | 0 | 20 | 0 | 20 |
| **Meta/SEO** | 0 | 15 | 5 | 20 |
| **Integration** | 10 | 10 | 10 | 30 |
| **Total** | **70** | **330** | **120** | **520** |

**Breakdown:**
- Runtime tools: Reactive primitives, routing queries, actions
- Dual-mode tools: View elements, attributes, events (runtime + emit)
- Fragment tools: Macro wrappers, complete app assembly

---

## Implementation Timeline

**Week 1:** Reactive primitives (signals, effects, memos)
**Week 2:** Reactive resources, context, actions
**Week 3:** Component macros (#[component], props)
**Week 4:** HTML elements (dual-mode, 140 tools)
**Week 5:** Attributes and events (dual-mode, 100 tools)
**Week 6:** View composition and control flow
**Week 7:** Server functions (#[server], actions)
**Week 8:** Routing (runtime + fragment)
**Week 9:** SSR/Islands (#[island], hydration)
**Week 10:** App assembly + integration testing

**Total:** 10 weeks for complete implementation

---

## Success Criteria

1. ✅ All Leptos attribute macros wrapped as fragment tools
2. ✅ All reactive primitives exposed as runtime tools
3. ✅ All HTML5 elements are dual-mode (runtime + emit)
4. ✅ Complete app assembly generates working Leptos project
5. ✅ view! macro composition works (nested elements, attributes, events)
6. ✅ Server functions compile and execute correctly
7. ✅ Routing works in both CSR and SSR modes
8. ✅ Islands hydration generates correct code
9. ✅ All 520 tools registered and tested
10. ✅ Comprehensive documentation with 30+ example apps

---

## Key Innovations

1. **Macro Harvesting:** First shadow crate to systematically wrap attribute macros as fragment tools
2. **Dual-Mode View Building:** HTML elements work at runtime (DOM) AND as code emission (view! syntax)
3. **Triple Harvest:** Runtime reactivity + Fragment macros + Dual-mode views = 100% framework coverage
4. **Attribute Macro Pattern:** Thin wrappers around #[component], #[server], #[island] for verified composition
5. **Complete App Assembly:** From components → routes → server → client → working deployed app
6. **Zero Compromise:** Every macro, every element, every attribute, every event - all exposed
