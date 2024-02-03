# mongodb (in docker image) with Rust

> docker pull mongodb/mongodb-community-server

> docker run -d -p 27017:27017 --name test-mongo mongodb/mongodb-community-server:latest

> export MONGODB_URI='mongodb://localhost:27017'

> cargo run
```
client uri="mongodb://localhost:27017"
Databases:
- admin
- config
- local
```

Online (MongoDB Atlas)

> export MONGODB_URI='mongodb+srv://<user>:<password>@cluster0.p8ivwpr.mongodb.net/?retryWrites=true&w=majority

To run into a docker machine:

docker build -t europe-west1-docker.pkg.dev/peak-stream-408711/cloud-run-source-deploy/mongo_qs:v1

> docker run -it -e "MONGODB_URI=mongodb+srv://johndoe:HONrtzSpSZE0WSMF@cluster0.p8ivwpr.mongodb.net/" -t europe-west1-docker.pkg.dev/peak-stream-408711/cloud-run-source-deploy/mongo_qs:v1 ./mongo_q
