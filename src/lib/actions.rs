use std::path::Path;

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
    if config.run_in.is_none() {
        println!(
            "{} {} {}",
            "Running".bright_green().bold(),
            step.bright_green().bold(),
            "script...".bright_green().bold()
        );
    }

    for script in commands {
        let dir = match &config.run_in {
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

fn run_projects(
    config: &Config,
    f: &dyn Fn(&Config) -> Result<(), ExecError>,
    step: &str,
) -> Result<(), ExecError> {
    if let Some(projects) = &config.projects {
        for project_path in projects {
            println!(
                "{} {} {} {}{}",
                "Running".bright_green().bold(),
                step.bright_green().bold(),
                "script in".bright_green().bold(),
                project_path.bright_magenta().bold(),
                "...".bright_green().bold()
            );

            let project_config = Config::new(project_path);
            f(&project_config)?;
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

    if config.run_in.is_some() {
        println!(
            "{} {} {}{}",
            msg.bright_green().bold(),
            "script in".bright_green().bold(),
            config.run_in.as_ref().unwrap().bright_magenta().bold(),
            "...".bright_green().bold()
        );
    } else {
        println!(
            "{}{}",
            msg.bright_green().bold(),
            "...".bright_green().bold()
        );
    };

    if action == MlxAction::Install {
        let mlx_dir = if script.mlx_dir.is_some() {
            script.mlx_dir.as_ref().unwrap()
        } else {
            "mlx"
        };

        let mlx_dir = if config.run_in.is_some() {
            format!("{}/{}", config.run_in.as_ref().unwrap(), mlx_dir)
        } else {
            mlx_dir.to_string()
        };

        if Path::new(&mlx_dir).exists() {
            return Ok(());
        }

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

        let mlx_dir = if config.run_in.is_some() {
            format!("{}/{}", config.run_in.as_ref().unwrap(), mlx_dir)
        } else {
            mlx_dir.to_string()
        };

        if !Path::new(&mlx_dir).exists() {
            return Ok(());
        }

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
        run_projects(config, &build, "build")?;
    }

    Ok(())
}

pub fn test(config: &Config) -> Result<(), ExecError> {
    if let Some(scripts) = &config.scripts.test {
        run_action(config, &scripts, "test")?;
    } else {
        run_projects(config, &test, "test")?;
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
        run_projects(config, &clean, "clean")?;
    }

    Ok(())
}

pub fn lint(config: &Config) -> Result<(), ExecError> {
    if let Some(script) = &config.scripts.lint {
        run_action(config, &vec![script.clone()], "lint")?;
    } else {
        run_projects(config, &lint, "lint")?;
    }

    Ok(())
}
