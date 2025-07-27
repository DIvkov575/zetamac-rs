#[derive(PartialEq, Default)]
pub enum AppState {
    #[default]
    Configuration,
    Testing,
    Results,
}
