fn main(){

    let int_a = 1;
    let mut int_b = int_a;

    println!("=== Copy ===");
    println!("int_a: {:?}", int_a);
    println!("int_b: {:?}", int_b);

    int_b = 6;
    println!("=== After int_b updated ===");
    println!("int_a: {:?}", int_a);
    println!("int_b: {:?}", int_b);


    #[derive(Debug)]
    struct Apple {
        price: f64,
        weight: f64, 
    }

    println!("");

    let apple_a = Apple{ price: 1.0, weight: 12.5};
    println!("=== Ownership ===");
    println!("apple_a: {:?}", apple_a); 
    let mut apple_b = apple_a; // Ownership move from apple_a -> apple_b 
    apple_b.price = 22.5;
    apple_b.weight = 3.4;
    let apple_c = &apple_b; // apple_c borrows apple_b
    let apple_d = &apple_b; // apple_c borrows apple_b

    println!("=== Ownership changed ===");
    //println!("apple_a: {:?}", apple_a); // uncomment and see what happen
    println!("apple_b: {:?}", apple_b);

    println!("");

    println!("=== Borrows ===");
    println!("apple_c: {:?}", apple_c);
    println!("apple_d: {:?}", apple_d);



    println!("");

    println!("=== Borrows mut ===");
    let mut apple_a = Apple{ price: 33.0, weight: 3.5}; // reuse the binding
    println!("apple_a: {:?}", apple_a);
    let apple_b = &mut apple_a;
    //let apple_c = &mut apple_a; // uncomment and see what happen
    println!("=== Borrows mut ===");
    apple_b.price = 2.0;
    println!("apple_b: {:?}", apple_b);
}
