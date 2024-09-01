mod helper;

use helper::structs::TupleE;
fn main() {
    println!("Hello, world!");
    let a_number: i32 = 10;
    let mut a_word: &str = "Ten"; // mut -> para poder mutar una variable. Por defecto son inmutables

    //todo!("Make my first code");
    println!("The first variable is {}, the second is {}", a_number, a_word);
    println!("Space");
    a_word = "Eleven";
    println!("The first variable is {a_number}, the second is {a_word}");

    // VARIABLE SHADOWING
    let shadow_num: i32 = 5;

    let shadow_num: i32 = shadow_num + 5;

    let shadow_num: i32 = shadow_num * 2;

    println!("The number is: {shadow_num}");

    //CADENAS
    let character_1: char = 'S';
    let character_2: char = 'f';

    let smiley_face: char = '😃';

    let string_1: &str = "miley ";
    let string_2: &str = "ace";

    println!("{smiley_face} is a {character_1}{string_1}{character_2}{string_2}");
    let text_concat: String = character_1.to_string() + string_1 + &character_2.to_string() + string_2;

    let text_concat_ref: &str = &text_concat;
    print_cadena(text_concat_ref);

    // tambien puedo:
    print_cadena(&text_concat);

    tuplas()
}

fn print_cadena (cadena: &str) {
    println!("{cadena}");
}

fn tuplas () {
    // IMPORTO TupleE DE LA CARPETA HELPERS
    //struct TupleE (char, i32, bool, char);
    let tuple_e: TupleE = ('E', 5i32, true, 'A');
    // Desestructuración
    let (a, b, c) = tuple_e;
    println!("{a}, {b}, {c}");

    
    
}
