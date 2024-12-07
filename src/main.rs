use std::cell::Cell;
use std::rc::Rc;

use gtk::prelude::*;
use gtk::{self, gdk, glib, Application, ApplicationWindow, Button, Label, Orientation};
use gtk::{Adjustment, Align, CssProvider, SpinButton};

const APP_ID: &str = "some.ricolla1.PomodoroTimer";
const ELEM_SPACING: i32 = 12;

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    app.run()
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("style.css"));
    let display = gdk::Display::default().expect("[*] Failed to get default display.");

    gtk::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    let title_label = Label::new(Some("Pomodoro timer"));
    let timer_label = Label::builder()
        .label("00")
        .build();
    timer_label.add_css_class("timer-label");
    let pom_spin_label = Label::builder()
        .xalign(0.0)
        .label("Pomodoro duration")
        .build();
    let break_spin_label = Label::builder().xalign(0.0).label("Break duration").build();
    let num_spin_label = Label::builder()
        .xalign(0.0)
        .label("Number of Pomodoros")
        .build();

    let button_start = Button::with_label("Start");
    let button_stop = Button::with_label("Start");

    let pom_adj = Adjustment::builder()
        .lower(0.0)
        .upper(59.0)
        .step_increment(1.0)
        .page_increment(10.0)
        .build();
    let break_adj = Adjustment::builder()
        .lower(0.0)
        .upper(59.0)
        .step_increment(1.0)
        .page_increment(10.0)
        .build();
    let num_adj = Adjustment::builder()
        .lower(0.0)
        .upper(10.0)
        .step_increment(1.0)
        .page_increment(10.0)
        .build();
    let pom_spin = SpinButton::builder().adjustment(&pom_adj).build();
    let break_spin = SpinButton::builder().adjustment(&break_adj).build();
    let num_spin = SpinButton::builder().adjustment(&num_adj).build();

    let main_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(ELEM_SPACING * 3)
        .margin_top(ELEM_SPACING)
        .margin_bottom(ELEM_SPACING)
        .margin_start(ELEM_SPACING)
        .margin_end(ELEM_SPACING)
        .build();
    let left_area = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(ELEM_SPACING)
        .valign(Align::Center)
        .build();
    let button_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(ELEM_SPACING)
        .hexpand(true)
        .halign(Align::Center)
        .build();
    let right_area = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(ELEM_SPACING)
        .build();
    let pom_area = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(ELEM_SPACING)
        .build();
    let break_area = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(ELEM_SPACING)
        .build();
    let num_area = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(ELEM_SPACING)
        .build();

    button_start.set_halign(Align::Center);
    button_stop.set_halign(Align::Center);

    button_box.append(&button_start);
    button_box.append(&button_stop);

    left_area.append(&title_label);
    left_area.append(&timer_label);
    left_area.append(&button_box);

    pom_area.append(&pom_spin);
    pom_area.append(&pom_spin_label);
    break_area.append(&break_spin);
    break_area.append(&break_spin_label);
    num_area.append(&num_spin);
    num_area.append(&num_spin_label);

    right_area.append(&pom_area);
    right_area.append(&break_area);
    right_area.append(&num_area);

    main_box.append(&left_area);
    main_box.append(&right_area);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Pomodoro Timer")
        .child(&main_box)
        .build();

    window.present();
}
