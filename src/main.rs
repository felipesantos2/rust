fn main() {
    // println!("Hello, world!");
    let age: i32 = 22;
    // i64 i31 i16 i8
    // String & str
    //

    println!("Minha idade é: {age}");

    println!("--------------------------");

    // 1. Criação (String Growable/Heap)
    let texto = String::from("  Olá, Rust Developer!  ");

    // 2. Limpeza (Trim)
    // Retorna um &str (slice), não aloca nova memória na heap (zero-copy)
    let limpo = texto.trim();
    
    println!("Limpo: '{}'", limpo); // "Olá, Rust Developer!"
    println!("--------------------------");

    // 3. Substituição (Replace)
    // Java/PHP: str_replace / .replace
    // Rust: Cria uma NOVA String na memória
    let novo_texto = limpo.replace("Rust", "Python");
    println!("Replace: {}", novo_texto); // "Olá, Python Developer!"
    println!("--------------------------");

    // 4. Split (O "Explode" do PHP) - ATENÇÃO AQUI
    // Diferente do PHP, ele não retorna um Array, retorna um "Iterator" (preguiçoso).
    // Para ter um "Array" (Vector), você precisa usar .collect()
    let partes: Vec<&str> = limpo.split(", ").collect();
    println!("Primeira parte: {}", partes[0]); // "Olá"
    println!("--------------------------");

    // 5. Contém (Contains)
    if limpo.contains("Developer") {
        println!("É um dev!");
    }
    
    println!("--------------------------");
    
    // 6. Concatenação (Melhor que usar +)
    // A macro format! é igual ao sprintf ou f-string.
    // É o jeito mais limpo de juntar variáveis sem brigar com o Borrow Checker.
    let mensagem = format!("{} começou a estudar hoje.", "Felipe");
    println!("{}", mensagem);

}
