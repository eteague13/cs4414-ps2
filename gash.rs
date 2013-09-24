use std::{io, run};
use std::str;
use std::path;





fn main() {
	static CMD_PROMPT: &'static str = "gash > ";
	let mut command_history = ~"";
	loop{
		print(CMD_PROMPT);
		let line = io::stdin().read_line();
		debug!(fmt!("line: %?", line));
		let mut argv: ~[~str] =  line.split_iter(' ').filter(|&x| x != "").transform(|x| x.to_owned()).collect();
		debug!(fmt!("argv %?", argv));

		if argv.len() > 0 {
			let program = argv.remove(0);
			debug!(fmt!("trying to launch: %?", program));
			match program {

				~"exit" => {
					return; 
				}

				~"cd" => {
					if argv.len() < 1 {
						//no args to cd change to home dir
						let home_directory: Path = std::os::homedir().unwrap();
						std::os::change_dir(~home_directory);
						let current_directory = std::os::getcwd();
						println(fmt!("%?", current_directory));
						//logging
						command_history.push_str("cd");
						command_history.push_str("\n");
					}
					else{
						//change to specified dir
						std::os::change_dir(~std::path::Path(argv[0]));
						let current_directory = std::os::getcwd();
						println(fmt!("%?", current_directory));
						//logging
						command_history.push_str("cd "+argv[0]); 
						command_history.push_str("\n");
					}
				}

				~"history" => {
					//print out history
					println(fmt!("%s", command_history));
					//history logging
					command_history.push_str("history");
					command_history.push_str("\n");
				}
				_ => {
					//history logging, we need to record all the arguments too.
					command_history.push_str(program);
					let mut argCount = 0;
					while argCount<argv.len(){
						command_history.push_str(" ");
						command_history.push_str(argv[argCount]);
						argCount+=1;
					}
					command_history.push_str("\n");
					

					//Background process
					if(argv.contains(&~"&")){
						//Remove & from args so the args are correct
						let lastIndex = argv.len()-1;
						argv.remove(lastIndex);
						//transfer ownership
						let argv = argv;
						//launch background process with a new schedule
						do std::task::spawn_sched(std::task::SingleThreaded){
							//Todo: add clasues for pipes and shit
							std::run::process_status(program, argv);
						}
					}
					//Foreground process
					else{
						//check for pipes or io redirect, assuming we have only one
						if(argv.contains(&~">>") || argv.contains(&~">") || argv.contains(&~"<") || argv.contains(&~"|") ){
							//std output
							if(argv.contains(&~">") || argv.contains(&~">>")){
								//set flags for append or truncate	
								let mut flags;
								if(argv.contains(&~">>")){
									flags = ~[io::Create, io::Append];
								}
								else{
									flags = ~[io::Create, io::Truncate];
								}
								//remove args
								let lastIndex = argv.len()-1;
								let fileName = &argv.remove(lastIndex); //remove file name
								argv.remove(lastIndex-1); //remove >

								//store output
								let process_result = run::process_output(program, argv);
								let results = str::from_bytes(process_result.output);
								let file = Path(*fileName);
								match io::file_writer(&file, flags){
									Ok(writer) => {
										writer.write_line(fmt!("%s", results)); 
									}
									Err(err) => {
										fail!(err)
									}
								}
							}
							//std input
							else if(argv.contains(&~"<")){
								//remove args
								let lastIndex = argv.len()-1;
								let fileName = &argv.remove(lastIndex);//remove file name
								argv.remove(lastIndex-1); //remove <	

								//create process and writer
								let mut process = run::Process::new(program, argv, run::ProcessOptions{
									env: None,
									dir: None,
									in_fd: None,
									out_fd: None,
									err_fd: None});
								let processWriter = process.input();
								
								//get input
								let fileReader: Result<@Reader, ~str> = std::io::file_reader(~path::Path(fileName.to_str()));
								let mut file_contents: ~[~str];
								match fileReader {
									Ok(reader) => {
										//write input to process
										processWriter.write(reader.read_whole_stream());
									}
									Err(msg) => {
										file_contents = ~[];
									}
								}
								//print out results
								let out = process.finish_with_output();
								println(fmt!("\n%s", str::from_bytes(out.output)));
							}
							//piping
							else if(argv.contains(&~"|")){

							}
						}	
						//no io redirect/piping						
						else{
							std::run::process_status(program, argv);
						}
					}

				}
			}//end match
		}//end if argv.len()>0
	}//end gash > loop
}//end main

