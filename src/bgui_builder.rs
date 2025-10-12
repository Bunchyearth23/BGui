use glfw::{Monitor, PWindow};

use crate::prelude::*;

enum WinMode {
    FULL,
    WINDOW,
}

pub struct BGuiBuilder {
    x: Option<u32>,
    y: Option<u32>,
    title: Option<String>,
    mode: Option<WinMode>,
}

impl BGuiBuilder {
    pub fn new() -> Self {
        Self {
            x: None,
            y: None,
            title: None,
            mode: None,
        }
    }

    pub fn build(self) -> BGui {
        let mut context = glfw::init(glfw::fail_on_errors).expect("Cannot init");

        let mode = self.mode.expect("Missing Mode");

        let res = match mode {
            WinMode::FULL => context.with_primary_monitor(|gl, m| {
                gl.create_window(
                    self.x.expect("Missing width"),
                    self.y.expect("Missing Height"),
                    self.title.unwrap_or("No Title".to_string()).as_str(),
                    glfw::WindowMode::FullScreen(m.expect("Missing primary monitor")),
                )
            }),
            WinMode::WINDOW => context.create_window(
                self.x.expect("Missing width"),
                self.y.expect("Missing Height"),
                self.title.unwrap_or("No Title".to_string()).as_str(),
                glfw::WindowMode::Windowed,
            ),
        };

        let window = res.expect("Failed window Init");

        BGui { display: window.0 }
    }
}
