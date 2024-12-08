use glib::clone;
use glib::ControlFlow;
use gtk::glib;
use gtk::Label;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

pub struct PomodoroTimer {
    pub pomodoro_duration: u64,
    pub break_duration: u64,
    pub num_pomodoros: u64,
    pub counter: Rc<Cell<u64>>,
}

impl PomodoroTimer {
    pub fn new(pomodoro_duration: u64, break_duration: u64, num_pomodoros: u64) -> Self {
        let counter = Rc::new(Cell::new(0));
        PomodoroTimer {
            pomodoro_duration,
            break_duration,
            num_pomodoros,
            counter,
        }
    }

    pub fn countdown(&self, label: &Label) -> ControlFlow {
        label.set_text(&format!("{:02}", self.counter.get()));

        if self.counter.get() > 0 {
            self.counter.set(self.counter.get() - 1);
            ControlFlow::Continue
        } else {
            ControlFlow::Break
        }
    }

    pub fn start(timer: Rc<RefCell<Self>>, timer_label: &Label, title_label: &Label) {
        let timer_clone = timer.clone();
        let timer = timer_clone.borrow_mut();

        timer.counter.set(timer.pomodoro_duration);
        title_label.set_text(&"Pomodoro Started!");
        glib::timeout_add_seconds_local(
            1,
            clone!(
                #[strong]
                timer_clone,
                #[strong]
                timer_label,
                move || {
                    let timer = timer_clone.borrow_mut();
                    timer.countdown(&timer_label)
                }
            ),
        );
    }
}
