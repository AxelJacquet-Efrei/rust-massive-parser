# parser-core

Cœur du modèle de document et des traits communs à tous les parsers.

- Définit la struct `Document` (backend mmap/buffer, offsets, API lignes)
- Définit le trait `DocumentParser` et les erreurs (`ParseError`)
- Utilisé par tous les parsers (txt, json, csv, etc.)
- API unifiée, compatible batch et serveur web

Aucune logique métier ici : uniquement les abstractions partagées.
