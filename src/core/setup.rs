#[derive(PartialEq)]
enum Case {
    Low,
    Up
}

pub(in crate::core) struct GenSetup {
    payload: String
}

impl GenSetup {
    pub(in crate::core) fn new() -> Self {
        Self {
            payload: String::new()
        }
    }

    fn gen_letters(&mut self, case: Case) {
        self.payload.clear();

        let byte_lit = if case == Case::Low { b'a' } else { b'A'};
    
        for l in 0..26 { 
            self.payload.push((byte_lit + l) as char);
        }
    }
    
    fn gen_spec_chars(&mut self) {
        self.payload.clear();

        let specs: Vec<u8> = vec![
            33, 34, 35, 36, 37, 38, 38, 40, 41, 42, 43, 44, 45, 46,
            58, 59, 60, 61, 62, 63, 64, 
            91, 92, 93, 94, 95, 
            123, 124, 125
        ]; 
    
        for c in specs {
            self.payload.push(c as char);
        }
    }

    pub(in crate::core) fn gen_payload_pool(&mut self) -> Vec<String> {
        let mut pool: Vec<String> = Vec::new();

        self.gen_letters(Case::Low); // Generate lowercase letters
        pool.push(self.payload.clone());

        self.gen_letters(Case::Up); // Generate uppercase letters
        pool.push(self.payload.clone());

        self.gen_spec_chars(); // Generate special characters
        pool.push(self.payload.clone());

        pool
    }
}