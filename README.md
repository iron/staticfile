middleware-seed [![Build Status](https://secure.travis-ci.org/iron/iron.png?branch=master)](https://travis-ci.org/iron/middleware-seed)
====

> A [rust-empty](https://github.com/bvssvni/rust-empty) derived seed to make a simple `Middleware` for the [Iron](https://github.com/iron/iron) framework.

## Getting started

```bash
./configure   # Gets all dependencies and builds them
make lib      # Build your `Middleware's` crate
make test     # Build and run tests
make examples # Build the examples
make doc      # Build documentation using rustdoc
```

##Usage

1. Create a new empty folder for your project.
2. Copy this entire seed to the project folder.
3. Add any extra dependencies in the `configure` file.
 - `Iron` is included for you.
4. Run `make clean && ./configure` to (re)load your dependencies.
5. Type `make help`.

## Get Help

One of us (@reem, @zzmp, @theptrk, @mcreinhard) is usually on `#iron` on the
mozilla irc. Come say hi and ask any questions you might have. We are also
usually on `#rust` and `#rust-webdev`.
