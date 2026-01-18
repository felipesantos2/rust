
// qual o nome dessas estruturas
// module
// macro
// #[path="./controllers/home.rs"]
// mod home;

// #[path="libs/functions.rs"]
// mod lib;

// mod auth;
// mod libs;

// mod dump;

fn main() {
    
    // arrays
    let a: [i16; 9] = [10, 30, 50, 60, 70, 70, 70, 70, 60];
    // tuplas
    let b = (10, 30, 50, 60, 70, 70, 70, 70, "Felipe");
    
    println!("{}", a[0]);

    println!("{}", b.8);
    

    // let user = crate::auth::model::Usuario::create("Felipe");

    // auth::model::User::create("Felipe");

    // home::run("Felipe Pinheiro dos Santos");

    // let name: &str = home::run_with_return("Felipe Pinheiro dos Santos");

    // println!("{name}");

    // println!("{}", name);

    // println!("{}", lib::get("Felipe P. Via módulo *****get****"))

}
