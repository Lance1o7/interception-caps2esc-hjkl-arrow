# interception-caps2esc-hjkl-arrow

Mostly you're here because you use vim/neovim or need vim like bindings instead of arrow keys.  
Hence no boilerplate.

## Why?

I could not find a program which only and only remaps  
- `CAPSLOCK` -> `ESCAPE`
- `CAPSLOCK + H J K L` -> `ARROW`

and 

**[Has no dependency on Xorg and keyboard layout](https://unix.stackexchange.com/questions/414926/bind-capshjkl-to-arrow-keys-caps-to-esc)** so that it works in tty mode

## Installation
### Debian 13

```sh
# Install rust if needed - https://rustup.rs/
sudo apt install interception-tools
git clone https://github.com/Lance1o7/interception-caps2esc-hjkl-arrow
cd interception-caps2esc-hjkl-arrow
cargo build  --release --all-features
sudo cp target/release/caps2esc-hjkl-arrow /usr/bin/
sudo cp caps2esc-hjkl-arrow.yaml /etc/interception/udevmon.d/
sudo vim /etc/interception-vimproved/config.yaml # change "intercept" to "interception"
# details: https://packages.debian.org/bookworm/interception-tools, The upstream command name 'intercept' is renamed to 'interception' in this package to avoid the name collision. If you wish to use command names which the upstream uses, please install the interception-tools-compat package.
sudo systemctl restart udevmon.service
sudo systemctl enable udevmon.service
```
### Fedora
```
sudo dnf copr enable brirec/interception-tools
sudo dnf install interception-tools
git clone https://github.com/Lance1o7/interception-caps2esc-hjkl-arrow
cd interception-caps2esc-hjkl-arrow
cargo build  --release --all-features
sudo cp target/release/caps2esc-hjkl-arrow /usr/bin/
sudo mkdir /etc/interception/
sudo mkdir /etc/interception/udevmon.d/
sudo cp caps2esc-hjkl-arrow.yaml /etc/interception/udevmon.d/
sudo systemctl restart udevmon.service
sudo systemctl enable udevmon.service
```

