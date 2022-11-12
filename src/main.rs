use clap::{Parser};


#[derive(clap::Parser, Debug)]
#[clap(author = "Tanisha Banik", version="1.0.0", about="Software Developer")]
struct CliArgs {
    // #[clap(subcommand)]
    #[clap(short, long, value_parser, help="pull image from a public image repository", default_value_t=false)]
    pull: bool, // pull image from a public image repository
    #[clap(subcommand)]
    operations: Operations,
    #[clap(long, value_parser, help="remove a container")]
    rm: String, // remove a container
    #[clap(long, value_parser, help="list the container", default_value_t=false)]
    list_cnt: bool, // list the containers
    #[clap(long, value_parser, help="list all the images", default_value_t=false)]
    list_img: bool, // list all the images
    #[clap(short, long, value_parser, help="prints the version", default_value="1.0.0")]
    version: String,
}

#[derive(clap::Subcommand, Debug)]
pub enum Operations {
    // run a container
    Run{
        name: String,
    },
    // start an exited container
    Start{
        name: String,
    },
}

fn main() {
    let args = CliArgs::parse();

    println!("Hello, world!");
}
