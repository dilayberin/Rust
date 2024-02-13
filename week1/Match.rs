fn Match(){
    fn main() {
        let num = 5;
       
        let result = match num {
            1 => "The number is one!",
            2 => "The number is two!",
            3 => "The number is three!",
            _ => "The number is something else!",
        };
       
        println!("{}", result);
    }
}