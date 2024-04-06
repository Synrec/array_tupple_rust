use std::io;

fn main() {
    let mut arr_index = String::new();
    io::stdin()
        .read_line(&mut arr_index)
        .expect("Failed to read input.");
    let arr_index: usize = match arr_index.trim().replace(" ", "").parse(){
        Ok(num) => num,
        Err(_) => return ()
    };
    let arr = [07,17,27,37,46];
    println!("{}", arr[arr_index])
}
