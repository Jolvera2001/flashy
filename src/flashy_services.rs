use crate::flashy::Flashy;
use crate::flashy_events::FlashyEvents;

impl Flashy {
    pub fn handle_events(&mut self, ctx: &egui::Context) {
        if let Some(promise) = &self.current_operation {
            if let Some(result) = promise.ready() {
                match result {
                    FlashyEvents::UserLogIn(user) => {},
                    FlashyEvents::UserLogOut => {},
                    FlashyEvents::DialogClosed(dialog) => {
                        println!("{}", format!("Dialog {} was closed", dialog))
                    }
                    FlashyEvents::DialogOpened(dialog) => {
                        println!("{}", format!("Dialog {} was opened", dialog))
                    }
                    FlashyEvents::OperationFailed { operation, error } => {
                        println!("{}", format!("Operation: {} failed with error : {}", operation, error));
                    }
                }
                self.current_operation = None;
            } else {
                println!("Promise not ready yet")
            }
        }
    }
}