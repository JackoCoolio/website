---
title: This website!
sortKey: 1
---

Okay, maybe I went a little overboard on this one... but I had fun!

This website is written in **Rust** with a few notable dependencies:

- [Axum](https://github.com/tokio-rs/axum), for serving HTTP requests
- [Maud](https://maud.lambda.xyz/), for HTML templating
- [Comrak](https://github.com/kivikakk/comrak) and [Serde](https://serde.rs/)
  for parsing Markdown files for each of these projects.

Since the VPS I host this server from runs **NixOS**, I have the configuration
I need for running this defined as a [NixOS
module](https://github.com/JackoCoolio/website/blob/main/nixos.nix) that I
import in my system configuration's `flake.nix`, which makes it super easy to
configure and maintain long-term.
