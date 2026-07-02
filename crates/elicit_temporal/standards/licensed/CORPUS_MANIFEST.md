# ISO Corpus Manifest

This manifest records the saved clause-bearing ISO working copies currently used
for `elicit_temporal` citation tightening.

| Standard | ISO product | Local files | Provenance | Fit for use | Limits |
| --- | --- | --- | --- | --- | --- |
| `ISO 8601-1:2019` | `70907` | `iso-8601-1-2019.sample.pdf`, `iso-8601-1-2019.sample.txt` | public iTeh sample-preview PDF mirror for the ISO catalog product | searchable TOC, searchable clause numbering, body text for the opening normative clauses | preview, not full standard text |
| `ISO 8601-1:2019/Amd 1:2022` | `81801` | `iso-8601-1-2019-amd1-2022.sample.pdf`, `iso-8601-1-2019-amd1-2022.sample.txt` | public iTeh sample-preview PDF mirror for the ISO amendment catalog product | searchable amendment item numbering and replacement text | preview, not full amended base text |
| `ISO 8601-2:2019` | `70908` | `iso-8601-2-2019.sample.pdf`, `iso-8601-2-2019.sample.txt` | public iTeh sample-preview PDF mirror for the ISO catalog product | searchable TOC, searchable clause numbering, body text for the opening normative clauses | preview, not full standard text |

Working rule:

- cite exact ISO clause numbers only when the saved preview corpus exposes the
  clause identifier with enough local context to avoid guessing
- keep informative public mirrors in doc comments when they sharpen meaning for
  clauses whose full body text is truncated in the preview
