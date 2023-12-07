sudo docker pull ghcr.io/basicprogrammer10/connorcode:master
sudo docker tag ghcr.io/basicprogrammer10/connorcode:master connorcode
sudo docker compose up --force-recreate -d
sudo docker image prune -a -f
