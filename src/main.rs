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

    // while &word.len() < &lett.len() {

    // let sorted: Vec<_> = vec_o_schrift.iter().map(|x| x.back.len()).collect();

    for i in vec_o_schrift.iter() {
        if i.back.is_empty() {
            word.insert(0, i.letter);
        }
        if i.front.is_empty() {
            word.push(i.letter);
        }
    }
    let mut counter = 0;
    while word.len() < lett.len() - 2 {
        for i in vec_o_schrift.iter() {
            if i.back.contains(&word[counter]) && i.front.contains(&word[counter + 1]) {
                word.insert(counter + 1, i.letter);
                counter += 1;
            }
        }
    }
    println!("{:?}", &word);
    while word.len() < lett.len() {
        counter = 4;
        for i in vec_o_schrift.iter() {
            if i.back.contains(&word[counter]) && i.front.contains(&word[counter + 1]) {
                word.insert(counter + 1, i.letter);
                counter += 1;
            }
        }
    }
    println!("{:?}", &word);
    // counter += 1;

    // counter = 3;
    // for i in vec_o_schrift.iter() {
    //     if i.back.contains(&word[counter]) && i.front.contains(&word[counter + 1]) {
    //         word.insert(counter + 1, i.letter);
    //         counter += 1;
    //     }
    // }

    // counter = 3;
    // for i in vec_o_schrift.iter() {
    //     if i.back.contains(&word[counter]) && i.front.contains(&word[counter + 1]) {
    //         word.insert(counter + 1, i.letter);
    //         counter += 1;
    //     }
    // }
    println!("{:?}", &word);
    let word2: String = word.into_iter().collect();
    word2
}
