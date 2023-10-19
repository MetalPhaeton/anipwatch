#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! See [GitHub Repository](https://github.com/MetalPhaeton/anipwatch)

extern crate image;
extern crate eframe;
extern crate serde;
extern crate serde_yaml;
extern crate getopts;
extern crate chobitlibs;
extern crate log;
extern crate env_logger;

mod error;
use self::error::Error;

mod state;
mod model;

mod view;
mod display_view;
mod switch_btn_view;
mod button_view;
mod animation_view;

mod skin;

mod settings;
mod save_data;
mod settings_loader;

mod application;
use self::application::ApplicationError;

use std::{
    process::ExitCode,
    rc::Rc,
    cell::RefCell,
    env
};

use eframe::{
    NativeOptions,
    IconData
};

use log::error;

use application::{Application, DummyApplication};

fn main() -> ExitCode {
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    let exit_code = Rc::new(RefCell::new(ExitCode::SUCCESS));

    let options = NativeOptions {
        resizable: false,
        decorated: false,
        transparent: true,
        always_on_top: true,
        icon_data: Some(
            IconData::try_from_png_bytes(
                include_bytes!("icon.png")
            ).expect("Error at main::main()")
        ),

        ..Default::default()
    };

    let exit_code_2 = exit_code.clone();
    let exit_code_3 = exit_code.clone();

    match eframe::run_native(
        "application",
        options,
        Box::new(move |cc| {
            match Application::new(
                &args,
                cc,
                exit_code_2
            ) {
                Ok(app) => Box::new(app),

                Err(error) => match error {
                    Error::ApplicationError(
                        ApplicationError::OnlyShowHelp(help)
                    ) => {
                        eprintln!("{}", help);
                        *exit_code_3.borrow_mut() = ExitCode::SUCCESS;
                        Box::new(DummyApplication)
                    },

                    Error::ApplicationError(
                        ApplicationError::CommandOptionError(help)
                    ) => {
                        eprintln!("{}", help);
                        *exit_code_3.borrow_mut() = ExitCode::FAILURE;
                        Box::new(DummyApplication)
                    },

                    _ => {
                        error.error_log();
                        Box::new(DummyApplication)
                    }
                }
            }
        })
    ) {
        Ok(..) => {
            let exit_code = *exit_code.borrow();
            exit_code
        },

        Err(error) => {
            error!("EFrame Error: {}", error);
            ExitCode::FAILURE
        }
    }
}
