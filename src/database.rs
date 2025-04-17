use rand::Rng;
use rusqlite::{params, Connection, Result};

pub fn fetch_random_word() -> Result<String> {
    println!("Current directory: {:?}", std::env::current_dir());
    let connection = match Connection::open("resources/db-cities-countries.db")
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to open database: {}", e);
            return Err(e);
        }
    };
    println!("Database connection established.");

    let categories = ["City", "Country", "Plant", "Animal", "River"];
    let letters = "ABCDE".chars().collect::<Vec<_>>();

    let random_category = categories[rand::thread_rng().gen_range(0..categories.len())];
    let random_letter = letters[rand::thread_rng().gen_range(0..letters.len())];

    println!("Random category: {}", random_category);
    println!("Random letter: {}", random_letter);

    let table_name = format!("{}_{}", random_category, random_letter);
    let query = format!("SELECT name FROM {} ORDER BY RANDOM() LIMIT 1", table_name);

    let mut stmt = connection.prepare(&query)?;
    let mut rows = stmt.query([])?;

    if let Some(row) = rows.next()? {
        let value: String = row.get(0)?;
        println!("Value: {}", value);
        return Ok(format!("Random {} ({}): {}", random_category, random_letter, value));
    }

    Ok(format!("No data found in table {}", table_name))
}