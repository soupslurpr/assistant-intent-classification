use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let categories = ["alarm", "reminder", "search"];
    let mut formatted_training_data = String::new();

    // TODO: Test if randomizing the order of each data point improves model accuracy
    for category in categories {
        let path = "raw_data/".to_owned() + category + ".txt";
        let path = Path::new(&path);
        let display = path.display();

        let file = match File::open(&path) {
            Err(e) => panic!("Couldn't open {}: {}", display, e),
            Ok(file) => file,
        };

        let reader = io::BufReader::new(file);
        
        for (_index, line) in reader.lines().enumerate() {
            let line = line?;
            formatted_training_data += &format!(
                "<|im_start|>user\n{}\n<|im_end|>\n<|im_start|>assistant\n{}\n<|im_end|>\n",
                line, category
            )
        }
    }

    std::fs::write("formatted_data/training.txt", formatted_training_data)?;

    Ok(())
}
