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

    for i in triplets.clone() {
        for c in i {
            if !lett.contains(&c) {
                lett.push(c);
            }
        }
    }

    println!("{:?}", lett);

    for i in lett.clone() {
        vec_o_schrift.push(Schrift {
            letter: i,
            back: Vec::new(),
            front: Vec::new(),
        });
        // println!("{:#?}", vec_o_schrift);
    }

    for i in vec_o_schrift.iter_mut() {
        for j in triplets.clone().iter_mut() {
            if j.contains(&i.letter) {
                let index = j.iter().position(|&l| &l == &i.letter).unwrap();
                for l in j.clone().iter() {
                    if j.iter_mut().position(|x| x == &l.clone()).unwrap() > index {
                        if !i.front.contains(&l) {
                            i.front.push(l.clone());
                        }
                    }
                    if j.iter_mut().position(|x| x == &l.clone()).unwrap() < index {
                        if !i.back.contains(&l) {
                            i.back.push(l.clone());
                        }
                    }
                }
            }
        }
    }

    let mut word: Vec<char> = Vec::new();
    let mut counter = 0;

    while &word.len() < &lett.len() {
        for i in vec_o_schrift.iter() {
            if i.back.len() == counter {
                word.push(i.letter);
                counter += 1;
            }
            // if i.back.is_empty() {
            //     word.push(i.letter);
            // }
        }
        println!("{:?}", &word);
    }

    println!("{:#?}", vec_o_schrift);
    println!("{:#?}", word);
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
