// GOAL: print the lyrics to the christmas carol the twelve days of 
// christmas, taking advantage of the repetetiveness of the song

const GIFTS: [&str; 12] = ["a partridge in a pear tree", "two turtle doves",
            "three french hens", "four calling birds", "five golden rings",
            "six geese-a-laying", "seven swans a-swimming",
            "eight maids a-milking", "nine ladies dancing",
            "ten lords a-leaping", "eleven pipers piping",
            "twelve drummers drumming"];

const SPELLED: [&str; 12] = ["first", "second", "third", "fourth", "fifth",
                            "sixth", "seventh", "eighth", "ninth", "tenth",
                            "eleventh", "twelfth"];

fn main() {
    println!("Lyrics for twelve days of Christmas.");
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
    println!("On the {z} day of christmas,");
    println!("my true love gave to me");

    if (x == 0) { 
        println!("{y}.");
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
        if (x == 11) {
            println!("and {y}!");
        }

        else {
            println!("and {y}.");
        }

    }
    println!("");
}
