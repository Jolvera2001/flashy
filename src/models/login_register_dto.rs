#[derive(Default)]
pub struct LoginRegisterDto {
    pub login_name: String,
    pub login_password: String,
    pub register_name: String,
    pub register_email: String,
    pub register_password: String,
}