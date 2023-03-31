pub mod compositor;
pub mod input;
pub mod xdg_shell;

//
// Wl Seat
//

use smithay::desktop::Window;
use smithay::input::{SeatHandler, SeatState};

use smithay::wayland::data_device::{
    ClientDndGrabHandler, DataDeviceHandler, ServerDndGrabHandler,
};
use smithay::{delegate_data_device, delegate_output, delegate_seat};

use crate::state::HoloState;

impl SeatHandler for HoloState {
    type KeyboardFocus = Window;
    type PointerFocus = Window;

    fn seat_state(&mut self) -> &mut SeatState<HoloState> {
        &mut self.seat_state
    }

    fn cursor_image(
        &mut self,
        _seat: &smithay::input::Seat<Self>,
        _image: smithay::input::pointer::CursorImageStatus,
    ) {
    }
    fn focus_changed(&mut self, _seat: &smithay::input::Seat<Self>, _focused: Option<&Window>) {}
}

delegate_seat!(HoloState);

//
// Wl Data Device
//

impl DataDeviceHandler for HoloState {
    fn data_device_state(&self) -> &smithay::wayland::data_device::DataDeviceState {
        &self.data_device_state
    }
}

impl ClientDndGrabHandler for HoloState {}
impl ServerDndGrabHandler for HoloState {}

delegate_data_device!(HoloState);

//
// Wl Output & Xdg Output
//

delegate_output!(HoloState);
