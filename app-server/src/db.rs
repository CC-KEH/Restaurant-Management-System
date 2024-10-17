use rusqlite::Connection;

pub fn get_db_connection() -> Connection {    
    let conn = Connection:open("restaurant.db").expect("Failed to open SQLite connection");

}

pub fn initialize_db() {
    println!("Initializing the database...")
    let conn = Connection::open("restaurant.db").expect("Failed to open SQLite connection");
    conn.execute("PRAGMA foreign_keys=ON;",[]).expect("Failed to enable foreign key support");

    println!("Create Table table");
    creata_table_table(conn).expect("Failed to create Table table.");

    println!("Create Menu table");
    creata_menu_table(conn).expect("Failed to create Menu table.");

    println!("Create Order table");
    creata_order_table(conn).expect("Failed to create Order table.");

    println!("Create OrderItem table");
    creata_order_item_table(conn).expect("Failed to create OrderItem table.");
}

 fn creata_table_table(conn: &Connection) -> rusqlite::Result<()>{
    conn.execute("CREATE TABLE IF NOT EXISTS tables(id INTEGER PRIMARY KEY, code TEXT NOT NULL UNIQUE)", [])?;
    Ok(())
}

 fn creata_menu_table(conn: &Connection) -> rusqlite::Result<()>{
    conn.execute("CREATE TABLE IF NOT EXISTS menus(id INTEGER PRIMARY KEY, name TEXT NOT NULL)", [])?;
    Ok(())
}

 fn creata_order_table(conn: &Connection) -> rusqlite::Result<()>{
    conn.execute("CREATE TABLE IF NOT EXISTS orders(id INTEGER PRIMARY KEY, table_id INTEGER NOT NULL, FOREIGN KEY (table_id) REFERENCES tables(id), UNIQUE(table_id))", [])?;
    Ok(())
}

 fn creata_order_item_table(conn: &Connection) -> rusqlite::Result<()>{
    conn.execute("CREATE TABLE IF NOT EXISTS order_items(id INTEGER PRIMARY KEY, order_id INTEGER NOT NULL, menu_id INTEGER NOT NULL, cooking_time NOT NULL, quantity INTEGER NOT NULL default 1, FOREIGN KEY (order_id) REFERENCES orders(id), FOREIGN KEY (menu_id) REFERENCES menus(id))", [])?;
    Ok(())
}