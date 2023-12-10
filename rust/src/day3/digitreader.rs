pub(super) struct DigitReader {
    value: u32,
    part_number_found: bool,
}

impl DigitReader {
    pub fn new() -> Self {
        DigitReader {
            value: 0,
            part_number_found: false,
        }
    }

    pub fn read(&mut self, val: &u32) {
        self.value = self.value * 10 + val;
    }

    pub fn part_number_found(&mut self) {
        self.part_number_found = true;
    }

    pub fn has_found_part_number(&self) -> bool {
        self.part_number_found
    }

    pub fn get_digit(&self) -> Option<u32> {
        if self.value > 0 {
            Some(self.value)
        } else {
            None
        }
    }

    pub fn reset(&mut self) {
        self.value = 0;
        self.part_number_found = false;
    }
}
