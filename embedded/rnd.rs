pub struct PseudoRandomGenerator {
    lfsr: u16,
}

impl PseudoRandomGenerator {
    pub fn new(seed: u16) -> Self {
        Self { lfsr: seed }
    }

    pub fn get(&mut self) -> u16 {
        let lfsr = &mut self.lfsr;
        *lfsr ^= *lfsr >> 7;
        *lfsr ^= *lfsr << 9;
        *lfsr ^= *lfsr >> 13;
        *lfsr
    }
}
