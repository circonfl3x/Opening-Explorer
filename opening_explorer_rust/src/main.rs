use std::fs::File;
use std::io::*;
use std::path::*;
use std::env;
mod tsv;

fn main()
{

    let args:Vec<_> = env::args().collect();
    let mut fpath = PathBuf::new();
    let mut code_search = String::new();
    let mut name_search = String::new();
    let mut sequence_search = String::new();

    if args.len() < 2
            {
                fpath = input();

            }
    else
    {
        for i in args.iter()
            {
                if i == "--path"
                {
if args.iter().position(|y| y == "--path").unwrap() <= args.len()-1
                            {
                                fpath = PathBuf::from(&args[args.iter().position(|y| y == "--path").unwrap()+1]);
                            }
                    else {
                        panic!("Supplied --path, but no path provided!");
                    }
                    if ! Path::exists(&fpath)
                        {
                            panic!("Supplied a filepath that doesn't exist");

                        }
                    }

                if i == "--name"
                        {
                            let index = args.iter().position(|y| y == "--name").unwrap_or(args.len());
                            if index < args.len() {
                                name_search = (&args[index + 1]).to_string();
                            } else {
                                panic!("Declared --name and not provided it.");
                            }

                        }
                if i == "--code"
                        {
                            let index = args.iter().position(|y| y == "--code").unwrap_or(args.len());
                            if index < args.len() {
                                code_search = (&args[index + 1]).to_string();
                            } else {
                                panic!("Declared --code and not provided it.");
                            }

                        }
                if i == "--pgn"
                        {
                            let index = args.iter().position(|y| y == "--pgn").unwrap_or(args.len());
                            if index < args.len() {
                                sequence_search = (&args[index + 1]).to_string();
                            } else {
                                panic!("Declared --pgn and not provided it.");
                            }

                        }

            }
        }


    let mut code:String;
    let mut name:String;
    let mut seq:String;



    if let Ok(lines) = read(fpath)
    {

        for line in lines
                {
                    if let Ok(l) = line
                            {
                                (code,name,seq) = tsv::Tsv::break_down_line(&l);
                                let mut print = true;
                                if  ! code_search.is_empty()
                                    {
                                        if code.contains(&code_search)
                                                {

                                                }else
                                                {
                                                    print = false;
                                                }
                                    }
                                if ! name_search.is_empty()
                                {
                                    if name.contains(&name_search){}
                                    else{print=false;}

                                }
                                if ! sequence_search.is_empty()
                                {
                                    if seq.contains(&sequence_search){}
                                    else{print=false;}
                                }
                                if print
                                        {
                                            println!("Name: {name}\nCode: {code}\nSequence: {seq}");
                                        }
                            }
                }
    }
}


fn read<R> (fpath: R) -> Result<Lines<BufReader<File>>>
where R : AsRef<Path>,
{
    let file = File::open(fpath)?;
    Ok(BufReader::new(file).lines())
}

fn input() ->  PathBuf
{
    let mut string = String::new();
    println!("Filepath not supplied! Please supply filepath: ");
    stdin()
    .read_line(&mut string)
    .expect("Cannot get input from stdin!");

    let fpath = PathBuf::from(string.trim());

    fpath

}