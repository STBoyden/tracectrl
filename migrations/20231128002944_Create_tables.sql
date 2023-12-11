-- Add migration script here

-- https://www.postgresqltutorial.com/postgresql-tutorial/postgresql-foreign-key/

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";


-- TODO: create backtrace and snippet tables

CREATE TABLE "snippets" (
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

CREATE TABLE "backtraces" (
  "id" SERIAL PRIMARY KEY
);

CREATE TABLE "logs" (
  "id" uuid UNIQUE PRIMARY KEY DEFAULT (uuid_generate_v4()),
  "message" text NOT NULL,
  "language" text NOT NULL,
  "snippet_id" int NOT NULL,
  "backtrace_id" int NOT NULL,
  "warnings" text[] NOT NULL,
  "date" timestamp NOT NULL DEFAULT (now())
);

ALTER TABLE "backtrace_snippet" ADD FOREIGN KEY ("backtrace_id") REFERENCES "backtraces" ("id") ON DELETE CASCADE ON UPDATE CASCADE;

ALTER TABLE "backtrace_snippet" ADD FOREIGN KEY ("snippet_id") REFERENCES "snippets" ("id") ON DELETE CASCADE ON UPDATE CASCADE;

ALTER TABLE "logs" ADD FOREIGN KEY ("snippet_id") REFERENCES "snippets" ("id");

ALTER TABLE "logs" ADD FOREIGN KEY ("backtrace_id") REFERENCES "backtraces" ("id");

