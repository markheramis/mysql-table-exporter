mod arguments;
mod database;

use arguments::Args;
use database::Database;
use std::fs::File;
use std::io::Write;
use clap::Parser;

fn main() {
    // Parse the command-line arguments.
    let args: Args = Args::parse();

    // Create a new `Database` instance using the command-line arguments.
    let mut db = Database::new(&args);

    // Execute a SQL query to get the data from the database.
    let result = db.query(&args.table);

    // Create a new file to write the SQL insert statements to.
    let mut file = File::create(format!("{}.sql", args.table)).unwrap();

    // Iterate over each row in the result set.
    for row in result {
        // Convert each column value in the row to a SQL string.
        let values: Vec<String> = row.unwrap()
            .iter()
            .map(|value| value.as_sql(false))
            .collect();

        // Construct the SQL insert statement.
        // If the `complete_insert` argument is "true", include the column names in the insert statement.
        // Otherwise, do not include the column names.
        let insert_query = if args.complete_insert == "true" {
            // Generate a complete SQL INSERT statement.
            format!("INSERT INTO {} ({}) VALUES ({})", args.table, db.columns, values.join(", "))
        } else {
            // Generate an SQL INSERT statement.
            format!("INSERT INTO {} VALUES ({})", args.table, values.join(", "))
        };

        // Write the insert statement to the file.
        writeln!(file, "{};", insert_query).unwrap();
    }
}
