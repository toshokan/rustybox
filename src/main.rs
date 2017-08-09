use std::io;
use std::io::Write;
use std::env;
use std::path::{Path,PathBuf};
use std::collections::HashMap;
use std::fs;

fn main() {
    let mut env:HashMap<String, String> = HashMap::new();
    env.insert(String::from("prompt"), String::from("rustybox >"));

    loop {
        let curr_path = env::current_dir().unwrap();
        print!("{}{} > ", env.get(&String::from("prompt")).unwrap(), curr_path.display());
        io::stdout().flush().expect("Failed to flush line");
        let mut userin = String::new();
        io::stdin().read_line(&mut userin).expect("Failed to read line");
        process_input(&userin);
    }
}

fn process_input(ui:&String) {
    let ui_vec:Vec<&str> = ui.split_whitespace().collect();
    match ui_vec[0] {
        "cd" => cd_main(&ui_vec),
        "ls" => ls_main(&ui_vec),
        _ => ()
    }
}

fn cd_main(it:&Vec<&str>){
    let home_dir = env::home_dir().unwrap();
    let mut n_path = PathBuf::new();
    let mut path_to = Path::new(home_dir.to_str().unwrap());
    if it.len() > 1 {
        n_path = parse_path(&String::from(it[1]));
        path_to = n_path.as_path();
    }
    env::set_current_dir(&path_to).unwrap();
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

fn parse_path(s:&String) -> PathBuf {
   let mut path = PathBuf::new();
   if Path::new(s).is_relative() {
       path = env::current_dir().unwrap();
   }
   for v in s.split("/") {
       match v {
           "~" => {
               let home_dir = env::home_dir().unwrap();
               path.push(home_dir);
           }
           ".." => {
               let mut temp = PathBuf::new();
               match path.parent() {
                  Some(p) => {
                      temp.push(p);
                  }
                  None => {
                      let root_dir = Path::new("/");
                      temp.push(root_dir);
                  }
               }
               path = temp;
           }
           e => {
               path.push(e);
           }
       }
   }
   return path;
}
