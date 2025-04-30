# ft_otp

Projet qui consiste à générer un code à 6 chiffres à usage unique sous le langage Rust.

Liste de spécifications techniques:
- RFC [HOTP](https://www.ietf.org/rfc/rfc4226.txt)
- RFC [TOTP](https://datatracker.ietf.org/doc/html/rfc6238)

## Options
```bash
# -g : Le programme reçoit en argument une clé publique hexadécimal d'une taille de 64 caractères,
#        il encryptera une clé privée dans un fichier ft_otp.key
./ft_otp -g key_example.hex

# -k : Le programme générera et affichera un mot de passe temporaire en fonction du fichier ft_otp.key
./ft_otp -k ft_otp.key
```

```bash
./ft_otp -g key_example.hex

./ft_otp -k ft_otp.key
```

Exemple de génération et comparaison entre ft_otp et l'outil Oathtool
![Capture d’écran du 2025-04-29 13-59-24](https://github.com/user-attachments/assets/0c7a444e-270d-4ee5-b871-559f434c055c)
