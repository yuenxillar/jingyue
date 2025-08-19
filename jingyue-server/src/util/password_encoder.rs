pub struct PasswordEncoder;

impl PasswordEncoder {
    pub fn encode(password: &str, salt: &str) -> String {
        let value = md5::compute(format!("{}:{}", password, salt).as_bytes());
        format!("{:x}", value)
    }

    pub fn verify(password: &str, salt: &str, encoded: &str) -> bool {
        let expected = Self::encode(password, salt);
        expected == encoded
    }
}
