#!/bin/sh
echo "gchopin:$USR_PSW" | chpasswd;
chown -R gchopin:gchopin /home/gchopin;
exec "$@"
