fn main() {
    println!(
        "{:?}",
        recover_secret(vec![
            ['t', 'u', 'p'],
            ['w', 'h', 'i'],
            ['t', 's', 'u'],
            ['a', 't', 's'],
            ['h', 'a', 'p'],
            ['t', 'i', 's'],
            ['w', 'h', 's']
        ])
    );
}

fn recover_secret(triplets: Vec<[char; 3]>) -> String {
    let mut word_vec: Vec<char> = triplets[0].into();
    println!("{:?}", &word_vec);
    for i in triplets {
        for c in i {
            if word_vec.is_empty() {
                word_vec.push(c);
            }
        }
    }
    "String".to_string()
}
