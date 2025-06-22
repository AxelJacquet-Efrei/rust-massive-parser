# json-parser

Parser JSON haute performance pour petits et très gros fichiers, usage batch ou serveur web.

- Utilise `simd-json` (SIMD) pour parsing massif, fallback sur `serde_json`.
- **Détection automatique du format** : JSONL (une ligne = un objet) ou JSON standard (tableau/objet).
- **Stratégie optimale appliquée automatiquement** :
  - Si JSONL : parsing massivement parallèle (mmap + rayon + SIMD), ultra-rapide, scalable, robuste.
  - Si tableau/objet : parsing streaming (faible RAM, découpe chaque élément du tableau).
- Mode CLI : `json-cli fichier.json [mode|--to-jsonl]` où `[mode]` = `auto` (défaut), `simd`, `stream`, `jsonl`.
- **Option avancée** : `--to-jsonl` permet de convertir un tableau JSON en JSONL et de bénéficier du parsing massif ultra-rapide.
- API unifiée avec `Document` (voir parser-core).
- Refus explicite si le format ne correspond pas (code de sortie 2).

## ⚡️ Analyse critique des performances JSON

- **JSONL** (un objet par ligne) : parsing massivement parallèle, quasi linéaire, scalable, très rapide (idéal pour batch/web, indexation, robustesse).
- **Tableau JSON massif** : parsing streaming, plus lent car nécessite de parcourir tout le tableau séquentiellement (limite de la spec JSON, pas de découpe native).
- **Pain point** : le parsing d’un tableau JSON massif est limité par la nécessité de charger et parser séquentiellement chaque élément (spécification JSON, pas de découpe native possible sans streaming). SIMD n’apporte pas de gain majeur sur ce format.
- **Game changer** : privilégier le format JSONL pour tous les usages massifs : permet le parsing parallèle, la robustesse, la scalabilité, et la rapidité extrême. Pour les données tabulaires, exporter en JSONL plutôt qu’en tableau JSON.
- **Optimisation possible** :
  - Pour les tableaux massifs, seul le streaming (serde_json::Deserializer) permet de limiter la RAM, mais reste séquentiel.
  - Pour JSONL, continuer à exploiter rayon + SIMD (déjà optimal).
  - Toute tentative de "paralléliser" le parsing d’un tableau JSON massif nécessiterait un parser custom (non standard, fragile, peu maintenable).

## Utilisation rapide

```rust
use json_parser::JsonParser;
let doc = JsonParser::parse_as_document("fichier.json".as_ref())?;
for objet in doc.lines() {
    println!("{}", objet);
}
```

- Voir aussi `../../INTEGRATION.md` pour l'intégration multi-format.
- Benchmarks : `cargo run --release --bin bench_manual`
- Conversion + parsing massif : `json-cli fichier.json --to-jsonl`
