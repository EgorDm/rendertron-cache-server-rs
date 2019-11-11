#!/bin/sh
sudo docker-compose up -d
sudo docker run -ti --network="host" --rm alpine/bombardier -c 200 -d 10s -l "http://172.18.16.200:5000/render/www.google.com/"
sudo docker-compose down
