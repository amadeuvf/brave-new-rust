use std::io;

//incluindo amadeu.rs
mod amadeu;

fn main() {
    loop {
        println!("Escolha o teste a executar");
        println!("1 - Testes do Amadeu");
        println!("0 - Sair");
        println!("Opcao: ");


        let mut opcao = String::new();

        io::stdin()
            .read_line(&mut opcao)
            .expect("Falha ao ler entrada.");
        
        let opcao: u8 = opcao.trim().parse().expect("Por favor, digite um numero!");
        match opcao {
            //Testes do Amadeu
            1 => amadeu::entry_point(),
            0 => {
                println!("Encerrando...");
                break;
            },
            _ => println!("Opcao invalida...")
        }
    }
}
