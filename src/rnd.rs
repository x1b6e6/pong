pub struct PseudoRandomGenerator {
    lfsrl: u16,
    lfsrh: u16,
}

impl PseudoRandomGenerator {
    pub fn new(seed: u32) -> Self {
        Self {
            lfsrl: ((seed >> 0) & 0xffff) as u16,
            lfsrh: ((seed >> 16) & 0xffff) as u16,
        }
    }

    fn lfsr_next(lfsr: &mut u16) {
        *lfsr ^= *lfsr >> 7;
        *lfsr ^= *lfsr << 9;
        *lfsr ^= *lfsr >> 13;
    }

    pub fn get<T>(&mut self) -> T
    where
        T: From<u32>,
    {
        Self::lfsr_next(&mut self.lfsrl);
        Self::lfsr_next(&mut self.lfsrh);
        let o = ((self.lfsrh as u32) << 16) | (self.lfsrl as u32);
        T::from(o)
    }
}
