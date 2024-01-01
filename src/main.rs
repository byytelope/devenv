use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use clap_complete::{generate, generate_to, Shell};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    project_name: String,
    #[arg(value_enum)]
    editor: Editor,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate shell completions
    Generate {
        shell: Option<Shell>,
        out_path: Option<String>,
    },
}

#[derive(Clone, Debug, ValueEnum)]
enum Editor {
    Vscode,
    Helix,
}

fn main() {
    let args = Cli::parse();

    println!("Name {}, Editor {:?}", &args.project_name, &args.editor);

    let mut cmd = Cli::command();
    if let Some(Commands::Generate { shell, out_path }) = args.command {
        match (shell, out_path) {
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
        };
    }
}
