use tracing::info;

use crate::{config::Action, state::HoloState};

impl HoloState {
    pub fn handle_action(&mut self, action: Action) {
        match action {
            Action::Quit => self.loop_signal.stop(),
            Action::Debug => todo!(),
            Action::Close => {
                if let Some(d) = self
                    .workspaces
                    .current()
                    .window_under(self.pointer_location)
                {
                    d.0.toplevel().send_close()
                }
            }
            Action::Workspace(id) => self.workspaces.activate(id),
            Action::MoveWindowToWorkspace(id) => {
                let window = self
                    .workspaces
                    .current()
                    .window_under(self.pointer_location)
                    .map(|d| d.0.clone());

                if let Some(window) = window {
                    self.workspaces
                        .move_window_to_workspace(&window, id, self.config.gaps);
                }
            }
            Action::MoveWindowAndSwitchToWorkspace(u8) => {
                self.handle_action(Action::MoveWindowToWorkspace(u8));
                self.handle_action(Action::Workspace(u8));
            }
            Action::ToggleWindowFloating => todo!(),
            Action::Spawn(command) => {
                if let Err(err) = std::process::Command::new("/bin/sh")
                    .arg("-c")
                    .arg(command.clone())
                    .spawn()
                {
                    info!("{} {} {}", err, "Failed to spawn \"{}\"", command);
                }
            }
        }
    }
}