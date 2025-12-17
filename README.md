# XR-PALS Middleware

The **XR-PALS middleware** server for the **Loco Positioning System (LPS)** handles positioning data uploads from the mixed-reality app.

## Build

```bash
$ cargo clean
$ cargo build --release
```

## Run the Server

```bash
$ ./target/release/xrpals-lps-server 
```

Uploaded files are stored in the `uploads/` directory.

## Test Upload

You can test the middleware with `curl`:

```shell
$ curl -X POST http://192.168.178.59:8080/upload -F "file=@./test/file.yaml"
```

## Project Structure

```
src/        # Rust source code
uploads/    # Uploaded files directory
Cargo.toml  # Project manifest
```

## Contributing

We welcome contributions! Please:

- Fork the repo
- Open a pull request
- Follow Rust’s style guidelines (`cargo fmt`, `cargo clippy`)

## License

Apache 2.0 License © 2025 XR-PALS Developers.  
(Main Developer: Victor Victor)  
