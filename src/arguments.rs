use clap::Parser;

/// This struct represents the command-line arguments for the program.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The hostname or IP address of the MySQL server.
    #[arg(long)]
    pub host: String,

    /// The port number on which the MySQL server is listening.
    #[arg(long)]
    pub port: u16,

    /// The name of the MySQL database to export data from.
    #[arg(short, long)]
    pub database: String,

    /// The name of the table in the MySQL database to export data from.
    #[arg(short, long)]
    pub table: String,

    /// A comma-separated list of column names to export data from.
    /// If this argument is not provided, data from all columns will be exported.
    #[arg(short, long)]
    pub columns: Option<String>,

    /// The username to use when connecting to the MySQL server.
    #[arg(short, long)]
    pub username: String,

    /// The password to use when connecting to the MySQL server.
    #[arg(short, long)]
    pub password: String,

    /// A boolean flag that determines whether to include column names in the generated INSERT statements.
    /// If this argument is set to "true", the column names will be included.
    /// If this argument is set to "false" or not provided, the column names will not be included.
    #[arg(short = 'i', long = "complete-insert", default_value = "true")]
    pub complete_insert: String,
}
