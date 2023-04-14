use std::{
    fs, thread,
    time::Duration,
};

pub fn watch<T: Fn(&str)>(file_path: &str, func: T) {
    let file_obj = fs::File::open(file_path).expect("Failed to open file");

    let mut last_write_time = file_obj.metadata().unwrap().modified().unwrap();
    let mut pre_contents = fs::read_to_string(file_path).expect("Failed to read file");

    loop {
        let current_write_time = file_obj.metadata().unwrap().modified().unwrap();
        if last_write_time != current_write_time {
            last_write_time = current_write_time;

            let contents = fs::read_to_string(file_path).expect("Failed to read file");
            let line = &contents[pre_contents.len()..].trim();

            func(line);

            pre_contents = contents;
        }

        thread::sleep(Duration::from_secs(1));
    }
}
