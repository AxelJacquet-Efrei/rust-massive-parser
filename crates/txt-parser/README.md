# txt-parser

Parser texte haute performance pour très gros fichiers (1 Go+).

- Indexation partielle ou complète (stride, streaming).
- Zéro-copy (mmap), multithread, robuste UTF-8.
- API unifiée via `Document` (voir doc globale).

## Exemple rapide

```rust
use txt_parser::TxtParser;
let doc = TxtParser::parse("fichier.txt".as_ref())?;
for line in doc.lines() {
    println!("{}", line);
}
```

- Pour l’intégration multi-format et l’API commune, voir `../../INTEGRATION.md`.
