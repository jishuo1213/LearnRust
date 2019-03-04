fn main() {
    let mut number = 3;

    let result = while number != 0 {
        println!("{}!", number);

        number = number - 1
    };

    println!("LIFTOFF!!!{:?}",result);
}
