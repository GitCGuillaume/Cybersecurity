Le programme Spider en langage Rust navigue depuis une URL et télécharge toutes les images trouvées.

## Options
```bash
./spider https://demo.cyotek.com/html/elements/img.php


# Téléchargement récursif
./spider -r https://demo.cyotek.com/html/elements/img.php


# Téléchargement récursif avec profondeur
./spider -r -l 1 https://demo.cyotek.com/html/elements/img.php


# Nom du dossier parent où seront stockées les images
./spider -p exemple -r -l 1 https://demo.cyotek.com/html/elements/img.php
```

