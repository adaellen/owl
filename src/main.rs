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

    let text = "foo\r\nbar\n\nbaz\n";
    let mut lines = text.lines();

    assert_eq!(Some("foo"), lines.next());
    assert_eq!(Some("bar"), lines.next());
    assert_eq!(Some(""), lines.next());
    assert_eq!(Some("baz"), lines.next());

    assert_eq!(None, lines.next());
}
