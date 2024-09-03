//mod helper;
mod ejercicios;

use crate::ejercicios::finabonacci::finabonacci;
use crate::ejercicios::who_likes_it::likes;

fn main() {
    let names = ["Pedro", "Marcos", "Gustavo", "Julio", "Pedro", "Marcos", "Gustavo", "Julio"];
    likes(&names);
    let resultado = finabonacci(6);
    println!("El resultado es: {resultado}");
    /* println!("Hello, world!");
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

    tuplas();
    loops(); */
}

/* fn print_cadena (cadena: &str) {
    println!("{cadena}");
} */

/* fn tuplas () {
    // IMPORTO TupleE DE LA CARPETA HELPERS
    //struct TupleE (char, i32, bool, char);
    let tuple_e: TupleE = TupleE('E', 5i32, true, 'A');
    // Desestructuración
    let TupleE (a, b, c, d) = tuple_e;
    println!("{a}, {b}, {c}, {d}");

    let array1 = ["Nelson", "Christian", "Marco"];
    
    for i in array1 {
        println!("El valor de i es: {i}");
    }


} */

/* fn loops () {
    for i in 1..20 {
        if i%2 == 0 {
            println!("This number is par: {i}")
        } else {
            println!("This number isn't par: {i}")
        }
    }

} */