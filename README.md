# miko_chip8emulator
This is simple chip8 emulator in Rust. It is a work in progress and is not yet complete.
Made using [this](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM) reference. 
example_roms were taken from [here](https://github.com/kripod/chip8-roms/tree/master).

Features:
- window can be resized, pixels will be as big as they can
- keyboard is mapped to qwerty-keyboard (from 1 to v), original chip8-keyboard look like [this](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#:~:text=8-,9,-E)
- works really slow, cause use only one thread
- don't have any way to affect parameters of already compiled program

ToDo:
- [ ] make two threads instead of one
- [ ] make a cli instead of hardcoded values
- [x] play tetris 
- [ ] write tests (or test_roms) for emulator