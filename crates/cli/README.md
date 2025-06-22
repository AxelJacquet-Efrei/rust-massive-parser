# parser-cli

CLI spécialisé pour parsing massif TXT, CSV/TSV, JSON/JSONL.

- Utilisez le binaire dédié pour chaque format :
  - `txt-cli fichier.txt`
  - `csv-cli fichier.csv` ou `csv-cli fichier.tsv`
  - `json-cli fichier.json` ou `json-cli fichier.jsonl`
- Refus explicite si le format ne correspond pas (code de sortie 2).
- Parsing silencieux, ultra-rapide, prêt pour usage batch ou serveur web.
- Code de sortie : 0 = OK, 1 = erreur parsing, 2 = mauvais format.

Pour l’intégration Rust ou l’API, voir la doc globale à la racine et `INTEGRATION.md`.
