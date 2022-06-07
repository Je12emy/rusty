fn main() {
    const TRHREE_HOURS_IN_SECCONDS: u32 = 60 * 60 * 3;

    let _x = 5;
    println!("The value of x is {}", _x);

    let _x = _x + 1;
    
    {
        let _x = _x + 1;
        println!("The value of x is {}", _x);
    }

    println!("The value of x is {}", _x);
}
