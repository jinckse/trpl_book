fn main() {
    let day_item: &[&str; 12] = &["a partridge in a pear tree",
    "Two turtle doves", 
    "Three French hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming"];

    let  day_line_prefix: String = String::from("On the ");

    let  day_line_suffix: String = String::from(" day of christmas \
        my true love sent to me");

    // Iterate through all twelve days, adding a line for each
    // one read before it
    for item in 0..day_item.len() {
        println!("{}{}{}", day_line_prefix, item + 1, day_line_suffix);
        for line in (0..=item).rev() {
            println!("{}", day_item[line]);
        }
    }
}
