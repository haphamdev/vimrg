# vimrg
Little tool to support glob for fzf-lua in nvim

How to use:
- Clone the project
- `go build`
- Copy `vimrg` to your PATH

Syntax:
Find `myservice` in all files
```
vimrg -- myservice
```

Find `myservice` in test files
```
vimrg -- *test* > myservice
```

Find `myservice` in non-test files
```
vimrg -- !*test* > myservice
```

Find `myservice` in non-test controller files
```
vimrg -- !*test* *controller* > myservice
```

Find `myservice` not in `/test/` directories
```
vimrg -- !*/test/* > myservice
```
