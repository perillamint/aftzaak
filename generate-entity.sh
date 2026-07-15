podman run -d --name aftzaak-db \
  -e POSTGRES_USER=aftzaak \
  -e POSTGRES_PASSWORD=aftzaak \
  -e POSTGRES_DB=aftzaak \
  -p 5432:5432 \
  docker.io/library/postgres:17

for i in $(seq 1 30); do
  if podman exec aftzaak-db pg_isready -U aftzaak -d aftzaak | grep -q "accepting connections"; then
    echo "DB READY after ${i}s"
    break
  fi
  sleep 1
done

export DATABASE_URL="postgres://aftzaak:aftzaak@localhost:5432/aftzaak"
sea-orm-cli migrate up

sea-orm-cli generate entity \
  -u "$DATABASE_URL" \
  -o src/entity \
  --with-serde both

podman rm -f aftzaak-db
