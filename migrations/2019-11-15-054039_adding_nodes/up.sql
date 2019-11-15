-- Your SQL goes here
CREATE TABLE "nodes" (
  "id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,
  "linked_to_id" integer,
  "type_id" integer NOT NULL,
  "name" text,
  "description" text,
  "group_id" integer NOT NULL,
  FOREIGN KEY ("linked_to_id") REFERENCES "nodes" ("id")
    ON DELETE RESTRICT,
  FOREIGN KEY ("type_id") REFERENCES "node_types" ("id"),
  FOREIGN KEY ("group_id") REFERENCES "groups" ("id")
    ON DELETE CASCADE
);