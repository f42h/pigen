use rand::Rng;

use super::setup::GenSetup;

pub(crate) struct Pigen {
    len: usize
}

impl Pigen {
    pub(crate) fn new(len: usize) -> Self {
        Self { len }
    }

    fn get_random_num(&self, to: usize) -> usize {
        rand::rng().random_range(0..to)
    }

    fn pick_random(&self, data: &String) -> Option<char> {
        let idx = self.get_random_num(data.chars().count());
        data.chars().nth(idx)
    }

    pub(crate) fn generate<Password>(&self, ret: Password)
    where Password: Fn(String) {
        let mut setup = GenSetup::new();
        let pool = setup.gen_payload_pool();
        let mut password = String::new();

        for _ in 0..=self.len {
            let entry = pool[self.get_random_num(3)].clone(); 
            let random_char = self.pick_random(&entry);
            if let Some(c) = random_char {
                password.push(c);
            }
        }

        ret(password)
    }
}
