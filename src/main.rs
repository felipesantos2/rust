
// qual o nome dessas estruturas
// // module
#[path="./controllers/home.rs"]
mod home;

fn main() {

    home::run("Felipe Pinheiro dos Santos");
 
    let name: &str = home::run_with_return("Felipe Pinheiro dos Santos");
 
    println!("{name}");
 
    println!("{}", name);

}
