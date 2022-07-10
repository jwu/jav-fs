// use std::{thread, time};
use std::collections::{HashMap};
use regex::{Regex};
use glob::glob;
use std::io::{stdout, Write};
use crossterm::{QueueableCommand, ExecutableCommand, cursor, terminal};

fn main() {
    let re_video = Regex::new(r".*\.(?i)(mp4|mkv|wmv)").unwrap();
    let re_file = Regex::new(
        r"[[:alpha:]]+-\d+|[[:alpha:]]+\d+"
    ).unwrap();

    let mut stdout = stdout();
    let mut cnt = 0;
    let mut video_cnt = 0;

    let mut files = HashMap::new();
    let mut files_failed = HashMap::new();
    let mut conflicts = Vec::new();

    stdout.execute(cursor::Hide).unwrap();

    for entry in glob("//192.168.3.10/jav/media/**/*").unwrap() {
    // for entry in glob("//192.168.3.10/jav/media/hd/#三宮つばき/**/*").unwrap() {
        match entry {
            Ok(path) => {
                cnt += 1;

                if !path.is_file() {
                    continue;
                }

                // do process
                let filename = path.file_name().unwrap().to_str().unwrap();
                if !re_video.is_match(filename) {
                    continue;
                }
                video_cnt += 1;

                let mat = re_file.find(filename);
                if !mat.is_none() {
                    let match_result = mat.unwrap().as_str().to_owned();
                    let fullpath = path.to_str().unwrap().to_owned();

                    if !files.contains_key(&match_result) {
                        files.insert(match_result, fullpath);
                    } else {
                        conflicts.push(fullpath);
                    }
                } else {
                    let match_result = mat.unwrap().as_str().to_owned();
                    let fullpath = path.to_str().unwrap().to_owned();

                    files_failed.insert(match_result, fullpath);
                }

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

    println!("scanned files {:?}", cnt);
    println!("videos files {:?}", video_cnt);
    println!("actual videos {:?}", files.len());
    println!("failed videos {:?}", files_failed.len());

    for entry in conflicts {
        println!("{}", entry);
    }

    // for (key, value) in files {
    //     println!("{}: {}", key, value);
    // }

    // for (key, value) in files_failed {
    //     println!("{}: {}", key, value);
    // }

    return;
}
