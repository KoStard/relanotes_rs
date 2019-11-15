table! {
    groups (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    node_types (id) {
        id -> Integer,
        name -> Text,
        value -> Text,
    }
}

table! {
    nodes (id) {
        id -> Integer,
        linked_to_id -> Nullable<Integer>,
        type_id -> Integer,
        name -> Nullable<Text>,
        description -> Nullable<Text>,
        group_id -> Integer,
    }
}

joinable!(nodes -> groups (group_id));
joinable!(nodes -> node_types (type_id));

allow_tables_to_appear_in_same_query!(
    groups,
    node_types,
    nodes,
);
