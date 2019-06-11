#![deny(missing_docs)]

//! # Fake-DPI
//! A window wrapper that simulates fake Hi-DPI screen by manipulating window events.
//!
//! Used to test application logic on computers without Hi-DPI screen support.

extern crate window;
extern crate input;

use std::time::Duration;
use std::error::Error;

use window::{
    AdvancedWindow,
    BuildFromWindowSettings,
    Position,
    Window,
    WindowSettings,
    Size,
};
use input::{Input, TimeStamp};

/// Wraps a window to simulate Hi-DPI screen.
pub struct FakeDpiWindow<W> {
    /// The inner window.
    pub inner: W,
    /// Controls DPI factor.
    ///
    /// This can be changed at run-time to test application logic.
    /// By default, this is set to `2.0`.
    pub dpi: f64,
}

impl<W: BuildFromWindowSettings> BuildFromWindowSettings for FakeDpiWindow<W> {
    fn build_from_window_settings(
        settings: &WindowSettings
    ) ->  Result<Self, Box<dyn Error + 'static>> {
        let dpi = 2.0;
        let mut settings = settings.clone();
        let size = settings.get_size();
        settings.set_size(Size {
            width: size.width * dpi,
            height: size.height * dpi,
        });
        Ok(FakeDpiWindow {
            inner: settings.build()?,
            dpi,
        })
    }
}

impl<W: Window> Window for FakeDpiWindow<W> {
    fn set_should_close(&mut self, val: bool) {self.inner.set_should_close(val)}
    fn should_close(&self) -> bool {self.inner.should_close()}
    fn size(&self) -> Size {
        let size = self.inner.size();
        Size {width: size.width / self.dpi, height: size.height / self.dpi}
    }
    fn swap_buffers(&mut self) {self.inner.swap_buffers()}
    fn wait_event(&mut self) -> (Input, Option<TimeStamp>) {
        let (e, t) = self.inner.wait_event();
        (map_input(self.dpi, e), t)
    }
    fn wait_event_timeout(&mut self, val: Duration) -> Option<(Input, Option<TimeStamp>)> {
        self.inner.wait_event_timeout(val).map(|(e, t)| (map_input(self.dpi, e), t))
    }
    fn poll_event(&mut self) -> Option<(Input, Option<TimeStamp>)> {
        self.inner.poll_event().map(|(e, t)| (map_input(self.dpi, e), t))
    }
    fn draw_size(&self) -> Size {self.inner.draw_size()}
}

impl<W: AdvancedWindow> AdvancedWindow for FakeDpiWindow<W> {
    fn get_title(&self) -> String {self.inner.get_title()}
    fn set_title(&mut self, val: String) {self.inner.set_title(val)}
    fn get_exit_on_esc(&self) -> bool {self.inner.get_exit_on_esc()}
    fn set_exit_on_esc(&mut self, val: bool) {self.inner.set_exit_on_esc(val)}
    fn get_automatic_close(&self) -> bool {self.inner.get_automatic_close()}
    fn set_automatic_close(&mut self, val: bool) {self.inner.set_automatic_close(val)}
    fn set_capture_cursor(&mut self, val: bool) {self.inner.set_capture_cursor(val)}
    fn show(&mut self) {self.inner.show()}
    fn hide(&mut self) {self.inner.hide()}
    fn get_position(&self) -> Option<Position> {self.inner.get_position()}
    fn set_position<P: Into<Position>>(&mut self, val: P) {self.inner.set_position(val)}
    fn set_size<S: Into<Size>>(&mut self, val: S) {self.inner.set_size(val)}
}

fn map_input(dpi: f64, e: Input) -> Input {
    use Input::*;
    use input::Motion::*;
    use input::ResizeArgs;

    match e {
        Focus(_) | Cursor(_) | Move(Touch(_)) | Move(ControllerAxis(_)) | Button(_) | Text(_) | FileDrag(_) | Close(_) => e,
        Move(MouseCursor(pos)) => Move(MouseCursor([pos[0] / dpi, pos[1] / dpi])),
        Move(MouseRelative(pos)) => Move(MouseRelative([pos[0] / dpi, pos[1] / dpi])),
        Move(MouseScroll(pos)) => Move(MouseScroll([pos[0] / dpi, pos[1] / dpi])),
        Resize(args) => Resize(ResizeArgs {
            draw_size: args.draw_size,
            window_size: [args.window_size[0] / dpi, args.window_size[1] / dpi],
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
