pub fn init_logger(){
	log4rs::init_file("src/config/log4rs.yaml", Default::default()).unwrap();
}