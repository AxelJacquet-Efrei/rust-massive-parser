# Parser massif TXT/JSON/CSV/TSV

[![CI](https://github.com/AxelJacquet-Efrei/rust-massive-parser/actions/workflows/ci.yml/badge.svg)](https://github.com/AxelJacquet-Efrei/rust-massive-parser/actions/workflows/ci.yml)
[![Clippy](https://github.com/AxelJacquet-Efrei/rust-massive-parser/actions/workflows/clippy.yml/badge.svg)](https://github.com/AxelJacquet-Efrei/rust-massive-parser/actions/workflows/clippy.yml)
[![Coverage](https://github.com/AxelJacquet-Efrei/rust-massive-parser/actions/workflows/coverage.yml/badge.svg)](https://github.com/AxelJacquet-Efrei/rust-massive-parser/actions/workflows/coverage.yml)
[![Docs](https://docs.rs/json-parser/badge.svg)](https://docs.rs/json-parser)
[![dependency status](https://deps.rs/repo/github/AxelJacquet-Efrei/rust-massive-parser/status.svg)](https://deps.rs/repo/github/AxelJacquet-Efrei/rust-massive-parser)

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

## Robustesse & Qualité
- Tous les parsers (TXT, CSV, JSON/JSONL) sont testés sur : succès, vide, fichier non trouvé, UTF-8 malformé (n'importe où), et erreurs de format.
- Les tests sont stricts : chaque ligne/record est vérifiée sans tolérance sur les retours à la ligne ou l'encodage.
- CI GitHub Actions : build, tests, clippy, couverture.
- Hook git pré-push : `cargo fmt`, `cargo clippy`, `cargo test` auto avant chaque push.
- Les gros fichiers de test (`fichier_1GB.*`) ne sont pas versionnés (voir `.gitignore`).

## Structure du projet
- `crates/cli` : binaire principal auto-adaptatif (TXT/CSV/JSON/JSONL)
- `crates/txt-parser`, `crates/csv-parser`, `crates/json-parser` : crates spécialisées
- `crates/parser-core` : cœur commun (Document, erreurs, mmap, etc.)
- `benches/`, `tests/` dans chaque crate

## Contribution
- Merci de lancer `cargo fmt`, `cargo clippy` et `cargo test` avant tout commit/push (automatisé par le hook).
- Benchmarks : voir dossiers `benches/` et instructions dans chaque crate.
