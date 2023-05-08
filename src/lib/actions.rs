use crate::lib::process;
use colored::Colorize;

use super::{
    config::{Command, Config},
    process::ExecError,
};

#[derive(PartialEq)]
enum MlxAction {
    Install,
    Remove,
}

fn run_action(config: &Config, commands: &Vec<Command>, step: &str) -> Result<(), ExecError> {
    if config.scripts.build.is_some() {
        println!(
            "{} {} {}",
            "Running".bright_green().bold(),
            step.bright_green().bold(),
            "script...".bright_green().bold()
        );

        for script in commands {
            match process::exec_command(
                &script.cmd,
                script.dir.as_ref().unwrap_or(&String::from(".")),
                script.pipe.as_ref().map(|x| &**x),
            ) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
    } else {
        println!(
            "{} {} {}",
            "No".yellow().bold(),
            step.yellow().bold(),
            "script found.".yellow().bold()
        );
    }

    Ok(())
}

fn manage_mlx(config: &Config, action: MlxAction) -> Result<(), ExecError> {
    if let Some(script) = &config.scripts.build {
        if action == MlxAction::Install {
            println!("{}", "Installing MLX...".bright_green().bold());

            let mlx_dir = if script.mlx_dir.is_some() {
                script.mlx_dir.as_ref().unwrap()
            } else {
                "mlx"
            };
            let command = format!(
                "git clone https://github.com/42Paris/minilibx-linux.git {}",
                mlx_dir
            );
            process::exec_command(&command, ".", None)?;
            return Ok(());
        } else if action == MlxAction::Remove {
            println!("{}", "Removing MLX...".bright_green().bold());

            let mlx_dir = if script.mlx_dir.is_some() {
                script.mlx_dir.as_ref().unwrap()
            } else {
                "mlx"
            };
            let command = format!("rm -rf {}", mlx_dir);
            process::exec_command(&command, ".", None)?;
            return Ok(());
        }
    }

    Ok(())
}

pub fn build(config: &Config) -> Result<(), ExecError> {
    if let Some(script) = &config.scripts.build {
        if let Some(mlx) = script.mlx {
            if mlx {
                manage_mlx(config, MlxAction::Install)?;
            }
        }
        run_action(config, &vec![script.clone()], "build")?;
    }

    Ok(())
}

pub fn test(config: &Config) -> Result<(), ExecError> {
    if let Some(scripts) = &config.scripts.test {
        run_action(config, &scripts, "test")?;
    }

    Ok(())
}

pub fn run(config: &Config) -> Result<(), ExecError> {
    if let Some(scripts) = &config.scripts.run {
        run_action(config, &scripts, "run")?;
    }

    Ok(())
}

pub fn clean(config: &Config) -> Result<(), ExecError> {
    if let Some(script) = &config.scripts.clean {
        run_action(config, &vec![script.clone()], "clean")?;
        if let Some(mlx) = script.mlx {
            if mlx {
                manage_mlx(config, MlxAction::Remove)?;
            }
        }
    }

    Ok(())
}

pub fn norm(config: &Config) -> Result<(), ExecError> {
    if let Some(script) = &config.scripts.norm {
        run_action(config, &vec![script.clone()], "norm")?;
    }

    Ok(())
}
