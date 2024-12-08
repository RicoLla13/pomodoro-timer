mod window;
mod pomodoro;

use gtk::prelude::*;
use gtk::{gio, gdk, glib, Application, CssProvider};
use window::Window;

const APP_ID: &str = "org.gtk_rs.CompositeTemplates1";

fn main() -> glib::ExitCode {
    // Register and include resources
    gio::resources_register_include!("pomodoro_rsc.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("style.css"));
    let display = gdk::Display::default().expect("[*] Failed to load default diplay.");

    gtk::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    // Create new window and present it
    let window = Window::new(app);
    window.present();
}
