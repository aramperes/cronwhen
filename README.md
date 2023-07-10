# cronwhen

Command-line utility for printing the next iteration of configured [crons](https://en.wikipedia.org/wiki/Cron).

[![crates.io](https://img.shields.io/crates/v/cronwhen.svg)](https://crates.io/crates/cronwhen)
[![MIT licensed](https://img.shields.io/crates/l/cronwhen.svg)](./LICENSE)
[![Build status](https://github.com/aramperes/cronwhen/actions/workflows/build.yml/badge.svg)](https://github.com/aramperes/cronwhen/actions)
[![Latest Release](https://img.shields.io/github/v/tag/aramperes/cronwhen?label=release)](https://github.com/aramperes/cronwhen/releases/latest)

## Example

```
$ cronwhen

0 * * * * echo 'hello world'
Next iteration in 11 min 26 sec (2023-07-10 02:00:00 UTC)

0 12 */2 * * echo 'good morning'
Next iteration in 34 hours 11 min 26 sec (2023-07-11 12:00:00 UTC)
```

## Download

cronwhen is available to install from [crates.io](https://crates.io/crates/cronwhen) with a stable Rust version:

```shell
cargo install cronwhen
```

## Work in Progress

This is currently a prototype. I would like to expand this utility with the following features:

- Pass crontab from `stdin`
- Windows support (with `schtasks`)
- Extended cron formats (seconds, `@daily`, etc.)

## License

MIT License. See `LICENSE` for details. Copyright &copy; 2023 Aram Peres.
