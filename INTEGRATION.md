# Intégration du parser massif TXT/CSV/TSV/JSON

Ce workspace fournit :
- Des binaires CLI spécialisés (`txt-cli`, `csv-cli`, `json-cli`) pour chaque format.
- Des crates Rust modulaires : `parser-core`, `txt-parser`, `json-parser`, `csv-parser`.

## Utilisation CLI

```sh
# Fichier texte
./target/release/txt-cli fichier.txt
# Fichier JSON (JSONL ou tableau)
./target/release/json-cli fichier.json
# Fichier CSV
./target/release/csv-cli fichier.csv
# Fichier TSV
./target/release/csv-cli fichier.tsv
```
- Refus explicite si le format ne correspond pas (code de sortie 2)
- Parsing silencieux, ultra-rapide, prêt pour usage batch ou serveur web

## Utilisation en Rust (librairie)

Ajoutez dans votre `Cargo.toml` :
```toml
[dependencies]
parser-core = { path = "crates/parser-core" }
txt-parser = { path = "crates/txt-parser" }
json-parser = { path = "crates/json-parser" }
csv-parser = { path = "crates/csv-parser" }
```

### Exemple TXT
```rust
use txt_parser::TxtParser;
let doc = TxtParser::parse("fichier.txt".as_ref())?;
for line in doc.lines() {
    println!("{}", line);
}
```

### Exemple JSON (JSONL ou tableau)
```rust
use json_parser::JsonParser;
let doc = JsonParser::parse_as_document("fichier.json".as_ref())?;
for objet in doc.lines() {
    println!("{}", objet); // chaque objet JSON sérialisé sur une ligne
}
```

### Exemple CSV/TSV
```rust
use csv_parser::CsvParser;
let doc = CsvParser::parse("fichier.csv".as_ref())?;
for ligne in doc.lines() {
    println!("{}", ligne); // chaque ligne CSV/TSV brute
}
```

- L’API `Document` est unifiée : accès rapide, zéro-copy, multithread, faible RAM.
- Voir aussi les README/INTEGRATION.md de chaque crate pour les détails spécifiques.

## Tests & Benchmarks

- `cargo test -p txt-parser` / `cargo test -p json-parser` / `cargo test -p csv-parser`
- `cargo bench -p json-parser` / `cargo bench -p csv-parser`

## Intégration serveur web

- Utilisez l’API `Document` dans vos handlers (actix, axum, etc.) pour traiter de très gros fichiers efficacement.

---

Pour plus d’exemples, voir les dossiers `crates/*/tests/` et `bench.rs`.
