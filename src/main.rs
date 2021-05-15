use log::{error, info, warn};
mod config;
mod logger;
mod user_interface;

fn main() {
    logger::init_logger();
    info!("booting up");
    warn!("this is a warning!!!");
    error!("An error has occurred!!! Grammar included!!!");

    println!("Hello, world!");
    user_interface::interface_with_me();


}
