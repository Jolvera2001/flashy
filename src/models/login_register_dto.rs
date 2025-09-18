#[derive(Default)]
pub struct LoginRegisterDto {
    pub login_name: String,
    pub login_password: String,
    pub register_name: String,
    pub register_email: String,
    pub register_password: String,
}

impl LoginRegisterDto {
    pub fn clear(&mut self) {
        self.login_name.clear();
        self.login_password.clear();
        self.register_name.clear();
        self.register_email.clear();
        self.register_password.clear();
    }

    pub fn clear_login_fields(&mut self) {
        self.login_name.clear();
        self.login_password.clear();
    }

    pub fn clear_register_fields(&mut self) {
        self.register_name.clear();
        self.register_email.clear();
        self.register_password.clear();
    }
}