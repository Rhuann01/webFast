use std::{env, fs};
use std::process::Command;
use colored::Colorize;

use crate::Comandos;

pub fn createProject(name: &str) {
    let import_tailwindcss = r#"@import "tailwindcss";"#;

    println!("{}", format!("{name} is being created...").blue().bold());

    Command::new("npm").args(["create", "vite@latest", name, "--template", "react-ts", "-y", "--no-immediate"])
    .status().expect(&"Crate vite project error".to_string().red().bold());

    println!("{}", format!("Project {name} created with react + vite now config tailwindcss..").blue().bold());

    env::set_current_dir(name).expect("Error to change folder");

    

    fs::write("vite.config.ts", r#"
import { defineConfig } from 'vite'
import tailwindcss from '@tailwindcss/vite'

export default defineConfig({
  plugins: [
   react(), tailwindcss(),
  ],
})
"#).expect("Error to created config in vite.config.ts");
    fs::write("src/index.css", import_tailwindcss).expect("Error to import tailwindcss in index.css");
    fs::remove_file("src/App.css").expect("Error to remove app.css");
    fs::remove_dir_all("src/assets").expect("Error to remove assets");
    
}