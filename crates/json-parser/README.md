# json-parser

Parser JSON haute performance pour petits et très gros fichiers, usage batch ou serveur web.

- Utilise `simd-json` (SIMD) pour parsing massif, fallback sur `serde_json`.
- Détection automatique JSONL (une ligne = un objet) ou JSON standard (tableau/objet).
- Mode CLI : `json-cli fichier.json [mode]` où `[mode]` = `auto` (défaut), `simd`, `stream`, `jsonl`.
- Parsing JSONL massivement parallèle (rayon + SIMD).
- API unifiée avec `Document` (voir parser-core).
- Refus explicite si le format ne correspond pas (code de sortie 2).

## Utilisation rapide

```rust
use json_parser::JsonParser;
let doc = JsonParser::parse_as_document("fichier.json".as_ref())?;
for objet in doc.lines() {
    println!("{}", objet);
}
```

- Voir aussi `../../INTEGRATION.md` pour l'intégration multi-format.
- Benchmarks : `cargo bench -p json-parser`
