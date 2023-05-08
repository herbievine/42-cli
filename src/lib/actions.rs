use crate::lib::process;
use colored::Colorize;

use super::{
    config::{Command, Config},
    process::ExecError,
};

#[derive(PartialEq, Debug)]
enum MlxAction {
    Install,
    Remove,
}

fn run_action(config: &Config, commands: &Vec<Command>, step: &str) -> Result<(), ExecError> {
    if config.prepend_path.is_none() {
        println!(
            "{} {} {}",
            "Running".bright_green().bold(),
            step.bright_green().bold(),
            "script...".bright_green().bold()
        );
    }

    for script in commands {
        let dir = match &config.prepend_path {
            Some(path) => format!(
                "{}/{}",
                path,
                script.dir.as_ref().unwrap_or(&String::from("."))
            ),
            None => script
                .dir
                .as_ref()
                .unwrap_or(&String::from("."))
                .to_string(),
        };

        match process::exec_command(&script.cmd, dir) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

fn run_children(
    config: &Config,
    f: &dyn Fn(&Config) -> Result<(), ExecError>,
    step: &str,
) -> Result<(), ExecError> {
    if let Some(children) = &config.children {
        for child_path in children {
            println!(
                "{} {} {} {}{}",
                "Running".bright_green().bold(),
                step.bright_green().bold(),
                "script in".bright_green().bold(),
                child_path.bright_magenta().bold(),
                "...".bright_green().bold()
            );

            let child_config = Config::new(child_path);
            f(&child_config)?;
        }
    }

    Ok(())
}

fn manage_mlx(config: &Config, script: &Command, action: MlxAction) -> Result<(), ExecError> {
    let msg = if action == MlxAction::Install {
        "Installing MLX"
    } else {
        "Removing MLX"
    };

    if config.prepend_path.is_some() {
        println!(
            "{} {} {}{}",
            msg.bright_green().bold(),
            "script in".bright_green().bold(),
            config
                .prepend_path
                .as_ref()
                .unwrap()
                .bright_magenta()
                .bold(),
            "...".bright_green().bold()
        );
    } else {
        println!("{}", msg);
    };

    if action == MlxAction::Install {
        let mlx_dir = if script.mlx_dir.is_some() {
            script.mlx_dir.as_ref().unwrap()
        } else {
            "mlx"
        };

        let mlx_dir = if config.prepend_path.is_some() {
            format!("{}/{}", config.prepend_path.as_ref().unwrap(), mlx_dir)
        } else {
            mlx_dir.to_string()
        };

        let command = format!(
            "git clone https://github.com/42Paris/minilibx-linux.git {}",
            mlx_dir
        );

        process::exec_command(&command, ".")?;
    } else {
        let mlx_dir = if script.mlx_dir.is_some() {
            script.mlx_dir.as_ref().unwrap()
        } else {
            "mlx"
        };

        let mlx_dir = if config.prepend_path.is_some() {
            format!("{}/{}", config.prepend_path.as_ref().unwrap(), mlx_dir)
        } else {
            mlx_dir.to_string()
        };

        let command = format!("rm -rf {}", mlx_dir);

        process::exec_command(&command, ".")?;
    }

    Ok(())
}

pub fn build(config: &Config) -> Result<(), ExecError> {
    if let Some(script) = &config.scripts.build {
        if let Some(mlx) = script.mlx {
            if mlx {
                manage_mlx(config, script, MlxAction::Install)?;
            }
        }
        run_action(config, &vec![script.clone()], "build")?;
    } else {
        run_children(config, &build, "build")?;
    }

    Ok(())
}

pub fn test(config: &Config) -> Result<(), ExecError> {
    if let Some(scripts) = &config.scripts.test {
        run_action(config, &scripts, "test")?;
    } else {
        run_children(config, &test, "test")?;
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
                manage_mlx(config, script, MlxAction::Remove)?;
            }
        }
    } else {
        run_children(config, &clean, "clean")?;
    }

    Ok(())
}

pub fn norm(config: &Config) -> Result<(), ExecError> {
    if let Some(script) = &config.scripts.norm {
        run_action(config, &vec![script.clone()], "norm")?;
    } else {
        run_children(config, &norm, "norm")?;
    }

    Ok(())
}
