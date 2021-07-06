trait QueryConstructor {
	fn construct_query_create_table(table_def: TableDefinition) -> String;
	fn construct_query_remove_table(table_def: TableDefinition) -> String;
}