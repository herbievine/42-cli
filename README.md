<pre>
        :::     ::::::::             ::::::::  :::        :::::::::::
      :+:     :+:    :+:           :+:    :+: :+:            :+:
    +:+ +:+        +:+            +:+        +:+            +:+
  +#+  +:+      +#+              +#+        +#+            +#+
+#+#+#+#+#+  +#+                +#+        +#+            +#+
     #+#   #+#                 #+#    #+# #+#            #+#
    ###  ##########            ########  ########## ###########
</pre>

This is a simple CLI tool to manage, test and run your 42 projects.

# Installation

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [Norminette](https://github.com/42School/norminette) (Optional)

## Install

### From GitHub

```bash
cargo install --git https://github.com/herbievine/42-cli.git

# Add the following to your .bashrc or .zshrc to persist the alias
alias ft="~/.cargo/bin/fourtytwo-cli"
```

# Usage

When you have a config file present in your project, 42 CLI will attempt to run your command. There are a few speciad attributes which come with these commands, like installing MLX and executing a command in a specific directory.

## Config file

You will need to create a `42-cli.toml` file in each of your projects. This file will look something like this:

```toml
# The name of the project
name = "push_swap"

[scripts]
build = { cmd = "make" }
run = [
  { cmd = "./push_swap 8 2 -2 0 2147483647" },
  { cmd = "./push_swap invalid :(" },
]
```

## Examples

Here, it is used in the `so_long` project. Each script is defined by an object (except `run` and `test` which contain an array of objects) with special keys to automate your development.

```toml
name = "so_long"

[scripts]
build = { cmd = "make", mlx = true, mlx_dir = "minilibx" }
run = [{ cmd = "./so_long maps/small.ber" }]
clean = { cmd = "make fclean", mlx = true, mlx_dir = "minilibx" }
lint = { cmd = "norminette ." }
```

Here you have 4 scripts: 
- `build`, which will run `make` after installing MLX in the `minilibx` directory.
- `run`, which will execute after running `build` (if present), and run `./so_long maps/small.ber` (`clean` can also be run at the end. See API Reference below)
- `clean`, which will run `make fclean`, and then delete MLX.
- `lint`, which simply runs norminette on your code.

Here is also an example with `ft_printf`, which contains a test suite to execute:

```toml
name = "ft_printf"

[scripts]
build = { cmd = "make" }
test = [
	{ cmd = "git clone https://github.com/Tripouille/printfTester.git tester" },
	{ cmd = "make m", dir = "tester" },
	{ cmd = "rm -rf tester" }
]
clean = { cmd = "make fclean" }
lint = { cmd = "norminette ." }
```

In this example, you will notice there is a `test` script, which executes the defined scripts one-by-one. The `test` script will execute the `build` script, and run the `clean` script after finishing (only if they are defined).

## Usage in CI/CD

If you want to automate your deployements, or simply run commands on each of your projects in on command, you can create a config file in the head of your directory, like so:

```
42/
├─ libft/
│  ├─ 42-cli.toml
├─ ft_printf/
│  ├─ 42-cli.toml
├─ get_next_line/
│  ├─ 42-cli.toml
├─ 42-cli.toml
```

The root `42-cli.toml` file should contain a key called `projects`, which all your projects defined as strings.

```toml
name = "42"
projects = [
	"libft",
	"ft_printf",
	"get_next_line"
]

# Leave this empty
[scripts]
```

42 CLI will attempt to run your command in each of the defined projects, if it's not present, it will just skip it.

> Note: the `run` command cannot be run as root.

## API Reference

### `help`

Displays the help menu.

```bash
ft help
```

### `build`

Build your project with the defined `build` scripts in the `42-cli.toml` file.

```bash
ft build
```

#### Options

- `-h, --help`: Print help.

### `run`

Run your project with the defined commands in the `42-cli.toml` file. It will first run the `build` script if defined.

```bash
ft run
```

#### Options

- `-h, --help`: Print help.
- `-c, --clean`: Run the clean script defined in the `42-cli.toml` file.

### `test`

Run the test script defined in the `42-cli.toml` file. It will first run the `build` script if defined, and run the `clean` script once the test is done (even if it fails).

```bash
fourtytwo-cli test
```

#### Options

- `-h, --help`: Print help.

### `clean`

Run the clean script defined in the `42-cli.toml` file.

```bash
fourtytwo-cli clean
```

#### Options

- `-h, --help`: Print help.

### `lint`

Run the lint script defined in the `42-cli.toml` file.

```bash
fourtytwo-cli lint
```

#### Options

- `-h, --help`: Print help.

# Support/Contributing

If you've found a bug or have a feature request, please open an issue. If you'd like to contribute, please open a pull request.
