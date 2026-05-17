use colored::Colorize;
use std::process::Command;
use std::{env, fs};

pub fn create_project(name: &str) {
    let import_tailwindcss = r#"@import "tailwindcss";"#;
    let files_svg_default = ["favicon.svg", "icons.svg"];
    println!("{}", format!("{name} is being created...").blue().bold());
    const ICON: &[u8] = include_bytes!("../assets/WebFast.svg");
    Command::new("npm")
        .args([
            "create",
            "vite@latest",
            name,
            "--",
            "--template",
            "react-ts",
            "-y",
            "--no-immediate",
        ])
        .status()
        .expect(&"Crate vite project error".to_string().red().bold());

    println!("{}", format!("Project {name} created with react + vite")
            .blue()
            .bold()
    );


    env::set_current_dir(name).expect("Error to change folder");
    println!("{}", format!("\nnow install tailwindcss and config...").yellow());

    Command::new("npm")
        .args(["install", "tailwindcss", "@tailwindcss/vite"])
        .status()
        .expect("Error to install tailwindcss/vite");
    fs::write(
        "vite.config.ts",
        r#"
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'

export default defineConfig({
  plugins: [
   react(), tailwindcss(),
  ],
})
"#,
    )
    .expect("Error to created config in vite.config.ts");
    fs::write("src/index.css", import_tailwindcss)
        .expect("Error to import tailwindcss in index.css");
    fs::remove_file("src/App.css").expect("Error to remove app.css");
    fs::remove_dir_all("src/assets").expect("Error to remove assets");
    fs::write("public/WebFast.svg", ICON).expect("Error to import logo");
    fs::write(
        "index.html",
        r#"
<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <link rel="shortcut icon" href="WebFast.svg" type="image/x-icon" />
    <title>WebFast</title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="/src/main.tsx"></script>
  </body>
</html>

    "#,
    )
    .expect("Error to refactor index.html");
    fs::write("src/App.tsx", r#"
export default function App() {
return (
<section className="bg-neutral-900 flex items-center justify-center flex-col w-screen h-screen gap-1">
    <div className=" flex items-end justify-center gap-1">
       <h1 className=" text-2xl font-bold text-white ">
        Project configured with 
       </h1>
        <a href="https://github.com/Rhuann01/webFast.git"  target="_blank">
          <img className="w-10" src="WebFast.svg" alt="LogoProject" />
        </a>
     </div>
      <a href="https://github.com/Rhuann01" target="_blank" className=" text-gray-400 hover:text-white">
        made by @Rhuann01
      </a>
    </section>
  );
}

    "#).expect("Error mounting default app");
    for file in files_svg_default {
        fs::remove_file(format!("public/{file}")).expect("Error remove svgs default");
    }

    Command::new("npm")
        .arg("install")
        .status()
        .expect("Error install npm modules");
    println!(
        "{}",
        format!("\nTHE PROJECT IS READY, RUNNING... ")
            .green()
            .bold()
    );
    Command::new("npm")
        .args(["run", "dev"])
        .status()
        .expect("Error install npm modules");
}
