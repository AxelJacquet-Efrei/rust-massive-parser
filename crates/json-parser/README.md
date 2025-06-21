# json-parser

Un parser JSON haute performance pour petits et très gros fichiers, adapté à un usage serveur web ou batch.

- Utilise `serde_json` pour les petits fichiers (chargement complet en mémoire).
- Utilise un parsing streaming (ligne à ligne pour JSONL, ou incrémental) pour les très gros fichiers.
- API similaire à `txt-parser` pour faciliter l'intégration.
- Compatible avec le trait commun de `parser-core`.

# Documentation spécifique JSON

Voir la documentation d’intégration générale à la racine (`../../INTEGRATION.md`).

## Utilisation rapide

```rust
use json_parser::JsonParser;
let doc = JsonParser::parse_as_document("fichier.json".as_ref())?;
for objet in doc.lines() {
    println!("{}", objet);
}
```

- Support JSONL et tableaux massifs.
- API unifiée avec le parser texte.
- Benchmarks : `cargo bench -p json-parser`
