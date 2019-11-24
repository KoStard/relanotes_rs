-- Your SQL goes here
CREATE TABLE "nodes" (
  "id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,
  "linked_to_id" integer,
  "type_id" integer NOT NULL,
  "name" text NOT NULL,
  "description" text,
  "subgroup_id" integer NOT NULL,
  FOREIGN KEY ("linked_to_id") REFERENCES "nodes" ("id")
    ON DELETE RESTRICT,
  FOREIGN KEY ("type_id") REFERENCES "node_types" ("id"),
  FOREIGN KEY ("subgroup_id") REFERENCES "subgroups" ("id")
    ON DELETE CASCADE
--  UNIQUE ("linked_to_id", "name", "subgroup_id")
);