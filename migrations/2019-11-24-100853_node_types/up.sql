-- Your SQL goes here
create table "node_types" (
    "id" integer not null primary key autoincrement,
    "name" text not null,
    "value" text not null unique
);