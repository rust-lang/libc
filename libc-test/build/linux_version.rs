#[derive(Default)]
pub struct LinuxVersion {
    split: Option<(u32, u32)>,
    code: Option<u32>,
}

impl LinuxVersion {
    pub fn parse_macro(&mut self, name: &str, value: &str) {
        match name {
            "LINUX_VERSION_CODE" => self.code = value.parse().ok(),
            "LINUX_VERSION_MAJOR" => self.split.get_or_insert_default().0 = value.parse().unwrap(),
            "LINUX_VERSION_PATCHLEVEL" => {
                self.split.get_or_insert_default().1 = value.parse().unwrap()
            }
            _ => (),
        }
    }

    // Prefer the split version macros, falling back to decoding LINUX_VERSION_CODE.
    pub fn get(&self) -> Option<(u32, u32)> {
        match self.split {
            Some(version) => Some(version),
            None => match self.code {
                Some(code) => Some((code >> 16, (code >> 8) & 0xff)),
                None => None,
            },
        }
    }
}
