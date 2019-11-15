-- Your SQL goes here
CREATE TABLE "node_types" (
    "id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,
    "name" text NOT NULL,
    "value" text NOT NULL
);

-- Regular
-- Child
-- Note

INSERT INTO "node_types" ("name", "value") 
VALUES ("Regular", "regular"),
("Child", "child"),
("Note", "note");