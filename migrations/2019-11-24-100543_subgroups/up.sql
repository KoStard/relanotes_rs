-- Your SQL goes here

create table "subgroups" (
    "id" integer not null primary key autoincrement,
    "group_id" integer not null,
    "name" text not null,
    foreign key ("group_id") references "groups" ("id")
        on delete cascade,
    unique ("group_id", "name")
);