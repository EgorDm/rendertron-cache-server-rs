#!/bin/sh
sudo docker-compose up -d
d run --network host --rm skandyla/wrk -t12 -c400 -d30s http://172.18.16.200:5000/render/http://www.google.com/
sudo docker-compose down
