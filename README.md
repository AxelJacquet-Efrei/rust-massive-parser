# Parser massif TXT/JSON/CSV/TSV

[![CI](https://github.com/AxelJacquet-Efrei/rust-massive-parser/actions/workflows/ci.yml/badge.svg)](https://github.com/AxelJacquet-Efrei/rust-massive-parser/actions/workflows/ci.yml)
[![Clippy](https://github.com/AxelJacquet-Efrei/rust-massive-parser/actions/workflows/clippy.yml/badge.svg)](https://github.com/AxelJacquet-Efrei/rust-massive-parser/actions/workflows/clippy.yml)
[![Coverage](https://github.com/AxelJacquet-Efrei/rust-massive-parser/actions/workflows/coverage.yml/badge.svg)](https://github.com/AxelJacquet-Efrei/rust-massive-parser/actions/workflows/coverage.yml)
[![Docs](https://docs.rs/json-parser/badge.svg)](https://docs.rs/json-parser)

Parser ultra-performant pour fichiers texte, CSV/TSV et JSON (JSONL/tableau), adapté au traitement de données massives (1 Go à 100+ Go).

- **CLI auto-adaptatif** (`parser-cli`)
- **API Rust unifiée** (`Document`)
- **Performance** : mmap, multithread, streaming, faible RAM
- **Robustesse** : gestion UTF-8, erreurs explicites, tests/benchmarks

## Exemples d’utilisation

### CLI
```sh
parser-cli fichier.txt
parser-cli fichier.json
parser-cli fichier.csv
parser-cli fichier.tsv
```

### Rust
```rust
use txt_parser::TxtParser;
let doc = TxtParser::parse("fichier.txt".as_ref())?;
for line in doc.lines() {
    println!("{}", line);
}

use json_parser::JsonParser;
let doc = JsonParser::parse_as_document("fichier.json".as_ref())?;
for objet in doc.lines() {
    println!("{}", objet);
}
```

## Benchmarks
- TXT/CSV/JSONL : <1s/Go (mmap + rayon)
- JSON array : ~5-6s/Go (serde/simd)
- Conversion JSON array → JSONL : ~6-7s/Go

## Limitations connues
- Pas de support natif pour les fichiers JSON imbriqués géants (préférer JSONL pour le batch)
- La RAM dépend du mode choisi (streaming recommandé pour les très gros fichiers)
- Les conversions massives peuvent être limitées par le disque

## Générer la documentation Rust
```sh
cargo doc --open
```

## Documentation technique et intégration
Voir [INTEGRATION.md](INTEGRATION.md)

## Statut
- Stable, testé, prêt pour production et intégration serveur web/batch.

## Licence
MIT

---

Badges, CI, et publication à compléter selon vos besoins.
