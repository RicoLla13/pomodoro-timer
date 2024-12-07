use std::cell::Cell;
use std::rc::Rc;

use glib::clone;
use gtk::prelude::*;
use gtk::{self, glib, Application, ApplicationWindow, Button, Label, Orientation};

const APP_ID: &str = "some.ricolla1.PomodoroTimer";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let label = Label::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let button_increase = Button::builder()
        .label("+")
        .margin_start(12)
        .margin_end(12)
        .build();

    let button_decrease = Button::builder()
        .label("-")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let number = Rc::new(Cell::new(0));
    let label_text = format!("Current count: {}", number.get());
    label.set_label(&label_text);

    button_increase.connect_clicked(clone!(
            #[weak]
            number,
            #[weak]
            label,
            move |_| {
                number.set(number.get() + 1);
                let label_text = format!("Current count: {}", number.get());
                label.set_label(&label_text);
            }
    ));

    button_decrease.connect_clicked(clone!(
            #[weak]
            label,
            move |_| {
                number.set(number.get() - 1);
                let label_text = format!("Current count: {}", number.get());
                label.set_label(&label_text);
            }
    ));

    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&label);
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Pomodoro Timer")
        .child(&gtk_box)
        .build();

    window.present();
}
