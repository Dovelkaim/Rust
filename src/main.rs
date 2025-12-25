fn main() {
    let words: String = String::from("Bonjour tout le monde  ");
    count_words(&words);

    let mut num: i32 = 5;
    mutli(&mut num);
    let mut phrase: String = String::from("Hello, ");
    add(&mut phrase);

    let mut count:i32=0;

    loop{
        count +=1;
        if count==4{
            println!("Le compteur est à : {}", count);
            break;
        }
    }
}

fn count_words(words: &String) {
    let mut count = 0;
    let trimmed = words.trim(); 
    for c in trimmed.chars() {
        if c == ' ' {
            count += 1;
        }
    }

    println!("Le nombre de mots est: {}", count + 1);
}
fn mutli(num : &mut i32){
    let result = *num *3;
    println!("Le résultat est : {}", result);
}
fn add(phrase :&mut String ){
    phrase.push_str("Rust");
    println!("{}", phrase);
}
