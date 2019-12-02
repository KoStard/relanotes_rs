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
        name -> Text,
        description -> Nullable<Text>,
        subgroup_id -> Integer,
    }
}

table! {
    subgroups (id) {
        id -> Integer,
        group_id -> Integer,
        name -> Text,
    }
}

joinable!(nodes -> node_types (type_id));
joinable!(nodes -> subgroups (subgroup_id));
joinable!(subgroups -> groups (group_id));

allow_tables_to_appear_in_same_query!(
    groups,
    node_types,
    nodes,
    subgroups,
);
