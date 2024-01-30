
use std::env;

pub trait Arguments{
    fn arguments(&self) -> String;
    fn exists(&self) -> bool;
}

impl Arguments for String{
    fn arguments(&self) -> String{
        // Gets the argument based on the argument name --example "value" and return the value
        let args: Vec<String> = env::args().collect();
        let mut value = String::new();
        if args.contains(&self.to_string()) {
            let index = args.iter().position(|r| r == self).unwrap();
            value = args[index + 1].to_string();
        }
        value
    }

    fn exists(&self) -> bool{
        // Gets the argument based on the argument name --example "value" and return the value
        let args: Vec<String> = env::args().collect();
        args.contains(&self.to_string())
    }
}

impl Arguments for i32{
    fn arguments(&self) -> String{
        // Gets the argument based on the index and return the value
        let args: Vec<String> = env::args().collect();
        let mut value = String::new();
        if args.len() > *self as usize {
            value = args[*self as usize].to_string();
        }
        value
    }

    fn exists(&self) -> bool {
        // Returns true if the argument exists
        let args: Vec<String> = env::args().collect();
        args.len() > *self as usize
    }
}

pub fn get_argument<T: Arguments>(arg: T) -> String{
    arg.arguments()
}

pub fn argument_exists<T: Arguments>(arg: T) -> bool{
    arg.exists()
}