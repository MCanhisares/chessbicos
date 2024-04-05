## Build db docker
```sh
docker run --network=host --name postgres -e POSTGRES_PASSWORD=admin -d postgres
```

## Manually connect to DB
```sh
docker exec -it postgres bash
psql -U postgres
```

## Build project docker
```sh
docker build -t chessbicos .
docker run -it --network=host --name chessbicos chessbicos
```

## Clean-up Docker
```sh
docker kill postgres
docker container rm postgres
docker kill chessbicos
docker image rm -f chessbicos
docker container rm chessbicos
```