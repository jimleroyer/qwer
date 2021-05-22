use crate::config::QwerConfig;

pub trait Command: Sized {
    type Error: std::error::Error;
    fn apply(self, config: &QwerConfig) -> Result<(), Self::Error>;

    fn handle_error(err: Self::Error, config: &QwerConfig) {
        std::process::exit(1);
    }

    fn call(self, config: QwerConfig) {
        match self.apply(&config) {
            Ok(()) => (),
            Err(err) => Self::handle_error(err, &config),
        }
    }
}
