# Chess 2.0 - Server
Project using Rust and gRPC for a Chess server.


## Build db docker
```sh
docker run --name postgres -d -p 5432:5432 -e POSTGRES_PASSWORD=admin postgres
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

## Database schema

https://dbdocs.io/marcel.canhisares/Chessbicos