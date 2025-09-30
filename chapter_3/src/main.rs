fn c2f(c: f32) -> f32 {
    c * 1.8 + 32.0
}

fn fib(n: u32) -> u64 {
    let mut a = 1;
    let mut b = 1;

    for _ in 0..n-1 {
        let tmp = a;
        a = b;
        b = tmp + a;
    }

    a
}

fn xmas() {
    let ordinals = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"
    ];

    let lines = [
        "A partridge in a pear tree",
        "Two turtle doves and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for i in 0..12 {
        let ordinal = ordinals[i];
        println!("On the {ordinal} day of Christmas, my true love sent to me");
        for j in (0..=i).rev() {
            let line = lines[j];
            println!("{line}");
        }
    }
}

fn main() {
    // celsius to fahrenheit
    let c = 32.5;
    let f = c2f(c);
    println!("{c}C = {f}F");

    for case in 1..10 {
        let fib = fib(case);
        println!("fib num {case} = {fib}");
    }

    xmas();
}
