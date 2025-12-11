use human_panic::{Metadata, setup_panic};
use parsel::ui;

fn main() {
    setup_panic!(
        Metadata::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
            .authors("Stanley Masinde <hello@stanleymasinde.com>")
            .homepage("github.com/StanleyMasinde/parsel")
            .support("- Open a support request by email to hello@stanleymasinde.com")
    );

    ui::run().unwrap();
}
