use chip8_emu::emu::Emu;

fn main() {
    let mut emu = Emu::new();
    emu.run("roms/test_opcode.ch8");
}