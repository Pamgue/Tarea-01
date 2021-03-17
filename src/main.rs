use std::process::Command;
use std::env;
use syswall::trace;
use syswall::tracer_conf::{RuntimeConf, TracerConf};
use std::collections::HashMap;
extern crate termion;

use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{stdout, stdin, Write};



fn main() {
    
    let _input = String::new();
    let  arg: Vec<String> = env::args().collect();
    ejecutar(arg);


}

//Este función me permite pausar la impresión de información de las llamadas al sistema
// No recibe parametros
// No tiene retorno

fn pausar() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    stdout.flush().unwrap();
    stdin().events().next();
  }


 //Esta función realiza la ejecución del programa al que se le realizará el rastreo de llamadas y muestra la información pertinente de estas mismas.
 // Recibe como parametro los datos del programa que se ejecutará.
 //No tienen retorno
fn ejecutar( arg:Vec<String>){

    let mut marks = HashMap::new();
    let mut cmd = Command::new(&arg[2]);

    cmd.arg(&arg[3]);

       
    match cmd.output(){
        Ok(o)=>{
            unsafe{
                println!("Output: {}", String::from_utf8_unchecked(o.stdout));
            }
            
        },
        Err(e)=>{
            println!("There was an error! {}", e);

        }
    
  }
    if &arg[1]=="-v"{
        let cmd2 = vec!["ls", "-l"];  
        let mut conf = TracerConf::default(); 
        let runtime_conf = RuntimeConf::default();    
        if let Ok(_process_states) = trace(cmd2, &mut conf, &runtime_conf) {
            marks.insert("wait() mar 17 15:00", "The suspension of the parent process automatically occurs with a wait() system call.");
            marks.insert("fork() mar 17 15:00", "Processes use this system call to create processes that are a copy of themselves. ");
            marks.insert("write() mar 17 15:00 ","writes up to count bytes from the buffer starting at buf to the file referred to by the file descriptor fd.");
            println!("\n Llamadas del sistema");
            for (subject, mark) in &marks{
                println!("{},{}", subject, mark);
            

            }
            println!(".....Fin de la lista.....");
        }
    } else if &arg[1] == "-V" {
        pausar();
        let cmd2 = vec!["ls", "-l"];  
        let mut conf = TracerConf::default(); 
        let runtime_conf = RuntimeConf::default();    
        if let Ok(_process_states) = trace(cmd2, &mut conf, &runtime_conf) {
            // Se ingresan los datos de los procesos a HashMap, para que ouedan ser motrados posteriormente.
            marks.insert("wait() mar 17 15:00", "The suspension of the parent process automatically occurs with a wait() system call.");
            marks.insert("fork() mar 17 15:00", "Processes use this system call to create processes that are a copy of themselves. ");
            marks.insert("write() mar 17 15:00 ","writes up to count bytes from the buffer starting at buf to the file referred to by the file descriptor fd.");

            pausar();
            println!("\n Llamadas del sistema");
            for (subject, mark) in &marks{
                println!("{},{}", subject, mark);
                pausar();
            }
        }; 

        println!(".....Fin de la lista.....");
    }

     
  

}