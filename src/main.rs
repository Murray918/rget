//import the clap crate
extern crate clap;

//use the Arg, App from clap
use clap::{Arg, App};
use indicatif::ProgressBar;



//main function or entry point 
fn main() {
    let matches = App::new("Rget")
        .version("0.1.0")
        .author("Andrew Murray")
        .about("wget shallow clone in Rust")
        .arg(Arg::with_name("URL")
            .required(true)
            .takes_value(true)
            .index(1)
            .help("url to download"))
        .get_matches();
    let url = matches.value_of("URL").unwrap();
    println!("{}", url)
}

fn create_progress_bar(quiet_mode: bool, msg: &str, length: Option<u64>) -> ProgressBar {
    //match is pattern matching the quiet_mode arguments to the desired ProgressBar
    let bar = match quiet_mode {
        true => ProgressBar::hidden(),
        false => {
            match length {
                Some(len) => ProgressBar::new(len),
                None => ProgressBar::new_spinner(),
            }
        }
    };
    
}