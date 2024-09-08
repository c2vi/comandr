use crate::ComandrResult;


#[derive()]
pub struct Command {
    pub name: String,
    pub closure: Box<dyn FnMut() -> ComandrResult<()> + Send + Sync>,
}
