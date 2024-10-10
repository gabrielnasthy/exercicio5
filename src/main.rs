// Disciplina : Linguagem e Lógica de Programação
// Professor : Alan Furukawa
// Descrição : Aqui você descreve o que o programa faz! (função)
// Autor(a) : Gabriel Aguiar Rocha
// Data atual : 04/10/2021
use std::io;

fn ler()->i32{
    let mut input=String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse().unwrap()
}

fn main() {
    println!("digite um numero inteiro");
    let mut num1=ler();
    println!("digite outro numero inteiro");
    let mut num2=ler();

    if num1 > num2{
        let mut diferenca = num1 - num2;
            println!("da diferença entre {} e {} é {}",num1 ,num2,diferenca);
    }else{
     let mut diferenca = num2 - num1;
        println!("da diferença entre {} e {} é {}",num2 ,num1,diferenca);
    }


}
