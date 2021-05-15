use crate::config::constants::logger_constants;

pub fn init_logger(){
	log4rs::init_file(logger_constants::LOGGER_CONFIG_PATH, Default::default()).unwrap();
}