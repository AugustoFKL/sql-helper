
## Tags

[![dependency status](https://deps.rs/repo/github/AugustoFKL/sql-helper/status.svg)](https://deps.rs/repo/github/AugustoFKL/sql-helper)

[![codecov](https://codecov.io/gh/AugustoFKL/sql-helper/branch/dev/graph/badge.svg?token=2RE9YD6RQ6)](https://codecov.io/gh/AugustoFKL/sql-helper)


# SQL Helper

This project is a POC of a SQL parser using Rust [nom](https://github.com/Geal/nom)
library. The final target is an expansible AST builder from queries, based on the [2016
ANSI standard](https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#_5_lexical_elements).


## Docs

We try really hard to have proper docs on each public structure, so projects that use
this one does not need to read the whole code to understand what's happening and what's
supported.

Rust docs are up to date, and each public structure has an explained functionality
and the current supported syntax. Although the tests are simple, as it's not really
feasible to test all the SQL structural matrix, they have a good example of what we
can guarantee is supported, and how to build it.

# Running tests

We use the stable version of Rust for testing, and there's no previous setup for
testing (no Docker or anything like that). To run the tests, simply run the `cargo test` command.





## Environment variables

No environment variables are required to run the project. However, you can set the log
level, as we intend to use [fern](https://docs.rs/fern/latest/fern).

As the logging is a WIP, we are not able to explain how to configure it to not affect
your project, but you can take a look at fern docs for more details on it.


## Contributing

Contributions are welcome!

To contribute, you simply need to follow the following steps:

1. Guarantee that your Rust is up-to-date, to guarantee compatibility between your
   local tests/linters and the CI;

2. After you finish your desired addition, feel free to open a PR to the dev branch,
   explaining what are you changing and why. The PR template should help you to do so,
   but feel free to adjust it to your needs;

3. The PR should be reviewed fast, but bigger changes should take a while to be
   reviewed. As the project has only a single maintainer, big refactors require a longer
   time, but it's guaranteed that you'll have your review. The project is open to big
   changes, if reasonable.

## Authors

- [@AugustoFKL](https://www.github.com/AugustoFKL)

