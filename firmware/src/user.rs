use std::{
    collections::BTreeSet,
    sync::{Arc, Mutex},
};

use esp_idf_svc::nvs::{EspNvs, NvsDefault};

#[derive(Clone)]
pub struct UserDB(Arc<Mutex<UserData>>);

struct UserData {
    nvs: EspNvs<NvsDefault>,
    codes: BTreeSet<String>,
}

impl UserDB {
    pub fn new(nvs: EspNvs<NvsDefault>) -> Self {
        UserDB(Arc::new(Mutex::new(UserData {
            nvs,
            codes: BTreeSet::new(),
        })))
    }

    pub fn add(&self, code: &str) {
        let mut data = self.0.lock().unwrap();
        data.codes.insert(code.to_owned());
        if let Err(e) = data.nvs.set_u8(code, 1) {
            log::error!("Error saving to nvs {}", e);
        }
    }

    pub fn contains(&self, code: &str) -> bool {
        let data = self.0.lock().unwrap();
        data.codes.contains(code)
    }

    pub fn delete(&self, code: &str) {
        let mut data = self.0.lock().unwrap();
        data.codes.remove(code);
        if let Err(e) = data.nvs.remove(code) {
            log::error!("Error saving to nvs {}", e);
        }
    }
}
