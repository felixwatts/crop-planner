use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct ParamsPrint {
    #[structopt(short, long)]
    pub bed: Option<std::string::String>,

    #[structopt(short, long)]
    pub week: Option<usize>
}

#[derive(Debug, StructOpt)]
pub struct ParamsTasks {
    #[structopt(short, long)]
    pub week: usize
}

#[derive(Debug, StructOpt)]
pub enum Cmd {
    #[structopt(name = "init", about = "Initialize a new harvest repository in the current directory")]
    Init,

    #[structopt(name = "solve", about = "Design a crop plan based on the current parameter settings")]
    Solve,

    #[structopt(name = "reset", about = "Drop the current crop plan")]
    Reset,

    #[structopt(name = "print", about = "Print the current crop plan to the console")]
    Print(ParamsPrint),
}

#[derive(StructOpt, Debug)]
#[structopt(name = "harvest", about = "A crop planning tool for market gardeners")]
pub struct Cli {
    #[structopt(subcommand)]
    pub command: Cmd,

    #[structopt(short, long, default_value = "params.json", parse(from_os_str))]
    params: std::path::PathBuf,
}

