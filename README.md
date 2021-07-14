<!--
*** This README is using the Best-README-Template (https://github.com/othneildrew/Best-README-Template).
-->

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]



<p align="center">
  <a href="https://github.com/dr-electron/blue-dragon">
    <img src="blue-dragon.svg" alt="Logo" width="400" height="400">
  </a>

  <h3 align="center">Blue Dragon</h3>

  <p align="center">
    A swearword filter for the first IOTA asset registry implementation
  </p>
</p>



<!-- TABLE OF CONTENTS -->
<details open="open">
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#build">Build</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
  </ol>
</details>



## About The Project

This project was developed as a swearword filter for the first IOTA asset registry implementation. The swearword filter part was heavily inspired by [gofuckyourself](https://github.com/JoshuaDoes/gofuckyourself).



## Getting Started

### Prerequisites

Install [Rust](https://www.rust-lang.org/tools/install).

### Build

Build a binary with `cargo build --release`.



## Usage

```
USAGE:
    blue-dragon [OPTIONS] --password <password> --user <user>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --url <assets-domain>    Domain of the asset registry [default: https://asset-
                                 registry.tokenizedassetsdemo.iota.cafe]
    -t, --delay <delay>          Delay in milliseconds between searches [default: 1000]
    -i, --id <network-id>        Network ID amongst "nectar", "pollen", "test" and "internal" [default: nectar]
    -p, --password <password>    Password for asset registry
    -u, --user <user>            Username for asset registry
```



<!-- MARKDOWN LINKS & IMAGES -->
[contributors-shield]: https://img.shields.io/github/contributors/dr-electron/blue-dragon.svg?style=for-the-badge
[contributors-url]: https://github.com/dr-electron/blue-dragon/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/dr-electron/blue-dragon.svg?style=for-the-badge
[forks-url]: https://github.com/dr-electron/blue-dragon/network/members
[stars-shield]: https://img.shields.io/github/stars/dr-electron/blue-dragon.svg?style=for-the-badge
[stars-url]: https://github.com/dr-electron/blue-dragon/stargazers
[issues-shield]: https://img.shields.io/github/issues/dr-electron/blue-dragon.svg?style=for-the-badge
[issues-url]: https://github.com/dr-electron/blue-dragon/issues
[license-shield]: https://img.shields.io/github/license/dr-electron/blue-dragon.svg?style=for-the-badge
[license-url]: https://github.com/dr-electron/blue-dragon/blob/main/LICENSE
