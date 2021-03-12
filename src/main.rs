fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return index;
        }
    }
    s.len()
}

fn first_word_after(s: &String, l: &String) -> usize {
    let bytes = s.as_bytes();
    let letter = l.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == letter[0] {
            return index;
        }
    }

    s.len()
}

fn main() {
    let s = String::from("Hello world");
    let index = first_word(&s);
    println!("Index {}", index);

    // String Slices
    // INDEX	VALUE
    // 0	H
    // 1	e
    // 2	l
    // 3	l
    // 4	o
    // 5	
    // 6	w
    // 7	o
    // 8	r
    // 9	l
    // 10	d

    // When you want to get a part of a string, 
    // you should use a range within brackets by specifying. For example;
    let hello = String::from("Hello world");
    let start_index = 0;
    let last_index = 5;
    let hello = &hello[start_index..last_index];
    println!("hello {}", hello);

    // Let's say we want to get a string after the specified letter.
    let s = String::from("Hello world");
    let letter = String::from(" ");

    let index = first_word_after(&s, &letter);

    println!("The string is {}", &s[index..]);

    // That's what we want :) You don't have to specify ending or starting index always. You can use like that;
    /*
        &your_var[start_index..];
        &your_var[..end_index];
        &your_var[..];

        The first example will get string starting the index you specified.
        The second example will get string until the index you specified.
        The last example will return the whole string.
    */

    /*
        Other Type Slices
    */

    let number_array = [1, 5, 7, 9, 11, 13, 15, 17, 19, 21];
    println!("Numbers: {:?}", &number_array[1..3]);
    // It will return Numbers: [5, 7].

    // So, according to this example, we can create other types of slices.
    let an_array = [true, true, false, true, false, false, true, false];
    println!("Array values: {:?}", &an_array[1..3]);
}
