use chrono::{DateTime, Utc};
use config::constants::note_constants::MAX_NOTE_REFERENCES;
use config::constants::note_constants::MAX_CONTACT_REFERENCES;
use config::constants::note_constants::MAX_TIME_PERIODS;
use config::constants::note_constants::MAX_NAMES_PER_CONTACT;
use config::constants::note_constants::MAX_GENDERS_PER_CONTACT;
use config::constants::note_constants::MAX_LOCATIONS_PER_CONTACT;
use config::constants::note_constants::MAX_OCCUPATIONS_PER_CONTACT;
use config::ResourceIdNumber;
use config::ResourceIdNumberCollection;
use std::time::Instant;
pub mod format;

pub struct note_ref {
	id_tag: resource_identifier_tag,
	text_description: ResourceIdNumber,
	contacts: ResourceIdNumberCollection,
	notes_related: ResourceIdNumberCollection,
	notes_complements: ResourceIdNumberCollection,
	time_period_created: ResourseIdNumber,
	time_periods_involved: ResourceIdNumberCollection,
}

pub trait Note {
	fn get_text_description(&self)->String;
	fn get_contacts(&self)-> &[Contact];
	fn get_notes_related(&self) -> &[Note];
	fn get_notes_complements(&self)-> &[Note];
	fn get_time_periods_involved(&self) -> &[TimePeriod];
}

pub struct contact_ref {
	id_tag: resource_identifier_tag,
	names_in_sequential_order: ResourceIdNumberCollection,
	name_initials: ResourceIdNumber,
	name_nickname: ResourceIdNumber,
	name_short: ResourceIdNumber,
	name_maiden: ResourceIdNumber,
	time_period_birthday: ResourceIdNumber,
	genders: ResourceIdNumberCollection,
	locations: ResourceIdNumberCollection,
	occupations: ResourceIdNumberCollection,
	note: ResourceIdNumber,
	languages: ResourceIdNumberCollection,
	photo: ResourceIdNumber,
	email_addresses: ResourceIdNumberCollection,
}

pub trait Contact {
	fn get_names_in_sequential_order(&self)->&[String];
	fn get_name_initials(&self)->String;
	fn get_name_nickname(&self)->String;
	fn get_name_short(&self)->String;
	fn get_name_maiden(&self)->String;
	fn get_time_period_birthday(&self)->TimePeriod;
	fn get_genders(&self)->&[Gender];
	fn get_locations(&self)->&[Location];
	fn get_occupations(&self)->&[Occupation];
	fn get_note(&self)->&[Note];
	fn get_languages(&self)-> &[Language];
	fn get_photo(&self);
	fn get_email_addresses(&self)->&[EmailAddress];
}

pub struct time_period_ref {
	id: ResourceIdNumber,
	start: DateTime,
	end: DateTime,
}

pub trait TimePeriod {
	fn get_start(&self) -> DateTime;
	fn get_end(&self) -> DateTime;
	fn get_duration(&self)-> OldDuration {
		return self.get_end().signed_duration_since(self.get_start())
	}
}

pub trait Gender {
	fn get_gender_name() -> String;
}

pub trait Location {

}

pub trait Occupation {

}

pub trait Language {

}

pub trait EmailAddress {
	fn get_email_user_name(&self)-> String;
	fn get_email_domain_name(&self)-> String;
	fn get_email_address_whole(&self)-> String {
		return [self.get_email_user_name(), self.get_email_domain_name()].join("@")
	}
}