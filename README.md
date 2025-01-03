Created by Seth Baker


I created the rust-lyrics repository to test the capabilities of using git 
on Ubuntu to push commits directly to the repository via the command line.

The code is a simple assignment from "The Rust Programming Language" book.
The assignment is:

- Print the lyrics to the Christmas carol "The Twelve Days of Christmas",
taking advantage of the repetition in the song


I accomplished this by creating two functions, print_carol and print_fill,
and two constant arrays GIFTS and SPELLED, with GIFTS containing the lyrics
associated with the gifts of each day and SPELLED containing the 12 ordinal
numbers (first, second, etc.) to help make the code look cleaner and act more
independently of extremely specific order based print statements.

print_carol() is an extremely simple function, made up of a for loop from 0
to 12 (in this case, the "i" would take values of 0 to 11, which are the
indexes in GIFTS and SPELLED), that passes in the value of i to print_fill().

print_fill() is a little more complex, it takes in a parameter of usize,
creates mutable variables count, y, and z. count is a "copy" of the integer
passed into the parameter, y is the entry in GIFTS at the index of the 
parameter, and z being the entry in SPELLED at the index of the parameter.
It calls a println! function that will ouput the first line of every verse,
taking in the value of z to specify which ordinal number is to be used. Then,
it calls a println! function that will always output "my true love gave to me"
regardless of the parameter. This is because the second line in every verse
is constant. Then, it has an if statement for punctuation. In every verse 
except the first, the punctuation after the first line consists of a comma, 
so I wrote a simple if/else statement that just calls a println! function
containing the variable y with a comma or a period after dependent upon the
value of the parameter.
Then, I have a while loop for while count > 1. It decrements count, resets
y to the entry at the index of count in GIFTS, and then calls println! which
outputs the value of y with a comma after it. After this while, I have an
additional if statement for if x > 0 (meaning that the lines being printed
are not the first verse of the song) meant to further implement the correct
punctuation for the lyrics of the song. In every verse except the very last
verse of the carol, the "and a partridge in a pear tree" line is followed by
a period. In the last verse, the line is followed by an exclamation point.
So, I used a nested if/else statement within the if x > 0 statement to ensure
the correct punctuation is being used. After the if statement, I call a 
simple println! function containing the empty string, which will ouptut a
blank line of output.

Finally, the main function simply has a println! function call containing the
point of the program, a line space, and then calls print_carol().

