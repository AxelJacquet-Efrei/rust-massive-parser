# Parser massif TXT/CSV/TSV/JSON

<!-- Badges CI & Qualité -->
[![CI](https://github.com/AxelJacquet-Efrei/rust-massive-parser/actions/workflows/ci.yml/badge.svg)](https://github.com/AxelJacquet-Efrei/rust-massive-parser/actions/workflows/ci.yml)
[![Docs.rs](https://docs.rs/parser-core/badge.svg)](https://docs.rs/parser-core)
[![Clippy](https://github.com/AxelJacquet-Efrei/rust-massive-parser/actions/workflows/clippy.yml/badge.svg)](https://github.com/AxelJacquet-Efrei/rust-massive-parser/actions/workflows/clippy.yml)
[![dependency status](https://deps.rs/repo/github/AxelJacquet-Efrei/rust-massive-parser/status.svg)](https://deps.rs/repo/github/AxelJacquet-Efrei/rust-massive-parser)

Parsers ultra-performants pour fichiers texte, CSV/TSV, et JSON (JSONL/tableau), adaptés au traitement de données massives (1 Go à 100+ Go).

- **Binaires CLI spécialisés** : `txt-cli`, `csv-cli`, `json-cli` (usage : `txt-cli fichier.txt`, etc.)
- **API Rust unifiée** (`Document`)
- **Performance** : mmap, multithread, SIMD, streaming, faible RAM
- **Robustesse** : gestion UTF-8, refus explicite sur mauvais format, codes de sortie normalisés
- **Tests/benchmarks** : pipeline automatisé, mesures reproductibles

## Documentation technique et intégration
Voir [INTEGRATION.md](INTEGRATION.md)

## Exemples d’utilisation
- Parsing CLI : `txt-cli fichier.txt`, `csv-cli fichier.csv`, `json-cli fichier.json`
- Parsing Rust : voir l’exemple dans chaque crate ou dans `INTEGRATION.md`

## Statut
- Stable, testé, prêt pour production et intégration serveur web/batch.

## Licence
MIT

---

Badges, CI, et publication à compléter selon vos besoins.
