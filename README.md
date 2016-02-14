# wire-drawer

A little wire drawer made with SDL in Rust.

![screenshot](https://github.com/GuillaumeGomez/wire-drawer/blob/master/resources/screenshot.png)

If you want to build it, please install SDL 1.x and then just use cargo:

```Shell
> cargo build --release
```

And to run it:

```Shell
> cargo run
```

It will load the default map. If you want to specify a map, pass it as a an argument like this:

```Shell
cargo run -- maps/42.map
```
