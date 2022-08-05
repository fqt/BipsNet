use std::collections::HashMap;

fn main() {
    let mut to_process: HashMap<u32, String> = HashMap::new();
    to_process.insert(1, "ok".to_string());
    to_process.insert(2, "bad".to_string());

    to_process.retain(process);
    println!("{:?}", to_process);
}

fn process(k: &u32, v: &mut String) -> bool {
    // do stuff with k and v
    v == "ok"
}
