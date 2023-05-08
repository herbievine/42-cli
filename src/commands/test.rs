use crate::{
    args::TestArgs,
    lib::{config::Config, process},
};

pub fn exec(_: &TestArgs, config: &Config) {
    // let TestArgs {
    //     program,
    //     arguments,
    //     make,
    //     ..
    // } = args;

    if config.scripts.install.is_some() {
        println!("Running install script...");

        for script in config.scripts.install.as_ref().unwrap() {
            match process::exec_commands(
                vec![&script.cmd],
                script.dir.as_ref().unwrap_or(&String::from(".")),
            ) {
                Ok(_) => (),
                Err(e) => {
                    println!("{}", e.message);
                    std::process::exit(1);
                }
            }
        }
    } else {
        println!("No install script found.");
    }

    if config.scripts.test.is_some() {
        println!("Running test script...");
        for script in config.scripts.test.as_ref().unwrap() {
            match process::exec_commands(
                vec![&script.cmd],
                script.dir.as_ref().unwrap_or(&String::from(".")),
            ) {
                Ok(_) => (),
                Err(e) => {
                    println!("{}", e.message);
                    std::process::exit(1);
                }
            }
        }
    } else {
        println!("No test script found.");
    }

    if config.scripts.clean.is_some() {
        println!("Running clean script...");
        for script in config.scripts.clean.as_ref().unwrap() {
            match process::exec_commands(
                vec![&script.cmd],
                script.dir.as_ref().unwrap_or(&String::from(".")),
            ) {
                Ok(_) => (),
                Err(e) => {
                    println!("{}", e.message);
                    std::process::exit(1);
                }
            }
        }
    } else {
        println!("No clean script found.");
    }

    // let program_path = match fs::canonicalize(&PathBuf::from(config.program.as_ref().unwrap())) {
    //     Ok(path) => path,
    //     Err(e) => {
    //         println!("{:?}", e);
    //         std::process::exit(1);
    //     }
    // };
    // let mut program_command = String::from(program_path.to_str().unwrap());
    // program_command.push(' ');
    // program_command.push_str(&arguments.join(" "));

    // match process::exec_commands(vec![&program_command], project_directory) {
    //     Ok(_) => println!("Exit code: 0"),
    //     Err(e) => {
    //         println!("{:?}", e.message);
    //         println!("Exit code: 1");
    //         std::process::exit(1);
    //     }
    // }
}
