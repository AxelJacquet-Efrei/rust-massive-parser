# csv-parser

Parser CSV/TSV haute performance pour fichiers massifs (batch, web, data science).

- Zéro-copy (mmap ou buffer)
- Indexation parallèle (rayon)
- Détection automatique du séparateur (virgule ou tabulation)
- API unifiée (Document)
- Pagination, stride, validation UTF-8 optionnelle
- Refus explicite si le format ne correspond pas (code de sortie 2)

Voir `../../INTEGRATION.md` pour l'intégration et des exemples d'usage.
