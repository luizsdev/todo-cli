use std::{env, fs::OpenOptions, io::Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let operation = &args[1];

    if operation.eq("add") {
        let content_to_insert = &args[2];

        println!("Inserting task {:?}", content_to_insert);
        let mut file = OpenOptions::new()
            .append(true)
            .open("todolist.txt")
            .expect("error while reading file");

        writeln!(file, "{}", content_to_insert).expect("error while inserting task");

        println!("Inserted task {:?}", content_to_insert);
    }

    if operation.eq("clean") {
        println!("Cleaning tasks");
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("todolist.txt")
            .expect("error while opening file in write mode");

        println!("Cleaned all tasks");
    }
}
