# elicit_csv

MCP shadow crate for [`csv`](https://crates.io/crates/csv) 1.x — BurntSushi's fast CSV
reader/writer library with Serde support.

## Overview

Exposes the full `csv` API surface as MCP tools via `CsvPlugin`, a `StatefulPlugin` that
holds live `ReaderBuilder`, `WriterBuilder`, `Reader`, and `Writer` instances keyed by UUID.

## Tool Prefixes

| Prefix | Description |
|--------|-------------|
| `csv__reader_builder__*` | `ReaderBuilder` configuration methods |
| `csv__reader__*` | `Reader<R>` instance methods |
| `csv__writer_builder__*` | `WriterBuilder` configuration methods |
| `csv__writer__*` | `Writer<W>` instance methods |
| `csv__string_record__*` | Stateless `StringRecord` helpers |
| `csv__byte_record__*` | Stateless `ByteRecord` helpers |
| `csv__position__*` | `Position` construction helpers |
| `csv__invalid_option__*` | `invalid_option` Serde helper snippet |
| `csv__result__*` | `csv::Result<T>` documentation |

## Typical Workflow

```
1. csv__reader_builder__new           → builder_id
2. csv__reader_builder__delimiter     (optional configuration)
3. csv__reader_builder__has_headers   (optional configuration)
4. csv__reader_builder__from_reader   → reader_id
5. csv__reader__headers               → ["col1", "col2", ...]
6. csv__reader__all_records           → [["val1", "val2"], ...]
7. csv__reader__close
```
