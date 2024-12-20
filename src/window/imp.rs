use glib::clone;
use glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate, Label};
use gtk::{prelude::*, SpinButton};
use std::cell::RefCell;
use std::rc::Rc;

use crate::pomodoro::PomodoroTimer;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/pomodoro/window.ui")]
pub struct Window {
    #[template_child]
    pub title_label: TemplateChild<Label>,
    #[template_child]
    pub timer_label: TemplateChild<Label>,
    #[template_child]
    pub start_button: TemplateChild<Button>,
    #[template_child]
    pub stop_button: TemplateChild<Button>,
    #[template_child]
    pub pom_spin: TemplateChild<SpinButton>,
    #[template_child]
    pub break_spin: TemplateChild<SpinButton>,
    #[template_child]
    pub num_spin: TemplateChild<SpinButton>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "PomodoroWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

#[gtk::template_callbacks]
impl Window {
    #[template_callback]
    fn handle_start_click(&self, _button: &Button) {
        let timer_label = self.timer_label.clone();
        let title_label = self.title_label.clone();

        let timer = Rc::new(RefCell::new(PomodoroTimer::new(5, 5, 2)));
        
        PomodoroTimer::start(timer, &timer_label, &title_label);
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Window {}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}
