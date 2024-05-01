mod login;
pub use login::LoginParams;

#[derive(Debug)]
pub enum CommandType {
    Login(LoginParams),
}

#[derive(Debug)]
pub struct ImapRequestItem {
    pub command: CommandType,
}

#[derive(Debug)]
pub struct ImapResponseItem {}
