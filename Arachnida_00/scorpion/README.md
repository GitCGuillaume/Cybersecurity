# Scorpion

Ce projet consiste à afficher les métadonnées d'un fichier et les données EXIF si disponibles.

Format exif supportés:
   -  JPEG
   -  PNG
   -  TIFF


```bash
make
make download

./scorpion no_exif/wikipedia.png 
File: no_exif/wikipedia.png
Content-type/Mime: image/png
Width: 100 Height: 100
File length: 13444 bytes
Permission: 100644
Last time modified: 2025-03-21 00:38:00 +01:00
Last time accessed: 2025-04-29 12:01:24 +02:00
Time creation: 2025-04-29 12:00:22 +02:00
Exif error: No Exif data found in PNG


./scorpion exif/Solmeta_Geotagger_N2_Kompass_GPS_on_Nikon_D300.jpg 
File: exif/Solmeta_Geotagger_N2_Kompass_GPS_on_Nikon_D300.jpg
Content-type/Mime: image/jpeg
Width: 1695 Height: 1582
File length: 1726858 bytes
Permission: 100644
Last time modified: 2013-11-17 11:31:54 +01:00
Last time accessed: 2025-04-29 12:01:48 +02:00
Time creation: 2025-04-29 12:00:24 +02:00
ImageDescription: "OLYMPUS DIGITAL CAMERA         "
Make: "OLYMPUS CORPORATION"
Model: "C5060WZ"
[...]
```

Compatible Linux et Windows 10
