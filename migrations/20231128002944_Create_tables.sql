-- Add migration script here

-- https://www.postgresqltutorial.com/postgresql-tutorial/postgresql-foreign-key/

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";


-- TODO: create backtrace and snippet tables

CREATE TABLE "snippet" (
  "id" SERIAL PRIMARY KEY,
  "line" int NOT NULL DEFAULT 0,
  "code" text NOT NULL
);

CREATE TABLE "backtrace_snippet" (
  "backtrace_id" int,
  "snippet_id" int,
  "amount" int NOT NULL DEFAULT 1,
  PRIMARY KEY ("backtrace_id", "snippet_id")
);

CREATE TABLE "backtrace" (
  "id" SERIAL PRIMARY KEY
);

CREATE TABLE "logs" (
  "id" uuid UNIQUE PRIMARY KEY DEFAULT (uuid_generate_v4()),
  "message" text NOT NULL,
  "language" text NOT NULL,
  "snippet" int NOT NULL,
  "backtrace" int NOT NULL,
  "warnings" text[] NOT NULL,
  "date" timestamp NOT NULL DEFAULT (now())
);

ALTER TABLE "backtrace_snippet" ADD FOREIGN KEY ("backtrace_id") REFERENCES "backtrace" ("id") ON DELETE CASCADE ON UPDATE CASCADE;

ALTER TABLE "backtrace_snippet" ADD FOREIGN KEY ("snippet_id") REFERENCES "snippet" ("id") ON DELETE CASCADE ON UPDATE CASCADE;

ALTER TABLE "logs" ADD FOREIGN KEY ("snippet") REFERENCES "snippet" ("id");

ALTER TABLE "logs" ADD FOREIGN KEY ("backtrace") REFERENCES "backtrace" ("id");

