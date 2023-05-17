use inquire::{error::InquireResult, Select};
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone)]
pub enum Endpoint {
    CreateTask,
    GetTask,
}

impl Endpoint {
    pub const VARIANTS: &'static [Endpoint] = &[Self::CreateTask, Self::GetTask];
}

impl Display for Endpoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{self:?}")
    }
}

pub fn select_endpoint() -> InquireResult<Endpoint> {
    Select::new("", Endpoint::VARIANTS.to_vec()).prompt()
}
