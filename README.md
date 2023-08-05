<p align="center"><img src="https://consumet.org/images/consumetlogo.png" width="175"/></p>

<h1 align="center"> consumet.rs </h1>

consumet.rs is a crates.io library which provides high-level APIs to get information about several entertainment mediums like books, movies, comics, anime, manga, etc.

Checkout the original [`consumet.ts`](https://github.com/consumet/consumet.ts/)

Credits to the beautiful people who wrote the original api consumet

[![](https://contrib.rocks/image?repo=consumet/consumet.ts)](https://github.com/consumet/consumet.ts/graphs/contributors)

<p align="center">
    <a href="https://crates.io/crates/consumet-api-rs">
        <img src="https://img.shields.io/crates/v/consumet-api-rs.svg">
    </a>
    <a href="https://discord.gg/qTPfvMxzNH">
      <img src="https://img.shields.io/discord/987492554486452315?color=7289da&label=discord&logo=discord&logoColor=7289da" alt="Official Discord">
    </a>
    <a href="https://github.com/consumet-rs/consumet.rs/blob/main/LICENSE">
      <img src="https://img.shields.io/crates/l/consumet-api-rs.svg">
    </a>
</p>

<h2> Table of Contents </h2>

- [Quick Start](#quick-start)
  - [Installation](#installation)
  - [Usage](#usage)
- [Documentation](#documentation)
- [Ecosystem](#ecosystem)
- [Support](#support)
- [License](#license)

## Quick Start

### Installation

To use consumet-api-rs in your project, run:
```bash
cargo install consumet-api-rs
```

### Usage

**Example** - searching for a movie using the flixhq provider.
```rs
use consumet_api_rs::providers::movies;

// Create a new instance of the FlixHQ provider
let flixhq = movies::FlixHQ;
// Search for a movie. In this case, "Vincenzo"
let data = flixhq.search("Vincenzo", None).await?;
println!("{:#?}", data);
```

Do you want to know more? Head to the [`Getting Started`](https://github.com/carrotshniper21/consumet-api-rs/tree/main/docs/guides/getting-started.md).

## Documentation
- [`Getting Started`](https://github.com/consumet-rs/consumet.rs/tree/main/docs/guides/getting-started.md)
- [`Guides`](https://github.com/consumet-rs/consumet.rs/tree/main/docs)
- [`Movies`](https://github.com/consumet-rs/consumet.rs/tree/main/docs/guides/movies.md)

## Ecosystem
- [Provider Status](https://github.com/consumet/providers-status/blob/main/README.md) - A list of providers and their status.
- [Discord Server](https://discord.gg/qTPfvMxzNH) - Join the official discord server and chat with the maintainers.

## Support
You can contact the maintainer of consumet-api-rs via [email](mailto:vipershniper07@gmail.com), or [join the official discord server](https://discord.gg/qTPfvMxzNH) (Recommended).

<a href="https://discord.gg/qTPfvMxzNH">
   <img src="https://discordapp.com/api/guilds/987492554486452315/widget.png?style=banner2">
</a>

## License
Licensed under [APACHE](./LICENSE).
