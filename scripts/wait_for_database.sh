#!/bin/bash
cd ..
docker-compose up -d
until pg_isready -h localhost -p 5432 -U username
#until docker run -it postgres -e POSTGRES_PASSWORD=password --add-host host.docker.internal:host- gateway docker.io/postgres:14-alpine -h localhost -U username pg_isready
do
  echo "Waiting for postgres"
sleep 2; done
echo "docker is now running"
#docker-compose down
exit