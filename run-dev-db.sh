#!/usr/bin/env bash

CONTAINER_NAME="tracectrl-db"
ADMINER_CONTAINER_NAME="${CONTAINER_NAME}-adminer"
DB_USER="tracectrl"
DB_NAME="tracectrl"

if [ ! "$(command -v docker)" ]; then
  echo "command \"docker\" doesn't exist on system"
  exit 1
fi

if [ ! -f ".dev-ps-pass" ]; then
  if [ ! "$(command -v openssl)" ]; then
    echo "command \"openssl\" doesn't exist on system"
    exit 1
  fi

  password=$(openssl rand -hex 32)
  echo "New DB password: $password - MAKE SURE TO *NOT* USE THIS IN PRODUCTION, IT IS PURELY FOR DEVELOPMENT PURPOSES."

  echo "export DB_PASS=$password" >./.dev-ps-pass
fi

# shellcheck source=/dev/null
source ./.dev-ps-pass

if [[ -z "${DB_PASS}" ]]; then
  echo "empty DB_PASS, check ./.dev-ps-pass"
  exit 1
fi

CONNECTION_STRING="postgresql://${DB_USER}:${DB_PASS}@localhost:5432/${DB_NAME}"

if docker container ls -a | grep -q -E "${CONTAINER_NAME}"; then
  read -r -p "Docker container with name '${CONTAINER_NAME}' already exists, delete? (y, N) > " -n1 answer
  echo

  case $answer in
  "y" | "Y")
    docker container rm -f "${CONTAINER_NAME}"
    echo "Removed ${CONTAINER_NAME} container."
    ;;
  *) ;;
  esac
fi

if docker container ls -a | grep -q -E "${ADMINER_CONTAINER_NAME}"; then
  read -r -p "Docker container with name '${ADMINER_CONTAINER_NAME}' already exists, delete? (y, N) > " -n1 answer
  echo

  case $answer in
  "y" | "Y")
    docker container rm -f "${ADMINER_CONTAINER_NAME}"
    echo "Removed ${ADMINER_CONTAINER_NAME} container."
    ;;
  *) ;;
  esac
fi

if docker run --name "$CONTAINER_NAME" \
  -p "5432:5432" \
  -e POSTGRES_PASSWORD="$DB_PASS" \
  -e POSTGRES_USER="$DB_USER" \
  -e POSTGRES_DB="$DB_NAME" \
  -d postgres; then
  echo "PostgresDB started..."
  echo "Connection string: ${CONNECTION_STRING}"
  echo "Stop container with 'docker stop ${CONTAINER_NAME}'"
else
  echo "Could not start Postgres database: please see above for errors, if present."
  exit 1
fi

if docker run --name "$ADMINER_CONTAINER_NAME" \
  -p "8080:8080" \
  --link "$CONTAINER_NAME":db \
  -e ADMINER_DEFAULT_SERVER=db \
  -d adminer; then
  echo "Adminer started..."
  echo "Started on http://localhost:8080"
  echo "Stop container with 'docker stop ${ADMINER_CONTAINER_NAME}'"
else
  echo "Could not start Adminer front-end: please see above for errors, if present."
  exit 1
fi
