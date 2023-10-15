
fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelvth"];
    let messages = ["And a partridge in a pear tree.", "Two turtle doves,", "Three french hens,", "Four calling birds,", "Five gold rings,", "Six geese a-laying,", "Seven swans a-swimming,", "Eight maids a-milking,", "Nine ladies dancing,", "Ten lords a-leaping,", "Eleven pipers piping,", "Twelve drummers drumming,"];

    println!("The 12 Days of Christmas Lyrics");

    for day in 0..days.len() {   
        println!("\n--------------------------------------------\n");
        println!("On the {} day of Christmas,", days[day]);
        println!("My true love gave to me");
        if day > 0 {
            for i in (0..day+1).rev() {
                println!("{}", messages[i]);
            }
        } else {
            println!("A partridge in a pear tree.");
        }
    }
    println!("\n--------------------------------------------");
}