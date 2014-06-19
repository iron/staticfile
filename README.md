staticfile [![Build Status](https://secure.travis-ci.org/iron/staticfile.png?branch=master)](https://travis-ci.org/iron/staticfile)
====

> Static file-serving implemented as Middleware for the Iron framework.

## Installation

Add this to your Iron server as a dependency in `configure`:

```bash
updateDependency 'https://github.com/iron/static.git' 'static' "/target/$TARGET/lib"
```

## Example Usage

See `examples`.

To run the example:

- `./configure && make examples doc && ./examples/doc`.

- Browse to `localhost:3000`.

## License

MIT
