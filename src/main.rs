// use std::{thread, time};
use std::collections::{HashMap};
use regex::Regex;
use glob::glob;
use std::io::{stdout, Write};
use crossterm::{QueueableCommand, ExecutableCommand, cursor, terminal};

fn main() {
    let mut stdout = stdout();
    let mut count = 0;
    let mut files = HashMap::new();
    let re_video = Regex::new(r".*\.(mp4|mkv|wmv)").unwrap();
    let re_file = Regex::new(r"[[:alpha:]]-\d").unwrap();

    stdout.execute(cursor::Hide).unwrap();

    for entry in glob("//192.168.3.10/jav/media/**/*").unwrap() {
        match entry {
            Ok(path) => {
                count += 1;

                if !path.is_file() {
                    continue;
                }

                // do process
                let filename = path.file_name().unwrap().to_str().unwrap();
                if !re_video.is_match(filename) {
                    continue;
                }

                let mat = re_file.find(filename);
                if !mat.is_none() {
                    let str = mat.unwrap().as_str();
                    files.insert(str, str);
                }
                // if re_file.is_match(filename) {
                //     files.insert(path.clone(), path.clone());
                // }

                // show process
                stdout.queue(cursor::SavePosition).unwrap();
                // stdout.write_all(format!("scan files {:?}", path).as_bytes()).unwrap();
                stdout.write_all(format!("scan files {:?}", filename).as_bytes()).unwrap();
                stdout.queue(cursor::RestorePosition).unwrap();
                stdout.flush().unwrap();

                stdout.queue(cursor::RestorePosition).unwrap();
                stdout.queue(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();
            },
            Err(e) => println!("{:?}", e),
        }
    }

    stdout.execute(cursor::Show).unwrap();

    println!("scanned files {:?}", count);
    println!("processed files {:?}", files.len());

    return;
}
