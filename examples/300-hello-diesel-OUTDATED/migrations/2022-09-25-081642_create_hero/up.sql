CREATE TABLE heroes (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
    hero_type_id INTEGER NOT NULL,
    FOREIGN KEY(hero_type_id) REFERENCES hero_types(id)
)
