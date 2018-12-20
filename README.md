# machine-ip

Get your local ip address in Rust, using `hostname -I`
under the hood, may not work in older versions of windows

```toml
machine-ip = "0.1"
```

then

```rust
extern crate machine_ip;

let ip = machine_ip::get().unwrap();
println!("local ip address: {:?}", ip.to_string());
```

## Warning
This crate just wraps the `hostname` comman, so it only works on unix based OS, and this doesn't work on windows.

## License

MIT
