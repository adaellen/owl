use log::{error, info, warn};
use log4rs;

mod user_interface;

fn main() {
    log4rs::init_file("src/config/log4rs.yaml", Default::default()).unwrap();
    info!("booting up");
    warn!("this is a warning!!!");
    error!("An error has occurred!!! Grammar included!!!");

    println!("Hello, world!");
    user_interface::interface_with_me();


}
