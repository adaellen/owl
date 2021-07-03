use chrono::{DateTime, Utc};
use config::constants::note_constants::MAX_NOTE_REFERENCES;
use config::constants::note_constants::MAX_CONTACT_REFERENCES;
use config::constants::note_constants::MAX_TIME_PERIODS;
use config::constants::note_constants::MAX_NAMES_PER_CONTACT;
use config::constants::note_constants::MAX_GENDERS_PER_CONTACT;
use config::constants::note_constants::MAX_LOCATIONS_PER_CONTACT;
use config::ResourceId;
use config::ResourceIdCollection;
use std::time::Instant;
pub mod format;

pub struct note {
	id: ResourceId,
	name: ResourceId,
	text_description: ResourceId,
	contacts: ResourceIdCollection,
	notes_related: ResourceIdCollection,
	notes_complements: ResourceIdCollection,
	time_periods_involved: ResourceIdCollection,
}

pub struct contact {
	id: ResourceId,
	name: ResourceId,
	names_in_sequential_order: ResourceIdCollection,
	name_initials: ResourceId,
	name_nickname: ResourceId,
	name_short: ResourceId,
	name_maiden: ResourceId,
	time_period_birthday: ResourceId,
	genders: ResourceIdCollection,
	locations: ResourceIdCollection,
	occupations: ResourceIdCollection,
	note: ResourceId,
	languages: ResourceIdCollection,
	photo: ResourceId,
	email_addresses: ResourceIdCollection,
}

pub struct time_period {
	id: ResourceId,

}