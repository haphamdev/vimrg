# vimrg
Little tool to support glob for fzf-lua in nvim

# How to use:
## Build the tool
- Clone the project
- Build: if you prefer Golang: `go build`. If you prefer Rust `cargo build --release`
- Copy `vimrg` to your PATH
## Configure nvim
- Install [fzf-lua](https://github.com/ibhagwan/fzf-lua)
- Use `:lua require('fzf-lua').live_grep({ cmd = "vimrg" })` to open the fzf live_grep supporting globs

# Syntax:
## Syntax for `vimrg` as a CLI tool:
Find `myservice` in all files
```
vimrg myservice
```

Find `myservice` in test files
```
vimrg *test* > myservice
```

Find `myservice` in non-test files
```
vimrg !*test* > myservice
```

Find `myservice` in non-test controller files
```
vimrg !*test* *controller* > myservice
```

Find `myservice` not in `/test/` directories
```
vimrg !*/test/* > myservice
```

## Syntax in nvim for fzf-lua
Remove `vimrg ` from the CLI syntax
