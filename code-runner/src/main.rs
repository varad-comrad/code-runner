use std::{io::Result, path};
extern crate clap;
use clap::{Parser};

extern crate serde;
use serde::{Deserialize, Serialize};

extern crate serde_json;

#[derive(Parser, Debug)]
#[command(author="varad-comrad", version="0.0.1", about="code runner in rust", long_about = None)]
struct Args{

    path: String,

    #[arg(long, default_value=".")]
    cwd: String,

    #[arg(long, default_value=None)]
    debug: bool,

    #[arg(long, default_value=None)]
    bench: bool,

    #[arg(long, default_value=None)]
    test: bool
}

fn execute_dir(path: &path::Path, cwd: &path::Path) -> Result<()>{
    Ok(())
}

fn execute_file(path: &path::Path, cwd: &path::Path) -> Result<()>{
    Ok(())
}

fn test(path: &path::Path, cwd: &path::Path) -> Result<()>{
    Ok(())
}

fn bench(path: &path::Path, cwd: &path::Path) -> Result<()>{
    Ok(())
}

fn debug(path: &path::Path, cwd: &path::Path) -> Result<()>{
    Ok(())
}

fn main() -> Result<()>{
    let cli = Args::parse();
    let (path, cwd) = (path::Path::new(&cli.path),path::Path::new(&cli.cwd));
    
    if cli.debug as i32 + cli.bench as i32 + cli.test as i32 > 1{
        // Find a better way to do this
        panic!("Only one of --debug, --bench, --test can be used at a time")
    }

    if cli.debug{
        debug(path, cwd)?;
    } else if cli.bench{
        bench(path, cwd)?;
    } else if cli.test{
        test(path, cwd)?;
    } else if path.is_file(){
        execute_file(path, cwd)?;
    } else{
        execute_dir(path, cwd)?;
    }
    Ok(())
}