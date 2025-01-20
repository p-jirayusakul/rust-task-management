# Task Management

โจทย์ฝึกจาก https://outline.devlab.dev/s/31dadbce-607a-4a62-9276-43551e1b81f5

## :sparkles: Feature
1. Create/Update/Delete/Read Task, master data
2. Specific Priority
3. Auth with JWT

## :notebook: Document
- [database design](https://dbdiagram.io/d/Task-Management-67078fc497a66db9a3831449)
- [collection postman](https://documenter.getpostman.com/view/6440353/2sAXxV6W24)
- collection postman file ใน root project ไฟล์ชื่อว่า [task management.postman_collection.json](https://gitlab.devlab.dev/xsifi/learing/backend/task_phatthakarn_jir/-/blob/main/task%20management-%20local%20auth.postman_collection.json)
- swagger [http://localhost:4000/api/v1/swagger](http://localhost:4000/api/v1/swagger)
## :boy: Username for testing
| Username| Password | Role | Ability |
| :---------------- | :------: | :----: | :---- |
| admin        |   Vrz!3Xn8   | Admin | everyting |
| manager           |   9QhrLw.C   | Manager | Create/Update/Read - but can not delete |
| member1    |  V78imwx*   | Member | Create/Update/Read - its own |
| member2 |  Zr7_94.K   | Member | Create/Update/Read - its own |

## Setup local environment

### Install tools

- [Docker desktop](https://www.docker.com/products/docker-desktop)
- [Rust](https://www.rust-lang.org)
- [Homebrew](https://brew.sh/)

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
### :computer: Setup infrastructure

- สร้างไฟล์ `.env.local` ที่ root project

    ``` bash
    DB_HOST=localhost
    DB_PORT=5432
    DB_DATABASE=task_management
    DB_USERNAME=postgres
    DB_PASSWORD=1234
    DB_SCHEMA=public
    
    ENV=localhost
    API_PORT=4000
    ALLOW_ORIGINS=http://localhost:3000
    JWT_SECRET=xxxxxxxxxxxxxxxxx
    JWT_EXPIRE_MILLISECOND=28800000
    ```

- #### ถ้ายังไม่เคย init schema มี 2 วิธี:
  ##### วิธีแรก copy query ในไฟล์ `internal/database/migrations/000001_init_schema.up.sql` ไปรันใน server database ของตัวเอง
  ##### หรือ รัน migrate ด้วย go lib
- Install [Migrate](https://github.com/golang-migrate/migrate/tree/master/cmd/migrate)

    ```bash
    brew install golang-migrate
    ```
- รัน migrate:

    ```bash
    migrate -path ./internal/database/migrations -database "postgres://${DB_USERNAME}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_DATABASE}?sslmode=disable&search_path=${DB_SCHEMA}" up
    ```
### Run in localhost
หลังจาก setup ทุกอย่างแล้ว
- Run API Server:

    ```bash
    cargo run
    ```

### Run with docker localhost
- Run docker compose:

    ```bash
    docker compose up
    ```
- Link resource:
- [API] http://localhost:4000/api/v1
- [Database] db-postgres
- [PG4Admin] http://localhost:5050 (username: user@domain.com, password: 1234)