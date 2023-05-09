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

## Config file

You will need to create a `42-cli.toml` file in each of your projects. This file will contain the following information:

```toml
# The name of the project
name = "pipex"

[scripts]
# The install script (optional)
install = [{ cmd = "make" }]

# The test script (optional)
test = [
  { cmd = "git clone xxx tester" },
  { cmd = "make m", dir = "tester" },
  { cmd = "rm -rf tester" }
]

# The cleanup script (optional)
clean = [{ cmd = "make fclean" }]

# The lint script (optional)
lint = [{ cmd = "norminette ." }]
```

Each script is defined by an array of commands. They will be executed in the order they are defined.


## Commands

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
