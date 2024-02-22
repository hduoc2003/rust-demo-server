<img src="https://opensource.com/sites/default/files/lead-images/rust_programming_crab_sea.png" align="right"
     alt="Size Limit logo by Anton Lovchikov" width="198" height="112">
# A small Todo-app backend written in Rust using Diesel and Axum
## How to run
First, you need to setup some environment variable files. Below is names and some examples of them.
### .env.production
```
HOST=0.0.0.0
PORT=1204
DATABASE=postgres
DATABASE_URL=postgres://username:password@db:5432/database_name
JWT_KEY=secret
JWT_EXPIRED_TIME=86400
RUST_LOG=DEBUG
```
### postgres.env
```
POSTGRES_USER=username
POSTGRES_PASSWORD=password
POSTGRES_DB=database_name
```
And then, in the root of project, run ``docker-compose up -d``. Now you can use ``127.0.0.1:1204`` as your backend server.
## Project structure

### [migrations](./migrations/)<img src="https://cdn-icons-png.flaticon.com/512/9243/9243391.png" align="left" alt="Size Limit logo by Anton Lovchikov" width="20" height="20" style="margin-right: 10px;">

This folder contains some migration files which auto-generated by [diesel_cli](https://crates.io/crates/diesel_cli).

### [src](./src/)<img src="https://cdn-icons-png.flaticon.com/512/9005/9005715.png" align="left" alt="Size Limit logo by Anton Lovchikov" width="20" height="20" style="margin-right: 10px;">
- **[api/v1](./src/api/v1)**
  - **[controllers](./src/api/v1/controllers/)**: handler for each api.
  - **[db](./src/api/v1/db/)**: code to handle database (such as switch between DBMS).
  - **[middlewares](./src/api/v1/middlewares/)**: put your middlewares here.
  - **[models](./src/api/v1/models/)**: store your database table structure here.
  - **[routes](./src/api/v1/routes/)**: for routing your api endpoints.
  - **[tests](./src/api/v1/tests/)**: do some unit testing here.
  - **[types](./src/api/v1/types/)**: everything related to types (Trait, Struct, ...).
  - **[utils](./src/api/v1/utils/)**: some helper functions.
- **[config](./src/config/)**: Initialize server configuration.
### [postman_collections](./postman_collections/)<img src="https://www.svgrepo.com/show/354202/postman-icon.svg" align="left" alt="Size Limit logo by Anton Lovchikov" width="20" height="20" style="margin-right: 10px;">
All API endpoints are here.

