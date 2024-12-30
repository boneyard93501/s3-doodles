# MinIO Distributed Cluster

Note to self for setting up a 4-node distributed [MinIO](https://github.com/minio/minio) cluster on my mbp.

## Prerequisites**

Make sure you have the following installed:

- Docker: [Install Docker Desktop](https://www.docker.com/products/docker-desktop/)
- Docker Compose: Included with Docker Desktop
- MinIO Client (`mc`)**: ` brew install minio/stable/mc`


## Setup

See [docker-compose.yaml](./docker-compose.yaml) and the corresponding [.env](./.env) file.


Start the cluster with:
```bash
docker-compose --env-file .env up -d
```

Verify containers are running:
```bash
docker ps
```

Access MinIO Web Console on the following URLs:
- Node 1: [http://localhost:9004](http://localhost:9004)
- Node 2: [http://localhost:9005](http://localhost:9005)
- Node 3: [http://localhost:9006](http://localhost:9006)
- Node 4: [http://localhost:9008](http://localhost:9008)


Credentials, from .env, e.g.:
Username: `adminUser`
Password: `securePassword123`


Set up MinIO client (`mc`):
```bash
mc alias set myminio http://localhost:9001 adminUser securePassword123
```

Check cluster health:
```bash
mc admin info myminio
```


Create a Bucket wth `mc` client:
```bash
mc mb myminio/test-bucket
```

Upload a File:
```bash
mc cp ./testfile.txt myminio/test-bucket/
```

List Files:
```bash
mc ls myminio/test-bucket
```

Simulate Node Failure:
```bash
docker stop minio1
```

Verify access is maintained:
```bash
mc ls myminio/test-bucket
```

Restart node:
```bash
docker start minio1
```

Clean Up -- Stop and remove the cluster:
```bash
docker-compose down -v
```

Dashboard:
* http://localhost:9004/browser

Rust:
* See a [sample rust client](./minio-rs/).