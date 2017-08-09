use std::io;
use std::io::Write;
use std::env;
use std::path::Path;
use std::collections::HashMap;
use std::fs;

fn main() {
    let curr_path = env::current_dir().unwrap();
    let mut env:HashMap<String, String> = HashMap::new();
    env.insert(String::from("path"), String::from(curr_path.to_str().unwrap()));
    env.insert(String::from("prompt"), String::from("rustybox >"));

    loop {
        print!("{}{} > ", env.get(&String::from("prompt")).unwrap(), env.get(&String::from("path")).unwrap());
        io::stdout().flush().expect("Failed to flush line");
        let mut userin = String::new();
        io::stdin().read_line(&mut userin).expect("Failed to read line");
        process_input(&userin, &mut env);
    }
}

fn process_input(ui:&String, e:&mut HashMap<String, String>) {
    let ui_vec:Vec<&str> = ui.split_whitespace().collect();
    match ui_vec[0] {
        "cd" => cd_main(&ui_vec, e),
        "ls" => ls_main(&ui_vec),
        _ => ()
    }
}

fn cd_main(it:&Vec<&str>, e:&mut HashMap<String, String>){
    let home_dir = env::home_dir().unwrap();
    let mut path_to = Path::new(home_dir.to_str().unwrap());
    if it.len() > 1 {
        path_to = Path::new(it[1]);
    }
    match env::set_current_dir(&path_to) {
        Ok(_) => {
            let curr_path = env::current_dir().unwrap();
            e.insert(String::from("path"), String::from(curr_path.to_str().unwrap()));
        }
        Err(_) => ()
    }
}

fn ls_main(it:&Vec<&str>){
    let current_dir = env::current_dir().unwrap();
    let mut path_req = Path::new(current_dir.to_str().unwrap());
    if it.len() > 1 {
        path_req = Path::new(it[1]);
    }
    match fs::read_dir(&path_req) {
        Ok(o) => {
            for v in o {
                println!("{}", v.ok().unwrap().path().display())
            }
        }
        _ => println!("oop")
    }
}
