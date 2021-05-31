use config::types::IdNumber;
pub mod cache;
pub mod storage;

pub struct resource_identifier {
	name: String,
	id: IdNumber,
}

pub struct resource_session {
	resource_cache: cache_session,
	next_resource_id: &mut IdNumber,
}

impl resource_session {
	pub fn retrieve_resource(id: resource_identifier){

	}
}
