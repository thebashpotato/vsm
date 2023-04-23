<div align="center">
  <h1>Vim Session Manager</h1>
  <img src="assets/vsm-usage.gif">
</div>
<br>
<div align="center">
  <img alt="Crates.io" src="https://img.shields.io/badge/standard--readme-OK-green.svg?style=flat-square">
  <img alt="Crates.io" src="https://img.shields.io/crates/v/vsm?style=flat-square">
  <img alt="docs.rs" src="https://img.shields.io/docsrs/vsm?style=flat-square">
  <img alt="Crates.io" src="https://img.shields.io/crates/d/vsm?style=flat-square">
  <br>
  <p>A <b>BLAZINGLY FAST</b>, simple, interactive command line session manager</p>
</div>

## Table of Contents

- [Background](#background)
- [Install](#install)
- [Usage](#usage)
- [Development](#development)
- [Maintainers](#maintainers)
- [Contributing](#contributing)
- [License](#license)

## Background

If you use vim or neovim on a daily basis and work in large codebases, it is probably not uncommon for you to have 10+ tabs open at a time,
with various splits. Once you close this vim session the layout is lost to the ethers. the `mksession` command in vim(neovim) can save you,
thus saving the session to a directory, promising to return you to your work exactly how you left it.
However, the problem is most of us accrue many of these session files scattered about, personally I have `41` vim session files,
easily loading them, rememembering the context of each one, and removing stale sessions becomes a hassle. enter vsm (Vim Session Manager),
it allows you to list, open, and remove sessions files, either interactively or by name. It also manages different variations of vim, and allows
you to switch between them. For example you may be a `neovide` user rather than `neovim`, vsm currently supports `vim, neovim, neovide and gvim`.

### Similar projects

Currently I have only found [one other project](https://github.com/xolox/vim-session) which allows more indepth session file management.
However it didn't suit my desires, as it is written in vim script, suffers from useless feature rot, and relies heavily on the xolox misc plugin,
together loading in over `1200` or so lines of vim script just to manage some session files, it also seems to be unmaintained as the
last change recorded was July 6, 2014. Because of this I wrote `vsm`, it seemed like a better solution to make an external cli program and off load
work from vim and shorten my plugin list as much as possible.

## Install

### Install with cargo

`cargo install vsm`

### Building from source

> Installs the optimized binary to `$HOME/.local/bin`

```bash
git clone https://github.com/thebashpotato/vsm
cargo install just
just install
```

## Usage

### Set up

> An environement variable `VIM_SESSIONS` is expected on the system,
> if it is not defined `vsm` will default to `~/.config/vim_sessions` when it looks
> for your session files. Below are 2 examples for settings the variable in different shells.
> You can set the path where ever you want.

- bash/zsh `export VIM_SESSIONS="$HOME/.config/vim_sessions"`

- fish `set -Ux VIM_SESSIONS "$HOME/.config/vim_sessions"`

### Create session files easier (in vim)

> `vsm` can load, list and remove session files, but it can't create them.
> That is the job of `vim`.

> Add the below snippet to your `.vimrc` or `init.vim` to make creating
> new session files much easier. Now in `normal mode` you can press `mk`
> to quickly save your session file.

```vim
if isdirectory(expand($VIM_SESSIONS))
  " Create a new sesion file (must give the file a unique name
  nnoremap mk :mksession $VIM_SESSIONS/
  " Overwrite an existing sessioon file with your current layout
  nnoremap mo :mksession! $VIM_SESSIONS/
else
  nnoremap mk :echo "VIM_SESSIONS directory does not exist"<CR>
  nnoremap mo :echo "VIM_SESSIONS directory does not exist"<CR>
endif
```

## Development

Only requires `just` to bootstrap all tools and configuration.

```bash
cargo install just
just init # setup repo, install hooks and all required tools
```

To run:

```bash
just run
```

To test:

```bash
just test
```

Before committing your work:

```bash
just pre-commit
```

To see all available commands:

```bash
just list
```

## Maintainers

[@thebashpotato](https://github.com/thebashpotato)

## Contributing

PRs accepted.

Small note: If editing the README, please conform to the [standard-readme](https://github.com/RichardLitt/standard-readme) specification.

## License

This project is licensed under:

- [APGL © 2022 Matt Williams](LICENSE)
