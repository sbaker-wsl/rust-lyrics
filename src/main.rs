// GOAL: print the lyrics to the christmas carol the twelve days of 
// christmas, taking advantage of the repetetiveness of the song

const GIFTS: [&str; 12] = ["a partridge in a pear tree", "Two turtle doves",
            "Three French hens", "Four calling birds", "Five golden rings",
            "Six geese a-laying", "Seven swans a-swimming",
            "Eight maids a-milking", "Nine ladies dancing",
            "Ten lords a-leaping", "Eleven pipers piping",
            "Twelve drummers drumming"];

const SPELLED: [&str; 12] = ["first", "second", "third", "fourth", "fifth",
                            "sixth", "seventh", "eighth", "ninth", "tenth",
                            "eleventh", "twelfth"];

fn main() {
    println!("Lyrics for The Twelve Days of Christmas:");
    println!("");
    print_carol();
}

fn print_carol() {
    for i in 0..12 {
        print_fill(i);
    }
}

fn print_fill(x: usize) {
    let mut count = x;
    let mut y = GIFTS[count];
    let z = SPELLED[count];
    println!("On the {z} day of Christmas,");
    println!("my true love gave to me");

    if x == 0 { 
        println!("A partridge in a pear tree.");
    }

    else {
        println!("{y},");
    }

    while count > 1 {
        count -= 1;
        y = GIFTS[count];
        println!("{y},");
    }

    if x > 0 {
        y = GIFTS[0];
        if x == 11 {
            println!("And {y}!");
        }

        else {
            println!("And {y}.");
        }

    }
    println!("");
}
