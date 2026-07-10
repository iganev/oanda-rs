# Development tools

Helper scripts used to derive the hand-written models in `src/models/` from
the OpenAPI spec in `schema/json/v20v3.json`. Useful when the spec submodule
is updated and models need to be diffed/refreshed.

- `schemadump.py def <Name>...` — compact dump of schema definitions
  (fields, types, required, descriptions); `op <path-substring>` dumps
  operations; `list` lists all definitions.
- `gen_model.py <Name>...` — prints Rust struct / `string_enum!` code for
  the given definitions (docs, serde renames, `Option`/`Vec` policy). The
  output is a starting point: review and adjust before committing.

The fixture generator embedded in the test suite's history builds
`tests/fixtures/*.json` from the same spec (one maximally-populated object
per union variant).
