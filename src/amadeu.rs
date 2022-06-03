use std::io;
use std::time::Instant;

pub fn entry_point(){
    loop {
        println!("Escolha o teste a executar");
        println!("1 - Numeros primos");
        println!("0 - Sair");
        println!("Opcao: ");


        let mut opcao = String::new();

        io::stdin()
            .read_line(&mut opcao)
            .expect("Falha ao ler entrada.");
        
        let opcao: u8 = opcao.trim().parse().expect("Por favor, digite um numero!");
        match opcao {
            //Testes do Amadeu
            1 => prime_numbers(),
            0 => {
                println!("Voltando...");
                break;
            },
            _ => println!("Opcao invalida...")
        }
    }
}

fn prime_numbers(){
    let mut primes = String::new();

        println!("Quantos numeros primos vc quer ver?");
        io::stdin()
            .read_line(&mut primes)
            .expect("Falha ao ler entrada.");
        
        let primes: u32 = primes.trim().parse().expect("Por favor, digite um numero maior que zero!");

        println!("Estes sao os {} primeiros numeros primos:", primes);

        let now = Instant::now();

        if primes > 0 {
            let mut count = 1;
            let mut numbers = 0;
            while numbers < primes {
                if is_prime(count) {
                    numbers += 1;
                    println!("\t{}", count);
                }
                count += 1;
            }
            println!("Analisados {} numeros para chegar a esses {} primos em {} segundos", count - 1, primes, now.elapsed().as_secs_f64());
             
        } else {
            println!("Entre um número maior que zero.")
        }
}

fn is_prime(number: u32) -> bool {
    let mut times = 0;
    for i in 2..number {
        if number % i == 0{
            times += 1;
        }
        if times > 0{
            return false;
        }
    }
    true
}