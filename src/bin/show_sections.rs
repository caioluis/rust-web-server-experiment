use self::models::*;
use diesel::prelude::*;
use rust_web_server_experiment::*;

fn main() {
    use self::schema::sections::dsl::*;

    let connection = &mut establish_connection();
    let results = sections
        .limit(5)
        .select(Section::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} sections", results.len());
    for section in results {
        println!("{}", section.title);
        println!("-----------\n");
        println!("{}", section.section_description);
    }
}
