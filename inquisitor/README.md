# Inquisitor

L'installation de libpcap peut être nécessaire.

Ce projet est pour l'éducation uniquement.

Inquisitor est un programme Man-in-the-middle attack qui consiste à empoisonner la table de protocole de résolution d’adresse des cibles en se faisant passer pour la gateway.

```bash
make all
./inquisitor <ip_src> <MAC_src> <ip_target> <MAC_target> <optional_interface_name>
```
Adresse source du client.

![Capture d’écran du 2025-04-29 16-40-49](https://github.com/user-attachments/assets/4a3f3986-4b5d-4222-87a1-7dabcb2bb2b0)


Cible du client originel.

![Capture d’écran du 2025-04-29 16-41-15](https://github.com/user-attachments/assets/ea1d8be4-67af-451a-aa26-0b430dd840d8)

Empoisonnement du client et de la cible.

![Capture d’écran du 2025-04-29 16-41-53](https://github.com/user-attachments/assets/8cd7abfc-eee8-4152-9f4e-734527910da8)

Le client pense maintenant qu'il doit passer par l'attaqueur.

![Capture d’écran du 2025-04-29 16-43-27](https://github.com/user-attachments/assets/e8715a81-1206-4ec0-b0aa-2c8a6c271cce)

La cible pense maintenant que les paquets passent par le pirate.
![Capture d’écran du 2025-04-29 16-43-43](https://github.com/user-attachments/assets/5466d1c0-c3df-4ded-b8d9-2050934d5a51)


Le programme peut maintenant voir le nom des fichiers qui transitent entre la cible et le client.

Ici nous recevons 2 packets, celui du client et de la cible avec le nom du fichier qui transite.

![Capture d’écran du 2025-04-29 16-44-37](https://github.com/user-attachments/assets/1bac9253-080b-441e-9b67-59b3b3f620d3)

Lorsque le programme se ferme, il restore les tables ARP du client et de la cible.
![Capture d’écran du 2025-04-29 16-45-29](https://github.com/user-attachments/assets/aa3938b7-231f-4871-8a46-d5757c597468)

![Capture d’écran du 2025-04-29 16-45-45](https://github.com/user-attachments/assets/456f6f46-0493-482d-94e7-3abbd34bb0d9)
