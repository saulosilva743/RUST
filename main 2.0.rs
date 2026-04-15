use std::io;

struct Produto {
    id: u32,
    nome: String,
    preco: f32,
    quantidade: u32,
}

fn ler_texto(msg: &str) -> String {
    let mut entrada = String::new();
    println!("{}", msg);
    io::stdin().read_line(&mut entrada).expect("Erro ao ler");
    entrada.trim().to_string()
}

fn ler_u32(msg: &str) -> u32 {
    let entrada = ler_texto(msg);
    entrada.parse::<u32>().unwrap_or(0)
}

fn ler_f32(msg: &str) -> f32 {
    let entrada = ler_texto(msg);
    entrada.parse::<f32>().unwrap_or(0.0)
}

fn inserir_produto(produtos: &mut Vec<Produto>) {
    let id = ler_u32("Digite o ID do produto:");
    let nome = ler_texto("Digite o nome do produto:");
    let preco = ler_f32("Digite o preço do produto:");
    let quantidade = ler_u32("Digite a quantidade do produto:");

    let produto = Produto {
        id,
        nome,
        preco,
        quantidade,
    };

    produtos.push(produto);
    println!("Produto cadastrado com sucesso!");
}

fn listar_produtos(produtos: &Vec<Produto>) {
    if produtos.is_empty() {
        println!("Nenhum produto cadastrado.");
    } else {
        for produto in produtos {
            println!("--------------------------");
            println!("ID: {}", produto.id);
            println!("Nome: {}", produto.nome);
            println!("Preço: R$ {:.2}", produto.preco);
            println!("Quantidade: {}", produto.quantidade);
        }
    }
}

fn editar_produto(produtos: &mut Vec<Produto>) {
    let id = ler_u32("Digite o ID do produto que deseja editar:");

    for produto in produtos {
        if produto.id == id {
            produto.nome = ler_texto("Novo nome:");
            produto.preco = ler_f32("Novo preço:");
            produto.quantidade = ler_u32("Nova quantidade:");
            println!("Produto editado com sucesso!");
            return;
        }
    }

    println!("Produto não encontrado.");
}

fn remover_produto(produtos: &mut Vec<Produto>) {
    let id = ler_u32("Digite o ID do produto que deseja remover:");

    for i in 0..produtos.len() {
        if produtos[i].id == id {
            produtos.remove(i);
            println!("Produto removido com sucesso!");
            return;
        }
    }

    println!("Produto não encontrado.");
}

fn buscar_produto(produtos: &Vec<Produto>) {
    let termo = ler_texto("Digite o nome do produto que deseja buscar:").to_lowercase();
    let mut encontrado = false;

    for produto in produtos {
        if produto.nome.to_lowercase().contains(&termo) {
            println!("--------------------------");
            println!("ID: {}", produto.id);
            println!("Nome: {}", produto.nome);
            println!("Preço: R$ {:.2}", produto.preco);
            println!("Quantidade: {}", produto.quantidade);
            encontrado = true;
        }
    }

    if !encontrado {
        println!("Nenhum produto encontrado.");
    }
}

fn main() {
    let mut produtos: Vec<Produto> = Vec::new();

    loop {
        println!("\n===== CATÁLOGO DE PRODUTOS =====");
        println!("1 - Inserir produto");
        println!("2 - Listar produtos");
        println!("3 - Editar produto");
        println!("4 - Remover produto");
        println!("5 - Buscar produto");
        println!("0 - Sair");

        let opcao = ler_u32("Escolha uma opção:");

        match opcao {
            1 => inserir_produto(&mut produtos),
            2 => listar_produtos(&produtos),
            3 => editar_produto(&mut produtos),
            4 => remover_produto(&mut produtos),
            5 => buscar_produto(&produtos),
            0 => {
                println!("Encerrando o programa...");
                break;
            }
            _ => println!("Opção inválida."),
        }
    }
}