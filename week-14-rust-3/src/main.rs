use std::fs;
fn main() {
    let greeting_file_result = fs::read_to_string("hello.txt"); // type is Result enum of greeting_file_result
    // It contains the enum Result not content of file
    match greeting_file_result{
        Ok(file_content) => {
            // file_content has the content of the file
            println!("File read successfully: {:?}", file_content);
        },
        Err(error) =>{
            println!("Failed to read file: {:?}", error);
        }
    }

    let content_file = fs::read_to_string("hello.txt").unwrap_or(String::from("Empty file"));
    println!("{}", content_file);

    // .unwrap() :: either return the actual value or panic

    // Option :: nullabiliu in safe and expressinve way
    let str = find_last_a(String::from("Hregwregve"));
    match str {
            Some(str) => println!("last a was found at {}",str),
            None => println!("Could not find an a inside the string"),
    }

    let str = find_first_a(String::from("Harkirat"));
    match str {
            Some(str) => println!("first a was found at {}",str),
            None => println!("Could not find an a inside the string"),
    }
}

fn find_last_a(str: String) -> Option<u32> {
    for (index, char) in str.char_indices().rev(){ // .rev() works on DoubleEndedIterator
        // for reverse char_indices().rev()
        if char == 'a' {
            return Some(index as u32); // can't simply return index
            // Option has 2 variants Some and Null
        }
    }
    None
}

fn find_first_a(str: String) -> Option<u32> {
    for (index, char) in str.chars().enumerate(){ // .rev() works on DoubleEndedIterator
        // for reverse char_indices().rev()
        if char == 'a' {
            return Some(index as u32); // can't simply return index
            // Option has 2 variants Some and Null
        }
    }
    None
}
