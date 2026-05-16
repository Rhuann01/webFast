    use clap::{Parser,Subcommand};
    mod templates;

    #[derive(Parser)]
    #[command(name = "Fast web",version = "1.0", about = "project to build fast web applications, like react-vite + tailwindcss already configured and with future additions")]
    struct Args {
        #[command(subcommand)]
        trigger: Comandos,
    }

    #[derive(Subcommand)]
    enum Comandos {
        Create {nome: String},
        Delete {nome: String},
    }
    fn main() {
    let args = Args::parse();   

    match args.trigger {
        Comandos::Create { nome } => {
            print!("{nome} is being created");
            templates::vite_react_tailwindcss::create_project(&nome);
        }, 
        Comandos::Delete { nome } => println!("teste {nome}")
    }
    }