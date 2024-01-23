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
