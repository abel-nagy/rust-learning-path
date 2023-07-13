fn main() {
    // Specify the data type "char"
    let character_1: char = 'S';
    let character_2: char = 'f';
   
    // Complier interprets a single item in quotes as the "char" data type
    let smiley_face = '😃';

    // Complier interprets a series of items in quotes as a "str" data type and creates a "&str" reference
    let string_1 = "miley ";

    // Specify the data type "str" with the reference syntax "&str"
    let string_2: &str = "ace";

    println!("{} is a {}{}{}{}.", smiley_face, character_1, string_1, character_2, string_2);
}