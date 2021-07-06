use config::types::database_types::DatabaseColumnType;

struct ColumnDefinition {
	column_name: String,
	column_type: DatabaseColumnType,
}

//Remember that this is only meant to represent the table definition, not the table.
struct TableDefinition {
	table_name: String,
	table_columns: &[ColumnDefinition],
}

pub fn construct_database_definition()-> &[TableDefinition]{
	return null
}