# parser-cli

CLI auto-adaptatif pour parser des fichiers texte ou JSON massifs.

- Détecte automatiquement le format (txt/json).
- Affiche des métriques détaillées (temps, mémoire, mode).
- Ne tente pas d’afficher le contenu des très gros fichiers.
- Utilise les parsers du workspace via l’API unifiée `Document`.

Pour l’intégration Rust ou l’API, voir la doc globale à la racine.
