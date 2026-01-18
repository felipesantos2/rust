
// qual o nome dessas estruturas
// module
// macro
#[path="./controllers/home.rs"]
mod home;

#[path="libs/functions.rs"]
mod lib;

mod auth;
mod libs;

fn main() {

    // let user = crate::auth::model::Usuario::create("Felipe");

    auth::model::User::create("Felipe");

    home::run("Felipe Pinheiro dos Santos");

    let name: &str = home::run_with_return("Felipe Pinheiro dos Santos");

    println!("{name}");

    println!("{}", name);

    println!("{}", lib::get("Felipe P. Via módulo *****get****"))

}
