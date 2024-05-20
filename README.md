# MySQL Table Exporter

MySQL Table Exporter is a command-line tool written in Rust that exports data from a MySQL table to a SQL file. The generated SQL file contains `INSERT` statements that can be used to import the data into another database.

## Features

- Export data from any MySQL table
- Option to select specific columns
- Option to include column names in the `INSERT` statements
- Connect to the MySQL server using custom credentials

## Usage

1. Install Rust: Follow the instructions on the official Rust website to install Rust on your machine.

2. Clone the repository: `git clone https://github.com/markheramis/mysql-table-exporter.git`

3. Navigate to the project directory: `cd mysql-table-exporter`

4. Build the project: `cargo build --release`

5. Run the program: `cargo run -- [OPTIONS]`

## Command-Line Arguments

- `--host`: The hostname or IP address of the MySQL server
- `--port`: The port number on which the MySQL server is listening
- `--database`: The name of the MySQL database to export data from
- `--table`: The name of the table in the MySQL database to export data from
- `--columns`: A comma-separated list of column names to export data from. If this argument is not provided, data from all columns will be exported
- `--username`: The username to use when connecting to the MySQL server
- `--password`: The password to use when connecting to the MySQL server
- `--complete-insert`: Whether to include column names in the `INSERT` statements. If this argument is set to "true", the column names will be included. If this argument is set to "false" or not provided, the column names will not be included

## Example

To export data from the `users` table in the `test` database on `localhost`, you can run the following command:

```bash
cargo run -- --host localhost --port 3306 --database test --table users --username root --password secret
```
This will create a users.sql file in the current directory with the exported data.

## Contributing
Contributions are welcome! Please feel free to submit a Pull Request.