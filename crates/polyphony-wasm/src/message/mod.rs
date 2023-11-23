pub mod crash;
pub mod dashboard;
pub mod login;

pub use crash::CrashMessage;
pub use dashboard::DashboardMessage;
pub use login::LoginMessage;

#[derive(Debug, Clone)]
pub enum Message {
    Login(LoginMessage),
    Dashboard(DashboardMessage),
    Crash(CrashMessage),
}
