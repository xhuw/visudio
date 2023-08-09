# visudio
audacity but a tui - big dreams


On linux, requires libalsa-dev and [rust](https://www.rust-lang.org/tools/install)

```
sudo apt-get install libasound2-dev
```


# Current status

run - `cargo run` and then select your desired audio device, it will then print live sample values to the terminal.

tested on windows and rpi. For some reason I2S device isn't showing up on rpi but USB does