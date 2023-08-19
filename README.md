# TP-Link L920 on/off script

This is a (very) simple Rust binary that can turn on/off a [TP-Link L920](https://www.tp-link.com/uk/home-networking/smart-bulb/tapo-l920-5/) led light strip in your local network.


## Installation

Requires you to have a modern version of Rust.

Clone this repo and run:

```bash
cargo build --release
```

Then the binary will be available in `<your-local-repo>/target/release/tapo-l920-on-off`

Alternatively, if you want to let cargo clone the git repo, compile the binary and make it available in your system PATH, you can run the following command:

```bash
cargo install --git git@github.com:lmammino/tapo-l920-on-off.git
```


## Usage

Note: you must have a Tapo account and your led strip must have been registered there.

The binary requires 3 environment variables to be set:

- `TAPO_USERNAME`: your TAPO email
- `TAPO_PASSWORD`: your TAPO password
- `TAPO_DEVICE_IP`: your L920 local IP Address

You can use the [`.env~sample`](/.env~sample) example in this repo as a reference.

Once, these variables are set, just execute the binary and if the led strip is on, it will be turned off and vice versa!

If you create a `.env` file with the necessary environment variables, you can simply run the [`run.sh`](/run.sh) script in this repository.


## Use with an Elgato Stream deck

TODO: I created this script so I can have a shortcut to turn my led strip on and off from my Elgato Stream Deck. I'll write here a quick guide on how to configure yours as well.


## Contributing

Everyone is very welcome to contribute to this project.
You can contribute just by submitting bugs or suggesting improvements by
[opening an issue on GitHub](https://github.com/lmammino/tapo-l920-on-off/issues).


## License

Licensed under [MIT License](LICENSE). © Luciano Mammino