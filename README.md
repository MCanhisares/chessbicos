## Build db docker
```sh
docker build -t sqlite ./src/db
docker run -d -v sqlite_data:/data --network=host --name sqlite sqlite

## Build project docker
```sh
docker build -t chessbicos .
docker run -it --network=host --name chessbicos chessbicos
```

## Clean-up Docker

```sh
docker kill sqlite
docker image rm -f sqlite
docker container rm sqlite
docker kill chessbicos
docker image rm -f chessbicos
docker container rm chessbicos
```