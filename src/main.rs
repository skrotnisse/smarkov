mod markovchainreader;
mod markovchain;

use markovchainreader::MarkovChainReader;

use std::env;

fn print_usage()
{
    println!("Usage: smarkov <inputfile> [OPTIONS]");
    println!("");
    println!("Description:");
    println!("  Processes an UTF-8 encoded text file and outputs a Markov Chain matrix on JSON format.");
    println!("");
    println!("Arguments:");
    println!("  <inputfile>   An UTF-8 coded text file.");
    println!("Options:");
    println!("  --minimize    Produces minimized JSON output");
    println!("");
}

fn parse_command_arguments(filename: &mut String, minimize_flag: &mut bool) -> Result<(), &'static str> 
{
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3
    {
        return Err("Invalid number of arguments.");
    }

    if args.len() == 3
    {
        if args[2] == "--minimize"
        {
            *minimize_flag = true;
        }
        else
        {
            return Err("Invalid option.");
        }
    }

    *filename = args[1].to_string();

    Ok(())
}

fn main() -> Result<(), String>
{
    let mut filename : String = String::new();
    let mut minimize_flag : bool = false;

    match parse_command_arguments(&mut filename, &mut minimize_flag)
    {
        Ok(_) => {},
        Err(e) => {
            print_usage();
            return Err(e.into())
        }
    }

    match MarkovChainReader::from_file(&filename)
    {
        Ok(mc) => 
        {
            let json_output = (*mc).to_json(minimize_flag);
            println!("{}", *json_output);
        }
        Err(e) => 
        {
            print_usage();
            return Err(format!("{}", e));
        }
    }

    Ok(())
}