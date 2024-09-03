/* You probably know the "like" system from Facebook and other pages. People can "like" blog posts, 
pictures or other items. We want to create the text that should be displayed next to such an item.

Implement the function which takes an array containing the names of people that like an item. 
It must return the display text as shown in the examples:

[]                                -->  "no one likes this"
["Peter"]                         -->  "Peter likes this"
["Jacob", "Alex"]                 -->  "Jacob and Alex like this"
["Max", "John", "Mark"]           -->  "Max, John and Mark like this"
["Alex", "Jacob", "Mark", "Max"]  -->  "Alex, Jacob and 2 others like this" */

pub fn likes (names: &[&str]) -> String {
    if names.len() == 0 {
        return format!("no one likes this");
    } else if names.len() == 1 {
        return format!("{} likes this", names[0]);
    } else if names.len() == 2 {
        return format!("{} and {} like this", names[0], names[1]);
    } else if names.len() == 3 {
        return format!("{}, {} and {} like this", names[0], names[1], names[2]);
    } else {
        let total: usize = names.len() - 2;
        return format!("{}, {} and {} others like this", names[0], names[1], total);
    }
}
