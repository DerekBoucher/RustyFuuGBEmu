# RustyFuuGBEmu

DMG Gameboy emulator fully written in rust 🦀

# TODO

- [ ] Implement Memory
    - [ ] Implement Cartridge
        - [x] ROM Only
        - [X] MBC1
        - [ ] MBC2
        - [ ] MBC3
        - [ ] MBC4
        - [ ] MBC5
        - [ ] MBC6
        - [ ] MBC7
        - [ ] MMM01
        - [ ] M161
        - [ ] HuC1
        - [ ] HuC-3
        - [ ] Others
    - [x] Add Boot ROM
    - [ ] I/O Registers
    - [ ] OAM RAM
- [ ] Implement CPU
    - [ ] Implement Timers
- [ ] Implement PPU
- [ ] Implement APU
- [ ] Implement Main

# Important Notes

## Obscure Half Carry Check for 16-bit OPs

Some specific OPCODES check and set the half carry flag using bits 3-> 4 while some others use 11 -> 12.
See https://stackoverflow.com/questions/57958631/game-boy-half-carry-flag-and-16-bit-instructions-especially-opcode-0xe8 for more details

## Ubuntu Dependencies

`glium` dependencies:
```sh
sudo apt install libfontconfig1-dev
```


