use mysql::*;
use mysql::prelude::*;
use super::Args;

// Define a `Database` struct to hold the database connection and the column names.
pub struct Database {
    // The pooled connection to the MySQL server.
    conn: PooledConn,
    // A string containing the names of the columns to be exported.
    pub columns: String,
}

// Implement methods for the `Database` struct.
impl Database {
    // The `new` function creates a new `Database` instance.
    // It takes a reference to an `Args` instance as a parameter.
    pub fn new(args: &Args) -> Self {
        // Create a new `OptsBuilder` instance and set the necessary options.
        let opts = OptsBuilder::new()
            .ip_or_hostname(Some(args.host.to_string()))
            .tcp_port(args.port)
            .user(Some(args.username.to_string()))
            .pass(Some(args.password.to_string()))
            .db_name(Some(args.database.to_string()));
        // Create a new connection pool and get a connection from it.
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();
        // Determine the column names.
        // If the `columns` argument is provided, use it.
        // Otherwise, query the `INFORMATION_SCHEMA.COLUMNS` table to get the column names.
        let columns = match &args.columns {
            Some(cols) => cols.split(',').map(|s| s.to_string()).collect::<Vec<String>>().join(", "),
            None => {
                let column_query = format!("SELECT COLUMN_NAME FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_NAME = '{}' AND TABLE_SCHEMA = '{}'", args.table, args.database);
                let column_result: Vec<String> = conn.query_map(column_query, |column_name| column_name).unwrap();
                column_result.join(", ")
            },
        };
        // Return a new `Database` instance.
        Self { conn, columns }
    }

    // The `query` function executes a SQL query and returns the result.
    // It takes a reference to a string containing the table name as a parameter.
    pub fn query(&mut self, table: &str) -> Vec<Row> {
        // Construct the SQL query.
        let query = format!("SELECT {} FROM {}", self.columns, table);
        // Execute the query and return the result.
        self.conn.exec(query, ()).unwrap()
    }
}
