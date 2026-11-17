fn main() {
    looping();
    vector();
    turple();
    createstring();
}


fn looping() {
    for i in 0..6 {
        if i == 3 {
            continue;
        }
        println!("{}", i);
    }
    println!("___________________________________________");
}

fn vector() {
    let mut cars: Vec<i32> = Vec::new();
    cars.push(12);

    while let Some(tup) = cars.pop() {
        println!("{tup}");
    }
    println!("___________________________________________");
}

fn turple() {
    let person = ("Jenny", 45, false);
    let (name, age, active) = person;

    println!("{name}");
    println!("{age}");
    println!("{active}");

    println!("___________________________________________");
}

fn createstring () {
    let text1 :String = String::from("This is string using String::from()");
    let text2 : String = "This is string using to_string".to_string();
    let mut textpush : String = String::from("Starting - ");
    textpush.push_str("Ending");
    textpush.push('!');

    println!("{text1}");
    println!("{text2}");
    println!("{textpush}");

}
