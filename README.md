# frigu

## Running locally

1. Install Rust
2. Install `trunk`, `sea-orm-cli`, `cargo-edit` and `cargo-watch` with cargo
   ```bash
   cargo install trunk sea-orm-cli cargo-edit cargo-watch
   ```
3. Install WASM target with rustup
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
4. Copy `.env.example` to `.env` and edit `DATABASE_URL` accordingly.
   During development, it's easier to use the MySQL container in `docker-mysql/`.

   If using the development database, you might need to create a database on MySQL (e.g. `DATABASE CREATE frigu;`).

5. Run pending database migrations
   ```bash
   sea-orm-cli migrate up -d ./server/migration
   ```
6. Run `dev.sh` to start dev server

### Creating User Accounts

To create a user account, you can send the following request from a **loopback interface** (i.e. localhost):

```bash
curl -X POST 'http://localhost:8080/api/register' \
  -H "Content-Type: application/json" \
  -d '{"name": "John Doe", "phone_number": "912345678", "password": "secret"}'
```

## Deploying

A pre-built docker image is available at `ghcr.io/diogotcorreia/frigu`.

Here is a sample `docker-compose.yml`:

```yml
version: "3"

services:
  frigu:
    image: ghcr.io/diogotcorreia/frigu:0
    restart: unless-stopped
    ports:
      - 127.0.0.1:8080:8080
    environment:
      - DATABASE_URL=mysql://frigu:frigu@db/frigu
      - HMAC_SECRET=some-secret
      - ADMIN_SUBNET=127.0.0.1/32
    depends_on:
      - db
  db:
    image: mariadb
    restart: unless-stopped
    environment:
      MARIADB_RANDOM_ROOT_PASSWORD: yes
      MARIADB_USER: frigu
      MARIADB_PASSWORD: frigu
      MARIADB_DATABASE: frigu
    volumes:
      - frigu-data:/var/lib/mysql

volumes:
  frigu-data:
```
