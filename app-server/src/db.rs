use rusqlite::Connection;

pub fn get_db_connection() -> Connection {    
    let conn = Connection:open("restaurant.db").expect("Failed to open SQLite connection");

}

pub fn initialize_db() {
    println!("Initializing the database...")
}
