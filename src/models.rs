use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::sections)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Section {
    pub id: i32,
    pub title: String,
    pub section_description: String,
    pub parent_section_id: Option<i32>,
}
