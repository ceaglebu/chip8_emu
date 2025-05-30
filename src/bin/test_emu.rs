use chip8_emu::emu::Emu;

fn main() {
    let mut emu = Emu::new();
    emu.run("roms/games/Tron.ch8");
}
