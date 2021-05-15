pub mod aural;
pub mod graphical;
pub mod script;
pub mod text;

pub fn interface_with_me(){
	println!("You are awesome!!!");
}

#[cfg(test)]
mod test;
