
use std::env;

trait Arguments{
    fn arguments(&self) -> String;
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
}

pub fn get_argument<T: Arguments>(arg: T) -> String{
    arg.arguments()
}