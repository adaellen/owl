use config::types::ResourceId;
pub mod cache;
pub mod storage;

pub struct resource_identifier {
	name: String,
	id: ResourceId,
}

pub struct resource_session {
	resource_cache: cache_session,
	next_resource_id: &mut ResourceId,
}

impl resource_session {
	pub fn retrieve_resource(id: resource_identifier){

	}
}

pub trait NamedResource {

}
