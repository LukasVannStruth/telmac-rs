pub mod cpu;
pub mod display;
pub mod keypad;

pub fn hello_lib() {
    println!("hi from the library");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    
}