pub mod crash;
pub mod dashboard;
pub mod login;

pub use crash::CrashScreen;
pub use dashboard::DashboardScreen;
pub use login::LoginScreen;

#[derive(Debug)]
pub enum Screen {
    Login(LoginScreen),
    Dashboard(DashboardScreen),
    Crash(CrashScreen),
}

impl Default for Screen {
    fn default() -> Self {
        Self::Login(LoginScreen::default())
    }
}
