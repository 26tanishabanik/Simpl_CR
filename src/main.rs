use clap::{Parser};
use curl::easy::{Easy, List};
use anyhow::Result;
use std::collections::HashMap;
use std::io::{stdout, Read, Write};
use std::env;

#[derive(clap::Parser, Debug)]
#[clap(author = "Simpl_CR", version="1.0.0", about="Container Runtime")]
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




fn main() -> Result<()>{
    // let args = CliArgs::parse();
    let mut handle = Easy::new();
    let mut list = List::new();
    let token: &str = &env::var("TOKEN").expect("$TOKEN is not set");;
    let header = format!("{}{}", "Authorization: Bearer ", token);
    list.append(&header);
    handle.verbose(true)?;
    handle.url("https://registry-1.docker.io/v2/ustclug/centos/tags/list")?;
    handle.http_headers(list)?;
    handle.write_function(|data| {
        println!("{:?}", data);
        stdout().write_all(data).unwrap();
        Ok(data.len())
    })?;
    handle.perform()?;
    // println!("Hello, world!");
    Ok(())
}
