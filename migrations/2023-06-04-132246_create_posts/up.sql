-- Your SQL goes here

CREATE TABLE sections (
    id SERIAL PRIMARY KEY,
    title VARCHAR(80) NOT NULL,
    section_description VARCHAR(255) NOT NULL,
    parent_section_id int REFERENCES sections(id) ON DELETE CASCADE ON UPDATE CASCADE
)
