// RUST PROJECT to connect files to a PostGresSQL DB

extern crate postgres;

use postgres::{Client, NoTls};


// Create a struct called person (This will be data that gets input into the DB)
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>
}

fn main() {
    // lets connect to the POSTGRES db using the CONNECTION modules, unwrap opens the connection
    let mut conn = Client::connect("postgresql://postgres@localhost::5433", NoTls::None)
        .unwrap();

    // the connection will then execute the command to create a new table called person, in that table we will
    // populate the data based on the struct
    conn.execute("CREATE TABLE person (
                id SERIAL PRIMARY KEY,
                name VARCHAR NOT NULL,
                data BYTEA
                )", &[]).unwrap();
    let me = Person {
        id: 0,
        name: "Alex".to_owned(),
        data: None
    };

    // this adds the "me" into the DB
    conn.execute("INSERT INTO person(name, data) VALUES ($1, $2)",
        &[&me.name, &me.data]).unwrap();


    // this is a custom search to retrieve and show the "me" that wwas added to the DB
    for row in &conn.query("SELECT id, name, data FROM person", &[]).unwrap() {
        let person = Person {
            id: row.get(0),
            name: row.get(1),
            data: row.get(2)
        };
        println!("Found person {}", person.name)
    }


}

