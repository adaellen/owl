pub mod sqlite;
pub mod def;
pub mod query;

use config::types::database_types::DatabaseColumnType;

trait Database {
	fn start(&self);
	fn run_query(&self, query: String);
}