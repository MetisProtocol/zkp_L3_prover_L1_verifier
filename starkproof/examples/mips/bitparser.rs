fn get_n_bits(v: u32, n: u8, starting_from: u8) -> u32 {
    let mask = u32::MAX >> (32 - n);
    let isolated_bits = v >> starting_from;
    isolated_bits & mask
}

pub fn get_register_opcode(v: u32) -> u8 {
    get_n_bits(v, 8, 0) as u8
}

pub fn get_immediate_opcode(v: u32) -> u8 {
    get_n_bits(v, 6, 26) as u8
}

pub fn get_s_register(v: u32) -> usize {
    get_n_bits(v, 5, 21) as usize
}

pub fn get_t_register(v: u32) -> usize {
    get_n_bits(v, 5, 16) as usize
}

pub fn get_d_register(v: u32) -> usize {
    get_n_bits(v, 5, 11) as usize
}

pub fn get_immediate_value(v: u32) -> u16 {
    get_n_bits(v, 16, 0) as u16
}
