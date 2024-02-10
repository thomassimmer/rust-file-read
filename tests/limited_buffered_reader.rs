#[cfg(test)]
mod tests {
    use rust_file_read::config::{NB_BYTES_ALLOWED_PER_LINE, USE_LIMITED_BUFFER};
    use rust_file_read::utils::helpers::build_reader;
    use std::fs;
    use std::thread::sleep;
    use std::time::Duration;
    use std::{fs::File, io::BufRead};

    fn write_in_file_and_read(n: usize, duration: Option<Duration>) {
        // Build a file with our line containing n zéros.
        let file_path = "test";
        File::create(file_path).unwrap();
        fs::write(file_path, &"0".repeat(n)).unwrap();

        // Open this file and create the reader.
        let file = File::open(file_path).unwrap();
        let reader = build_reader(file);
        let lines = reader.lines().enumerate();

        for (_index, bufread) in lines {
            let line = bufread.unwrap_or_default();

            if USE_LIMITED_BUFFER && n > NB_BYTES_ALLOWED_PER_LINE {
                assert_eq!(line.len(), NB_BYTES_ALLOWED_PER_LINE);
            } else {
                assert_eq!(line.len(), n);
            }

            if let Some(duration) = duration {
                println!("\n---------- LOOK htop NOW -----------\n");
                sleep(duration);
                println!("\n---------- STOP LOOKING htop NOW -----------\n");
            }
        }

        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn read_lines_under_limit() {
        write_in_file_and_read(4, None);
    }

    #[test]
    fn read_lines_above_limit() {
        write_in_file_and_read(1_000_000_000, Some(Duration::from_millis(5000)))
    }
}
