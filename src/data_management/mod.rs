use config::types::ResourceIdNumber;
pub mod cache;
pub mod storage;
pub mod database;

enum ResourceType {
	Note,
	Name,
	Gender,
	Contact,
	TimePeriod,

}

pub struct resource_identifier_tag {
	name: String,
	id: ResourceIdNumber,
}

pub struct resource_session {
	resource_cache: cache_session,
	next_resource_id: &mut ResourceIdNumber,
}

impl resource_session {
	pub fn retrieve_resource(id: resource_identifier_tag){

	}
}

pub trait NamedResource {

}
