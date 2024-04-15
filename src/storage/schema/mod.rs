diesel::table! {
    todos {
        id -> Integer,
        title -> Text,
        completed -> Bool,
    }
}
