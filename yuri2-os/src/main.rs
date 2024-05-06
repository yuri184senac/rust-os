use std::env::args_os;
use std::io::{self, stdout, Write};
use std::process::{Child, Command, ExitCode};
fn main() {    
    loop {
       
        print!("yuri-shell > ");
        let _ = stdout().flush();
        //Faz com que o > apareca antes da linha
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error"); 

        let output =  Command::new("cmd")
        .arg("/C")
        .arg(input).spawn().expect("Falha na execução do processo");

        let terminal = output.stdout;
        
        if !terminal.is_none() {
            println!("{:?}", terminal); 
           
        } 
        
        
        
        
        
               
    }    
}
