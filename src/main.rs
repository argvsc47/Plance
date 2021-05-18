mod cli;
mod delete;
mod new;

fn main() {
    let args = cli::Cli::get();
    match args.action {
        cli::Subcommand::NEW => new::create_project(&args),
        cli::Subcommand::DELETE => delete::delete_project(&args), //delete::remove_project(&args),
        cli::Subcommand::DEFAULT => (),
    };
}