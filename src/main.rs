#[derive(Debug, Clone)]
struct Schrift {
    letter: char,
    back: Vec<char>,
    front: Vec<char>,
}

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
    let mut lett: Vec<char> = Vec::new();
    let mut vec_o_schrift: Vec<Schrift> = Vec::new();

    for i in triplets {
        for c in i {
            if !lett.contains(&c) {
                lett.push(c);
            }
        }
    }

    println!("{:?}", lett);

    for i in lett {}

    // for i in triplets[1..].iter() {
    //     println!("{:?}", i);
    //     for c in i.to_vec().iter() {
    //         if i.position(|&c| c == 0) {
    //             word_vec.push(c);
    //         }
    //     }
    // }
    "String".to_string()
}

#[derive(Debug, Clone)]
struct Schrift {
    letter: char,
    back: Vec<char>,
    front: Vec<char>,
}
