pub mod login;
pub mod register;
mod convert;
mod measures;

pub use login::Login;
pub use register::Register;
pub use convert::Convert;
pub use measures::MeasureList;

#[derive(Debug, Clone, Copy, Default)]
pub enum Page {
    #[default]
    Convert,
    Login,
    Register,
    MeasureList,
}

impl Page {
    pub fn path(&self) -> &'static str {
        match self {
            Page::Convert => "/",
            Page::Login => "/login",
            Page::Register => "/register",
            Page::MeasureList => "/measures",
        }
    }
}