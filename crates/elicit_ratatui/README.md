# elicit_ratatui

Dual-mode MCP tools for building terminal user interfaces with
[ratatui](https://ratatui.rs).

Each tool operates in two modes:

1. **Runtime mode** — returns a JSON widget description that can be rendered
   by a ratatui terminal backend.
2. **Emit mode** — generates idiomatic ratatui Rust code via the elicitation
   code-emission pipeline.

## Features

| Feature   | Description                              |
|-----------|------------------------------------------|
| `emit`    | Enable code generation (`quote` + emit)  |
| `runtime` | Enable terminal backend integration      |

## Tool Categories

- **Widgets** — Block, Paragraph, List, Table, Gauge, Sparkline, Tabs, Clear
- **Style** — Foreground/background colour, text modifiers, named/RGB/indexed colours
- **Layout** *(coming soon)* — Vertical/horizontal splits, constraints

## Quick Start

```toml
[dependencies]
elicit_ratatui = { version = "0.9.1", features = ["emit"] }
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE)
or [MIT license](LICENSE-MIT) at your option.
