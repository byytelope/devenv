use std::process::Command;

use clap::{Args, CommandFactory, Parser, ValueEnum};
use clap_complete::{generate, generate_to, Shell};
use enigo::*;
use rust_search::{FilterExt, SearchBuilder};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    // #[command(subcommand)]
    // commands: Option<Commands>,
    project_name: Option<String>,
    #[arg(value_enum)]
    editor: Option<Editor>,
    #[command(flatten)]
    comp_gen: Option<CompGen>,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = true)]
struct CompGen {
    #[arg(long, short, value_enum, value_name = "SHELL")]
    generate_completions: Option<Shell>,
    #[arg(long, short, required = false)]
    out_path: Option<String>,
}

#[derive(Clone, Debug, ValueEnum)]
enum Editor {
    Vscode,
    Helix,
}

fn main() {
    let args = Cli::parse();
    let mut cmd = Cli::command();

    println!("Name {:?}, Editor {:?}", &args.project_name, &args.editor);

    if let Some(project_name) = args.project_name {
        let mut search = SearchBuilder::default()
            .location("~/Dev")
            .search_input(project_name)
            .custom_filter(|dir| dir.metadata().unwrap().is_dir())
            .depth(2)
            .build()
            .collect::<Vec<String>>();
        search.sort();
        let path = search.first().expect("No matching directories found.");
        println!("{:?}", path);

        let mut enigo = Enigo::new();

        enigo.key_sequence(format!("cd {}", &path).as_str());
        enigo.key_click(Key::Return);

        if let Some(editor) = args.editor {
            match editor {
                Editor::Helix => {
                    enigo.key_down(Key::Control);
                    enigo.key_down(Key::Option);
                    enigo.key_down(Key::LeftArrow);
                    enigo.key_up(Key::Control);
                    enigo.key_up(Key::Option);
                    enigo.key_up(Key::LeftArrow);

                    enigo.key_sequence("hx .");
                    enigo.key_click(Key::Return);

                    enigo.key_sequence(format!("open {} -a iTerm", &path).as_str());
                    enigo.key_click(Key::Return);

                    // Command::new("open")
                    //     .args([path, "-a", "iTerm"])
                    //     .output()
                    //     .expect("Failed to open iTerm.");

                    enigo.key_down(Key::Control);
                    enigo.key_down(Key::Option);
                    enigo.key_down(Key::RightArrow);
                    enigo.key_up(Key::Control);
                    enigo.key_up(Key::Option);
                    enigo.key_up(Key::RightArrow);
                }
                Editor::Vscode => {
                    // Command::new("code")
                    //     .arg(path)
                    //     .output()
                    //     .expect("Failed to open VSCode.");

                    enigo.key_sequence("code .");
                    enigo.key_click(Key::Return);

                    enigo.key_down(Key::Control);
                    enigo.key_down(Key::Option);
                    enigo.key_down(Key::RightArrow);
                    enigo.key_up(Key::Control);
                    enigo.key_up(Key::Option);
                    enigo.key_up(Key::RightArrow);
                }
            }
        };
    };

    if let Some(CompGen {
        generate_completions,
        out_path,
    }) = args.comp_gen
    {
        match (generate_completions, out_path) {
            (Some(shell), Some(out_path)) => {
                // Generate shell comp to out_path
                generate_to(shell, &mut cmd, "devenv", out_path).expect("Compgen failed.");
            }
            (Some(shell), None) => {
                // Generate shell comp to stdout
                generate(shell, &mut cmd, "devenv", &mut std::io::stdout());
            }
            (None, Some(out_path)) => {
                // Generate zsh comp to out_path
                generate_to(Shell::Zsh, &mut cmd, "devenv", out_path).expect("Compgen failed.");
            }
            (None, None) => {
                // Generate zsh comp to ~/.config/zsh/completion
                generate_to(
                    Shell::Zsh,
                    &mut cmd,
                    "devenv",
                    "/Users/mohamedshadhaan/.config/zsh/completion",
                )
                .expect("Compgen failed.");
            }
        }
    };
}
