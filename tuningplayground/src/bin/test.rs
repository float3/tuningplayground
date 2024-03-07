use tuningplayground::convert_notes_core;

fn main() {
    let notes: Vec<String> = vec![
        "A4".to_string(),
        "B3".to_string(),
        "C6".to_string(),
        "D2".to_string(),
        "E1".to_string(),
        "F##N1".to_string(),
        "Gb4".to_string(),
    ];
    println!("{}", convert_notes_core(notes));
}
