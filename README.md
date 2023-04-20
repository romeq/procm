# procm
Show program's resource usage using [sysinfo](https://crates.io/crates/sysinfo) crate.


## Installation
```sh
cargo install --git https://github.com/romeq/procm procm
```

## Usage
```sh
# Show pid
procm show <pid>

# Monitor pid
watch -ct procm show <pid>
```
