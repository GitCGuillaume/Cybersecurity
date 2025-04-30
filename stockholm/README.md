# Stockholm

Projet Rust Linux à éducation uniquement, ne pas utiliser dans un environnement réel ou à des fins malicieux.

Stockholm est un projet malware basé sur [Wannacry](https://fr.wikipedia.org/wiki/WannaCry) qui infectera tous les fichiers comportant les mêmes extensions que le malware WannaCry.

Cependant pour ce projet, seul le dossier "infection" dans le dossier utilisateur sera infecté.

Le chiffrement est basé sur une clé de 16 caractères via la méthode : [AES 128](https://datatracker.ietf.org/doc/html/rfc3826)

## Compilation
```bash
# Compilation
make all

# Nettoyage
make fclean
```

### Liste de commandes
```bash
./stockholm [16 CARACTÈRES]
./stockholm --reverse [16 CARACTÈRES]

./stockholm --help
-h or --help pour montrer une aide de commande.
-v or --version pour afficher la version actuelle
-s or --silent pour ne produire aucune sortie
-r or --reverse suivi de la clé pour inverser l'infection.
```

État initial

![initial](https://github.com/user-attachments/assets/dbc13146-7df0-4b40-9f67-52cd55767f47)

État infécté

![infected](https://github.com/user-attachments/assets/0f9559c7-aa19-4c1e-9f35-0d62b085dafa)
