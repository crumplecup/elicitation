# ISO Clause Map Template

Use this template to extract precise authoritative citations from the licensed
local corpus into a stable, reviewable working document before updating Rust
doc comments.

## Workflow

1. Read the clause in the local licensed artifact under this directory.
2. Record the exact clause or subclause identifier.
3. Paraphrase the governing rule narrowly and conservatively.
4. Map the rule to one or more contract symbols or evidence fields.
5. Update the Rust doc comments only after the clause map entry is complete.
6. Mark the module worksheet complete only after every contract in that module
   carries exact authoritative references.

## Entry format

| Source artifact | Clause | Narrow rule paraphrase | Contract targets | Notes |
| --- | --- | --- | --- | --- |
| `iso-8601-1-2019.*` | `§x.y.z` | | | |

## Extraction rules

- Use the exact local artifact filename in `Source artifact`.
- Prefer the narrowest clause or subclause that directly governs the contract.
- If one contract depends on several clauses, list all of them explicitly.
- If one clause governs several contracts, repeat the clause across rows rather
  than hiding the mapping.
- Do not use this file for public cross-checks. Public material belongs under
  `../public/` and remains informative only.
