-- Add migration script here

CREATE TABLE "clients_logs" (
  "client_id" int,
  "log_id" uuid,
  "amount" int NOT NULL DEFAULT 1,
  PRIMARY KEY ("client_id", "log_id")
);

CREATE TABLE "clients" (
  "id" SERIAL PRIMARY KEY,
  "last_connected" timestamp NOT NULL DEFAULT (now()),
  "logs_sent" int NOT NULL DEFAULT 0
);

ALTER TABLE "clients_logs" ADD FOREIGN KEY ("client_id") REFERENCES "clients" ("id") ON DELETE CASCADE ON UPDATE CASCADE;

ALTER TABLE "clients_logs" ADD FOREIGN KEY ("log_id") REFERENCES "logs" ("id") ON DELETE CASCADE ON UPDATE CASCADE;
