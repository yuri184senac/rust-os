use std::io::{self, stdout, Write};
use std::process::Command;
fn main() {
    print!("> ");
    //Faz com que o > apareca antes da linha
    stdout().flush();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error");

    
    Command::new("cmd")
            .args(["/C", "echo hello"])
            .output()
            .expect("failed to execute process")
    

   
    
   
}
