# Introduction
CRUD App

# Stack
- RUST
- Postgres SQL DB
- Docker & Kubernetes

# How to run

### To run on local (Only if you have postgres running on local)

```bash
cargo run
```
### How to run using docker compose (This will set up everything including DB)
```bash
docker build . -t crud:latest && cd docker && docker compose up
```

# Kubernetes Deployment

### Deploy DB and App
```bash
cd deployment/db && 
kubectl apply -f db-namespace.yaml -f db-deployment.yaml -f db-service.yaml 
&& cd ../app && 
kubectl apply -f app-namespace.yaml -f app-deployment.yaml -f app-service.yaml
```

# Author
Sooraj Kumar