table! {
    sessions (id) {
        id -> Int4,
        details -> Text,
        started_at -> Nullable<Timestamp>,
        finished_at -> Nullable<Timestamp>,
    }
}
