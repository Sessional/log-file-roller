use clap::{App, load_yaml};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::{self, BufRead};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();

    let file_size = matches.value_of("file-size").unwrap_or("2MiB");
    let file_name = matches.value_of("output-file").unwrap_or("output");
    let file_count = matches.value_of("file-count").unwrap_or("6");
    let file_extension = matches.value_of("file-extension").unwrap_or("log");

    let maximum_file_size = size_to_number(file_size.to_string());

    let newest_file_name = format!("{}.{}", file_name, file_extension);
    let mut file = File::create(&newest_file_name).unwrap();

    let stdin = io::stdin();

    loop {
        let current_file_size = size_of_file(&newest_file_name);
        if current_file_size >= maximum_file_size {
            shuffle_files(file_count.parse::<u64>().unwrap(), file_name.to_string(), file_extension.to_string());
            file = File::create(&newest_file_name).unwrap();
        }

        let mut line = String::new();
        stdin.lock().read_line(&mut line).expect("Unable to read stdin line.");

        writeln!(file, "{}", line).expect("Couldn't write to file.");

        File::sync_all(&file).expect("Failure to call fsync.");
    }
}

fn shuffle_files(maximum_file_count: u64, file_name: String, file_extension: String) {
    for file_number in (0..maximum_file_count).rev() {

        let next_file_name = format!("{}.{}.{}", file_name, file_number + 1, file_extension);
        let mut current_file_name = format!("{}.{}.{}", file_name, file_number, file_extension);
        if file_number == 0 {
            current_file_name = format!("{}.{}", file_name, file_extension);
        }

        let path_to_file = Path::new(&current_file_name);
        let does_current_file_exist = path_to_file.exists();

        if file_number == maximum_file_count && does_current_file_exist {
            fs::remove_file(path_to_file).expect("Unable to delete file.");
        } else if does_current_file_exist && file_number + 1 < maximum_file_count {
            fs::rename(current_file_name, next_file_name).expect("Failed to rename a file to it's next counterpart.");
        }
    }
}

fn size_to_number(file_size: String) -> u64 {
    let unit = file_size.split(char::is_numeric).collect::<Vec<&str>>()[1];
    let count = file_size.split(char::is_alphabetic).collect::<Vec<&str>>()[0];

    let kilobytes = u64::pow(2, 10);
    let megabytes = u64::pow(2, 20);
    let gigabytes = u64::pow(2, 30);

    if unit == "B" {
        return count.parse::<u64>().unwrap();
    } else if unit == "KiB" {
        return count.parse::<u64>().unwrap() * kilobytes;
    } else if unit == "MiB" {
        return count.parse::<u64>().unwrap() * megabytes;
    } else if unit == "GiB" {
        return count.parse::<u64>().unwrap() * gigabytes;
    }

    return 2 * megabytes;
}

fn size_of_file(file_name: &String) -> u64 {
    let metadata = fs::metadata(file_name);
    return metadata.unwrap().len();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determine_size_from_bytes() {
        let bytes = size_to_number(String::from("1B"));
        assert_eq!(1, bytes);
    }

    #[test]
    fn determine_size_from_kilobytes() {
        let bytes = size_to_number(String::from("1KiB"));
        assert_eq!(1024, bytes);
    }

    #[test]
    fn determine_size_from_megabytes() {
        let bytes = size_to_number(String::from("1MiB"));
        assert_eq!(1048576, bytes);
    }

    #[test]
    fn determine_size_from_gigabytes() {
        let bytes = size_to_number(String::from("1GiB"));
        assert_eq!(1073741824, bytes);
    }

    #[test]
    fn determine_file_size() {
        let bytes = size_of_file(&String::from("tests/file_size_test.txt"));
        println!("Bytes: {}", bytes);
        assert_eq!(31, bytes);
    }

    // Validates that tests/shuffle.1.txt does not move to shuffle.2.txt
    // And that tests/shuffle.txt becomes tests/shuffle.1.txt
    #[test]
    fn validate_shuffle_files() {
        File::create("tests/shuffle.txt").expect("Unable to create test file.");
        File::create("tests/shuffle.1.txt").expect("Unable to create test file.");

        shuffle_files(2, "tests/shuffle".to_string(), "txt".to_string());

        let first_shuffled_file = Path::new("tests/shuffle.1.txt");
        let second_shuffled_file = Path::new("tests/shuffle.2.txt");
        assert_eq!(true, first_shuffled_file.exists());
        assert_eq!(false, second_shuffled_file.exists());
        fs::remove_file(first_shuffled_file).expect("Unable to clean up after test.");
    }
}