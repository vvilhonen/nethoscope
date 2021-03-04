# Nethoscope

Employ your built-in wetware pattern recognition and signal processing facilities to understand your network traffic.

> Check video on how it works at https://www.youtube.com/watch?v=j1fqy6CmmeM

## Installation with [cargo](https://rustup.rs/)

    cargo install nethoscope

There are various dependencies on each platform described below:

### macOS

Pcap should be included with the macOS so no extra steps required.

### Linux

On debian based systems `libpcap-dev` package should be enough. More information [here](https://github.com/ebfull/pcap#linux).

### Windows

Windows requires pcap compatible library for which the [npcap](https://nmap.org/npcap/) is the best option. 

For using the binary you only need the [npcap installer](https://nmap.org/npcap/dist/npcap-1.10.exe) and for compiling from source, `Lib/x64/wpcap.lib` file needs to be copied to the project root from the [npcap SDK](https://nmap.org/npcap/dist/npcap-sdk-1.06.zip) before compiling in addition.

## Credits

This experiment was easily implemented building on these two excellent crates:

- [pcap](https://github.com/ebfull/pcap) to capture network traffic

