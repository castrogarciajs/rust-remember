mod npm;

#[derive(Debug)]
struct Pet {
    name: String,
    nickname: String,
}

impl Pet {
    fn great(&self) {
        println!("Hola {}", self.name);
    }
}

fn func_normal() -> i32 {
    return 100;
}

fn practice_for(items: &[&str; 10]) {
    for word in items {
        println!("{}", word);
    }
}

fn for_with_vec(names: &Vec<&str>) {
    for name in names.iter() {
        println!("{}", name);
    }
}

fn for_with_vec_and_struct(pets: &Vec<Pet>) {
    for pet in pets.iter() {
        println!("{} - {}", pet.name, pet.nickname);
    }
}

fn main() {
    // Variable normal
    let remember = "String reference";

    // const normal
    const PI: f32 = 3.14;

    if PI == 3.14 {
        println!("Hello PI");
    }

    // ---------------- Date primitive ------------------ //
    // boolean
    let boolean = true;

    // number
    let number = 10;

    // string
    let string = String::from("String normal");

    // Float
    let float = 9.99;

    // ---------------- Date com ----------------------- //

    let array = [10, 20, 30];

    let lola = Pet {
        name: String::from("Lola"),
        nickname: String::from("Lolancia"),
    };

    let mut n = 10;

    let y = &n;

    println!("{} Valor prestado", y);
    println!("{}", n);

    n = 12;

    println!("{}", PI);
    println!("{}", n);
    println!("{}", boolean);
    println!("{}", float);
    println!("{}", number);
    println!("{}", string);
    println!("{}", remember);
    println!("{:?}", array);
    println!("{:?}", lola);
    println!("{}", lola.name);
    println!("{}", lola.nickname);
    lola.great();
    let return_number = func_normal();

    println!("{}", return_number);
    println!("Hello, world!");
    let items = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"];

    let names = vec!["Jhon", "Camila", "Sebastian"];

    let pets = vec![
        Pet {
            name: String::from("Lola"),
            nickname: String::from("Lolancia"),
        },
        Pet {
            name: String::from("Tom"),
            nickname: String::from("Tommy"),
        },
        Pet {
            name: String::from("Jerry"),
            nickname: String::from("Jer"),
        },
    ];

    let dependecy = npm::Npm {
        name_package: String::from("react")
    };

    println!("{}", dependecy.name_package);
    practice_for(&items);
    for_with_vec(&names);
    for_with_vec_and_struct(&pets);
}
