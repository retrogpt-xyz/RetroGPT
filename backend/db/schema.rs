// @generated automatically by Diesel CLI.

diesel::table! {
    tmsgs (id) {
        id -> Int4,
        body -> Text,
        prnt_id -> Nullable<Int4>,
    }
}
