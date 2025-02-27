# CS 552 Operating Systems Project 2

This is a simple shell program written in Rust.

Note: The library used to handle signal processing was officially recommended by the rust developers but is 
currently failing many of its CI/CD tests on GitHub. It does however seem to be functioning well enough for the
purposes of this shell.

Steps to configure, build, run, and test the project.

## Building
To build a shared rust library:
```bash
make
```

## Testing

```bash
make check
```

## Clean

```bash
make clean
```

## Install Dependencies

If needed, the rust build system (rustup and cargo) can be installed/updated by running the following command:

```bash
sudo make install-deps
```
