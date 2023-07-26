pub mod app;

use app::App;


fn main () -> Result<(), eframe::Error> {
    eframe::run_native(
        "Counter",
        eframe::NativeOptions::default(),
        Box::new(App::setup),
    )
}

