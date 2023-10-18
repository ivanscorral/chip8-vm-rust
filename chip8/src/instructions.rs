pub(crate) enum Opcode {
    Halt,
    ClearScreen,
    Return,
    JumpToAddress(u16),
    CallAddress(u16),
    SkipIfRegEqualsByte(u8),
    SkipIfRegNotEqualsByte(u8),
    SkipIfRegEqualsReg,
    LoadByteIntoReg(u8),
    AddByteToReg(u8),
    LoadRegIntoReg,
    OrRegWithReg,
    AndRegWithReg,
    XorRegWithReg,
    AddRegToReg,
    SubtractRegFromReg,
    ShiftRight,
    SubstractRegFromOtherReg,
    ShiftLeft,
    SkipIfRegNotEqualsReg,
    LoadIndex(u16),
    JumpToAddressPlusV0(u16),
    RandomByte(u8),
    DrawSprite(u8),
    SkipIfKeyPressed,
    SkipIfKeyNotPressed,
    LoadDelayTimerIntoReg,
    LoadKeyIntoReg,
    LoadRegIntoDelayTimer,
    LoadRegIntoSoundTimer,
    AddRegToIndex,
    LoadFontIntoReg,
    LoadBCDIntoMem,
    StoreRegsIntoMem,
    LoadRegsFromMem,
    Unknown,
    Sys,
}

pub(crate) fn parse_opcode(opcode: u16) -> (Opcode, u8, u8) {
    let reg_x = ((opcode & 0x0F00) >> 8) as u8;
    let reg_y = ((opcode & 0x00F0) >> 4) as u8;
    let k = (opcode & 0x00FF) as u8;
    let n = (opcode & 0x000F) as u8;
    let addr = opcode & 0x0FFF;
    println!("Executing opcode: {:04X}", opcode);
    println!(
        "addr: 0x{:03X}\treg_x: V{:01X}\treg_y: V{:01X}\tkk: 0x{:02X}\tnnn: 0x{:03X}",
        addr, reg_x, reg_y, k, addr
    );
    let parsed_opcode = match opcode & 0xF000 {
        0x0000 => match opcode & 0x00FF {
            0x00E0 => Opcode::ClearScreen,
            0x00EE => Opcode::Return,
            _ => Opcode::Sys,
        },
        0x1000 => Opcode::JumpToAddress(addr),
        0x2000 => Opcode::CallAddress(addr),
        0x3000 => Opcode::SkipIfRegEqualsByte(k),
        0x4000 => Opcode::SkipIfRegNotEqualsByte(k),
        0x5000 => Opcode::SkipIfRegEqualsReg,
        0x6000 => Opcode::LoadByteIntoReg(k),
        0x7000 => Opcode::AddByteToReg(k),
        0x8000 => match opcode & 0x000F {
            0x0000 => Opcode::LoadRegIntoReg,
            0x0001 => Opcode::OrRegWithReg,
            0x0002 => Opcode::AndRegWithReg,
            0x0003 => Opcode::XorRegWithReg,
            0x0004 => Opcode::AddRegToReg,
            0x0005 => Opcode::SubtractRegFromReg,
            0x0006 => Opcode::ShiftRight,
            0x0007 => Opcode::SubstractRegFromOtherReg,
            0x000E => Opcode::ShiftLeft,
            _ => Opcode::Unknown,
        },
        0x9000 => Opcode::SkipIfRegNotEqualsReg,
        0xA000 => Opcode::LoadIndex(addr),
        0xB000 => Opcode::JumpToAddressPlusV0(addr),
        0xC000 => Opcode::RandomByte(k),
        0xD000 => Opcode::DrawSprite(n),
        0xE000 => match opcode & 0x00FF {
            0x009E => Opcode::SkipIfKeyPressed,
            0x00A1 => Opcode::SkipIfKeyNotPressed,
            _ => Opcode::Unknown,
        },
        0xF000 => match opcode & 0x00FF {
            0x0007 => Opcode::LoadDelayTimerIntoReg,
            0x000A => Opcode::LoadKeyIntoReg,
            0x0015 => Opcode::LoadRegIntoDelayTimer,
            0x0018 => Opcode::LoadRegIntoSoundTimer,
            0x001E => Opcode::AddRegToIndex,
            0x0029 => Opcode::LoadFontIntoReg,
            0x0033 => Opcode::LoadBCDIntoMem,
            0x0055 => Opcode::StoreRegsIntoMem,
            0x0065 => Opcode::LoadRegsFromMem,
            _ => Opcode::Unknown,
        }
        _ => Opcode::Unknown,
    };
    (parsed_opcode, reg_x, reg_y)
}
