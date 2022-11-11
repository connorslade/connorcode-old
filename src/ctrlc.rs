use std::{process, sync::Arc};

use crate::{analytics, app::App, color::Color};

pub fn init(app: Arc<App>) {
    ctrlc::set_handler(move || {
        color_print!(Color::Yellow, "\n[*] Exiting");

        // Dump in-memory analytics
        analytics::dump(app.clone());

        // Cleanup Database
        app.database
            .lock()
            .pragma_update(None, "wal_checkpoint", "TRUNCATE")
            .unwrap();

        // Eggsit
        process::exit(0);
    })
    .unwrap();
}
