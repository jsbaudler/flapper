use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;

fn get_rows(connection: &sqlite::Connection, table: &String, column: &String) -> Vec<String> {
    let mut rows: Vec<String> = Vec::new();

    let query = format!("SELECT {} FROM {}", column, table);
    let mut statement = connection.prepare(query).unwrap();

    while let Ok(sqlite::State::Row) = statement.next() {
        rows.push(statement.read::<String, _>(0).unwrap());
    }

    rows
}

fn get_columns(connection: &sqlite::Connection, table: &String) -> HashMap<String, Vec<String>> {
    let mut columns: HashMap<String, Vec<String>> = HashMap::new();

    let query = format!("SELECT name FROM PRAGMA_TABLE_INFO('{}')", table);
    let mut statement = connection.prepare(query).unwrap();

    while let Ok(sqlite::State::Row) = statement.next() {
        let columnname = statement.read::<String, _>(0).unwrap();
        let rows = get_rows(connection, table, &columnname);

        columns.insert(columnname, rows);
    }

    columns
}

fn get_tables(connection: &sqlite::Connection) -> HashMap<String, HashMap<String, Vec<String>>> {
    let mut tables: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();

    let query = "SELECT name FROM sqlite_schema WHERE type = 'table' AND name NOT LIKE 'sqlite_%'";
    let mut statement = connection.prepare(query).unwrap();

    while let Ok(sqlite::State::Row) = statement.next() {
        let tablename = statement.read::<String, _>(0).unwrap();
        let columns = get_columns(connection, &tablename);

        tables.insert(tablename, columns);
    }

    tables
}

pub fn get_databases() -> HashMap<String, HashMap<String, HashMap<String, Vec<String>>>> {
    let mut databases: HashMap<String, HashMap<String, HashMap<String, Vec<String>>>> =
        HashMap::new();

    let paths = fs::read_dir("./dbs/").unwrap();

    for path in paths {
        let file = path.unwrap().path();

        // if an sqlite file
        if file.extension() == Some(OsStr::new("sqlite")) {
            let connection = sqlite::open(&file).unwrap();
            let filename = file.as_path().display().to_string();
            let tables = get_tables(&connection);

            databases.insert(filename, tables);
        }
    }
    databases
}

pub fn main() -> HashMap<String, HashMap<String, HashMap<String, Vec<String>>>> {
    get_databases()
}
