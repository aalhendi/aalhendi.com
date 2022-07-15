---
setup: |
  import Layout from '../../layouts/BlogPost.astro'
title: PDE Setup
publishDate: 08 July 2022
description: How to get up and running on a new machine.
---

There's been a few times I have had to work on a VM or SSH into a server to do some work. I wanted to lower some of the friction associated with getting ready and comfortable in a new, foreign environment. This guide might also be useful for those who wanted to clone my setup but found the installation process a little tedious.

Overview:

1. Build [Neovim](https://github.com/neovim/neovim/wiki/Building-Neovim) from source or a [release package](https://github.com/neovim/neovim/releases/).
2. Install dependencies for the setup. These are needed for the Neovim plugins to run smoothly

   ```bash
   sudo apt-get install ripgrep fd-find tmux curl python3 python3-pip
   ```

3. Install [pnpm](https://pnpm.io/installation) and set up the Node env via `pnpm env use --global lts`

   ```bash
   curl -fsSL https://get.pnpm.io/install.sh | sh -
   ```

4. Clone nvim [dotfiles](https://www.github.com/aalhendi/dotfiles) + whatever else you might want.

   ```bash
   git clone https://www.github.com/aalhendi/dotfiles && mkdir --parents ~/.config/nvim/ && mv dotfiles/.config/nvim/* ~/.config/nvim/
   ```

5. Install global packages for LSP support

   ```bash
   pnpm add -g typescript graphql vscode-langservers-extracted dockerfile-language-server-nodejs pyright graphql-language-service-cli @prisma/language-server @tailwindcss/language-server typescript-language-server yaml-language-server neovim
   ```

   ```bash
   pip3 install 'pynvim'
   ```

6. Install [Rust](https://www.rust-lang.org/tools/install)

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

7. Install [rust-analyzer](https://rust-analyzer.github.io/manual.html#installation)

   ```bash
   rustup component add rust-src
   ```

8. Install other packages [clangd](https://clangd.llvm.org/installation.html), [Marksman](https://github.com/artempyanykh/marksman). If clagd-12 is not available, try clagd-9 or clangd-8

   ```bash
   sudo apt-get install clangd-12
   ```

9. Run `:PackerSync` from within Neovim
