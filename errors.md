Error: ```warning: spurious network error (1 tries remaining): [6] Couldn't resolve host name (Could not resolve host: static.crates.io)
error: failed to download from `https://static.crates.io/crates/clap/4.5.20/download`

Caused by:
  [6] Couldn't resolve host name (Could not resolve host: static.crates.io)```


[source.crates-io]
index = "https://github.com/rust-lang/crates.io-index" # I set the source to the official github index

[http]
proxy = "" # leaving this as a blank string resets proxy to default