// @generated automatically by Diesel CLI.

diesel::table! {
    sections (id) {
        id -> Int4,
        #[max_length = 80]
        title -> Varchar,
        #[max_length = 255]
        section_description -> Varchar,
        parent_section_id -> Nullable<Int4>,
    }
}
