// Lyrics source: https://genius.com/Christmas-songs-the-twelve-days-of-christmas-lyrics

fn day_name(day: i32) -> String {
    let day = match day {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eigth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => "",
    };

    return day.to_owned();
}

fn verse_start(day: i32) -> String {
    return format!(
        "On the {} of Christmas,\nmy true love sent to me",
        day_name(day)
    );
}

fn present_for_day(day: i32) -> String {
    let present = match day {
        1 => "a Partridge in a Pear Tree",
        2 => "Two Turtle Doves",
        3 => "Three French Hens",
        4 => "Four Calling Birds",
        5 => "Five Golden Rings",
        6 => "Six Geese a Laying",
        7 => "Seven Swans a Swimming",
        8 => "Eight Maids a Milking",
        9 => "Nine Ladies Dancing",
        10 => "Ten Lords a Leaping",
        11 => "Eleven Pipers Piping",
        12 => "12 Drummers Drumming",
        _ => "",
    };
    return present.to_owned();
}

fn main() {
    println!("Twelve Days of Christmas:");

    for day in 1..13 {
        println!("{}", verse_start(day));

        // count days backwards for the correct present

        for present_count in (1..(day + 1)).rev() {
            let present = present_for_day(present_count);
            println!("{}", present);
        }
        println!("");
    }
}
