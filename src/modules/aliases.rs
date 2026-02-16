type DBType = String;
type MysqlDBType = DBType;
type PostgresDBType = DBType;

pub fn alias() {
    let mysql: MysqlDBType = "mysql".to_string();
    let postgres: PostgresDBType = "postgres".to_string();

    println!("mysql: {}", mysql);
    println!("postgres: {}", postgres)
}