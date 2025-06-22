# Parser massif TXT/JSON

Parser ultra-performant pour fichiers texte et JSON (JSONL/tableau), adapté au traitement de données massives (1 Go à 100+ Go).

- **CLI auto-adaptatif** (`parser-cli`)
- **API Rust unifiée** (`Document`)
- **Performance** : mmap, multithread, streaming, faible RAM
- **Robustesse** : gestion UTF-8, erreurs explicites, tests/benchmarks

## Documentation technique et intégration
Voir [INTEGRATION.md](INTEGRATION.md)

## Exemples d’utilisation
- Parsing CLI : `parser-cli fichier.txt` ou `parser-cli fichier.json`
- Parsing Rust : voir l’exemple dans chaque crate ou dans `INTEGRATION.md`

## Statut
- Stable, testé, prêt pour production et intégration serveur web/batch.

## Licence
MIT

---

Badges, CI, et publication à compléter selon vos besoins.
