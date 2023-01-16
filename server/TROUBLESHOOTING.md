cargo install diesel_cli --no-default-features --features postgres --verbose

If you get an error while setting up diesel on windows its because the compiler needs to be pointed at postgres

You need to add the following to `$HOME/.cargo/config.toml`:

```
[target.x86_64-pc-windows-msvc.pq]
rustc-link-search = ["C:\\Program Files\\PostgreSQL\\13\\lib"]
rustc-link-lib = ["libpq"]
```


https://github.com/diesel-rs/diesel/issues/2519#issuecomment-894931824

The path also needs to be exported in your environment variables in order for the diesel CLI to work

https://stackoverflow.com/questions/67358735/rust-windows-10-diesel-doesnt-work-diesel-setup-provides-no-output