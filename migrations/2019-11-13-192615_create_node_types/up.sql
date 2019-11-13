-- Your SQL goes here
CREATE TABLE "node_type" (
    "id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,
    "name" text NOT NULL,
    "value" text NOT NULL
);

-- Regular
-- Child
-- Note

INSERT INTO "node_type" ("name", "value") 
VALUES ("Regular", "regular"),
("Child", "child"),
("Note", "note");