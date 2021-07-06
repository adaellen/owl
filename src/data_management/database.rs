use config::types::database_types::DatabaseColumnType;

struct ColumnDefinition {
	column_name: String,
	column_type: DatabaseColumnType,
}

impl ColumnDefinition {

}

//Remember that this is only meant to represent the table definition, not the table.
struct TableDefinition {
	table_name: String,
	table_columns: &[ColumnDefinition],
}

impl TableDefinition {
	fn create_table(&self){

	}
}

struct DatabaseDefinition {
	tables: &[TableDefinition],
}

trait DatabaseConnection {

}

trait QueryConstructor {
	fn construct_query_create_table(table_def: TableDefinition) -> String;
	fn construct_query_remove_table(table_def: TableDefinition) -> String;
}

trait Database {
	fn start(&self);
	fn run_query(&self, query: String);
}