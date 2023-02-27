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
    // println!("{:?}", &word_vec);
    for i in triplets[1..].iter() {
        println!("{:?}", i);
        for c in i.to_vec().iter() {
            if i.position(|&c| c == 0) {
                word_vec.push(c);
            }
        }
    }
    "String".to_string()
}
