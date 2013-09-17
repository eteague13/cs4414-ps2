use std::{io, run};
use std::str;


fn main() {
    static CMD_PROMPT: &'static str = "gash > ";
    let mut command_history = ~"";
    loop {
        print(CMD_PROMPT);
        let line = io::stdin().read_line();
        debug!(fmt!("line: %?", line));
        let mut argv: ~[~str] = line.split_iter(' ').filter(|&x| x != "")
                                 .transform(|x| x.to_owned()).collect();
	let mut argsOrig: ~[~str] = line.split_iter(' ').filter(|&x| x != "")
                                 .transform(|x| x.to_owned()).collect();
        debug!(fmt!("argv %?", argv));
        
        if argv.len() > 0 {
            let program = argv.remove(0);
            match program {
                ~"exit"     => {return; }
		~"cd"	    =>{
				if argv.len() < 1 {
					//let current_directory = std::os::getcwd();
     					//println(fmt!("Args less than 2: %?", current_directory));
					let home_directory: Path = std::os::homedir().unwrap();
					std::os::change_dir(~home_directory);
					let current_directory = std::os::getcwd();
     					println(fmt!("%?", current_directory));
				}else{
					std::os::change_dir(~std::path::Path(argv[0])); 
					command_history.push_str("cd"); 
					command_history.push_str("\n");
					let current_directory = std::os::getcwd();
     					println(fmt!("%?", current_directory));
				}
			 	}
		~"history"  =>{let mut counter = 0;
					println(fmt!("%s", command_history));
					
				}
                _           => {
				println(fmt!("%s", argsOrig[0]));
				let mut count = 0;
				while count < argsOrig.len() {
					if argsOrig[count].contains("<") {
						println("< is here");
					} else if argsOrig[count].contains(">"){
						println("> is here");
					} else if argsOrig[count].contains("|") {
						println("| is here");
					}
					count+=1;
				}
				
				//run::process_status(program, argv);

				}
            }
        }
    }
}
