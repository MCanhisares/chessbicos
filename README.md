## Build db docker
```sh
docker build -t sqlite .
docker run -d --network=host --name sqlite sqlite
```

## Build project docker
```sh
docker build -t chessbicos .
docker run -it --network=host --name chessbicos chessbicos
```

## Clean-up Docker

```sh
docker image rm -f sqlite
docker image rm -f chessbicos
docker container rm sqlite
docker container rm chessbicos
```