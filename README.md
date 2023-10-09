<div align="center">

# Fortune Kind

Fortune favors the kind! 
A fortune rewrite in rust, without the contentious garbage.

**README Sections:** [Options](#options) — [Installation](#installation) — [Development](#development)

[![Built with Nix](https://img.shields.io/badge/Built_With-Nix-5277C3.svg?logo=nixos&labelColor=73C3D5)](https://nixos.org)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](CODE_OF_CONDUCT.md)

[![Unit tests](https://github.com/eza-community/eza/actions/workflows/unit-tests.yml/badge.svg)](https://github.com/eza-community/eza/actions/workflows/unit-tests.yml)
![Crates.io](https://img.shields.io/crates/v/fortune-kind?link=https%3A%2F%2Fcrates.io%2Fcrates%2Feza)
![Crates.io](https://img.shields.io/crates/l/fortune-kind?link=https%3A%2F%2Fgithub.com%2Fcafkafk%2Feza%2Fblob%2Fmain%2FLICENCE)

</div>

![Usage GIF](out.gif)

> **Note** 
> This software is under active development. It's a great time to contribute!

### Try it with Nix ❄️

If you already have Nix setup with flake support, you can try out `fortune-kind` with the `nix run` command:

    nix run github:cafkafk/fortune-kind

Nix will build `fortune-kind` and run it.

If you want to pass arguments this way, use e.g. `nix run github:cafkafk/fortune-kind -- -s`.

## Installation

### Nix/NixOS ❄️


> **Note**
> Installing packages imperatively isn't idiomatic Nix, as this can lead to [many issues](https://stop-using-nix-env.privatevoid.net/).

*Imparative Installation*

For `nix profile` users:

```shell
nix profile install github:cafkafk/fortune-kind#
```

*Declarative Installation*

To add it to your `flake.nix`:

```nix
{
...
    inputs.fortune-kind.url = "github:cafkafk/fortune-kind";
...
}
```

Then, add it to your `systemPackages` wherever you prefer:

```nix
{ inputs }: {
  environment.systemPackages = [
    inputs.fortune-kind.packages.${pkgs.system}.default
  ];
}
```

<details>
    <summary> Installing From crates.io </summary>

> **Warning**
> Installing from crates.io won't set a `FORTUNE_DIR`

To install the crate:

```cargo install fortune-kind```

</details>

## Motivation

Many distributions have faced challenges with `fortune-mod` due to concerns
about its maintainer and the presence of contentious fortunes in its data
files. Instead of trying to replace `fortune-mod` or recreate a historically
accurate fortune program, our goal is to serve those who value handpicked,
randomly generated fortunes.

## Contributing

We welcome contributions! If you find any issues or have suggestions, please
open an issue. If you'd like to contribute directly, feel free to open a pull
request.

### Fortune Acceptance Process

We manually integrate fortunes from `fortune-mod`, moving them from the
`oldtunes` directory to the `fortunes` directory. Each fortune undergoes a
rigorous manual verification process. While the selection criteria can be a
topic of discussion, the final say rests with cafkafk's judgment.

For more info about contributing and the acceptance policy, please see
[EDITORIAL.md](https://github.com/cafkafk/fortune-kind/blob/main/EDITORIAL.md)
