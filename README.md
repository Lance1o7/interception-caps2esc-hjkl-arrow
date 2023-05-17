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

### Arch users

```sh
git clone https://aur.archlinux.org/interception-caps2esc-arrow-git.git
cd interception-caps2esc-arrow-git
makepkg -si
```

### Other distributions

#### Install Dependencies

- [Interception Tools](https://gitlab.com/interception/linux/tools)  
    - void `xbps-install -S interception-tools`
    - ubuntu
        ```sh
        sudo add-apt-repository ppa:deafmute/interception
        sudo apt install interception-tools
        ```
    - fedora
        ```sh
        sudo dnf copr enable fszymanski/interception-tools
        sudo dnf install interception-tools
        ```
    - from source [build from source](https://gitlab.com/interception/linux/tools#building)

#### Building and Installing

```sh
git clone https://github.com/akarsh1995/interception-caps2esc-hjkl-arrow
cd interception-caps2esc-hjkl-arrow
cmake -S . \
    -B build \
    -DCMAKE_INSTALL_PREFIX=/usr/local \
    -DCMAKE_BUILD_TYPE=Release \
    -Wno-dev
cmake --build build
sudo cmake --install build
sudo cp ./caps2esc-hjkl-arrow.yaml /etc/interception/udevmon.d/caps2esc-hjkl-arrow.yaml
# restart udevmon daemon (interception-tools)
sudo systemctl restart udevmon.service
# enable daemon on restart
sudo systemctl enable udevmon.service
```

## Execution

If everything goes well you'll be able to make use of your capslock as intended.
