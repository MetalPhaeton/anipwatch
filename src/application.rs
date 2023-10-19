use crate::{
    error::Error,
    state::*,
    settings::Settings,
    settings_loader::SettingsLoader,
    save_data::SaveData,
    skin::Skin,
    model::Model
};

use eframe::{
    App,
    CreationContext,
    Frame,
    egui::{
        Ui,
        Context,
        Visuals,
        Rgba,
        Vec2,
        TextureHandle,
        Color32,
        CentralPanel,
        Frame as GuiFrame
    }
};

use getopts::Options;

use std::{
    fmt,
    path::{Path, PathBuf},
    fs::OpenOptions,
    io::{BufReader, Write, BufWriter},
    rc::Rc,
    cell::RefCell,
    process::ExitCode
};

use chobitlibs::{
    chobit_map::ChobitMap,
    chobit_hash::fnv_1a_64
};

#[derive(Debug, Clone, PartialEq)]
pub enum ApplicationError {
    CommandOptionError(String),
    OnlyShowHelp(String),
    CouldNotConvertPath(String),
    CouldNotParseSettingsFile
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, r#"{{"error": "EventError", "type": "#)?;

        match self {
            Self::CommandOptionError(help) => {
                write!(
                    formatter,
                    r#""CommandOptionError", "help": {:?}"#,
                    help
                )?;
            },

            Self::OnlyShowHelp(help) => {
                write!(
                    formatter,
                    r#""OnlyShowHelp", "help": {:?}"#,
                    help
                )?;
            },

            Self::CouldNotConvertPath(path) => {
                write!(
                    formatter,
                    r#""CouldNotConvertPath", "path": {:?}"#,
                    path
                )?;
            },

            Self::CouldNotParseSettingsFile => {
                write!(
                    formatter,
                    r#""CouldNotParseSettingsFile""#
                )?;
            },
        }

        write!(formatter, "}}")
    }
}

pub struct Application {
    state: State,
    window_size: Vec2,
    _textures: ChobitMap<TextureHandle>,
    skins: ChobitMap<Rc<RefCell<Skin>>>,
    current_skin: Rc<RefCell<Skin>>,
    model: Model,

    exit_code: Rc<RefCell<ExitCode>>
}

impl Application {
    pub fn new(
        args: &[String],
        cc: &CreationContext,
        exit_code: Rc<RefCell<ExitCode>>
    ) -> Result<Self, Error> {
        let (mut state, window_size, textures, skins) =
            Self::load_settings(args, cc)?;

        let mut model = Model::new();
        model.init(&mut state)?;

        let current_skin = skins.get(state.current_skin_id).ok_or_else(
            || Error::NoSkin {id: state.current_skin_id}
        )?.clone();

        state.init();

        Ok(Self {
            state: state,
            window_size: window_size,
            _textures: textures,
            skins: skins,
            current_skin: current_skin,
            model: model,

            exit_code: exit_code
        })
    }

    fn load_settings(
        args: &[String],
        cc: &CreationContext
    ) -> Result<(
        State,
        Vec2,
        ChobitMap<TextureHandle>,
        ChobitMap<Rc<RefCell<Skin>>>,
    ), Error> {
        let opts = Self::gen_options();

        let matches = opts.parse(&args[1..]).or_else(
            |_| Err(Error::from(
                ApplicationError::CommandOptionError(
                    Self::gen_usage(&opts, &args[0])
                )
            ))
        )?;

        if matches.opt_present("h") {
            return Err(Error::from(ApplicationError::OnlyShowHelp(
                Self::gen_usage(&opts, &args[0])
            )));
        }

        let settings_file_name: &String = matches.free.get(0).ok_or_else(
            || Error::from(
                ApplicationError::CommandOptionError(
                    Self::gen_usage(&opts, &args[0])
                )
            )
        )?;

        let reader = BufReader::new(
            OpenOptions::new().read(true).open(&settings_file_name)?
        );

        let settings: Settings = serde_yaml::from_reader(reader).or_else(
            |_| Err(Error::from(ApplicationError::CouldNotParseSettingsFile))
        )?;

        let root_dir = Self::get_root_dir(&settings_file_name)?;

        let window_size =
            SettingsLoader::load_window_size(&settings.window_size)?;

        let textures = SettingsLoader::load_textures(
            &cc.egui_ctx,
            &root_dir,
            &settings.textures
        )?;

        let skins = SettingsLoader::load_skins(&settings.skins, &textures)?;

        let state = Self::settings_to_state(
            &settings,
            root_dir
        )?;

        Ok((state, window_size, textures, skins))
    }

    fn gen_options() -> Options {
        let mut ret = Options::new();

        ret.optflag("h", "help", "print usage");

        ret
    }

    #[inline]
    fn gen_usage(opts: &Options, program_name: &str) -> String {
        opts.usage(format!(
            "{} SETTINGS_FILE",
            program_name
        ).as_str())
    }

    #[inline]
    fn get_root_dir<'a>(
        settings_file_name: &'a str
    ) -> Result<&'a Path, Error> {
        Path::new(settings_file_name).parent().ok_or_else(
            || Error::from(ApplicationError::CouldNotConvertPath(
                settings_file_name.to_string()
            ))
        )
    }

    fn settings_to_state(
        settings: &Settings,
        root_dir: &Path
    ) -> Result<State, Error> {
        let save_data_path = PathBuf::from(&settings.save_data_file);

        let (
            watch_mode,
            stopwatch_time,
            saved_time,
        ) = match SettingsLoader::load_save_data_file(
            &root_dir,
            &save_data_path
        ) {
            Ok(SaveData {
                watch_mode,
                stopwatch_time,
                saved_time,
                ..
            }) => (watch_mode, stopwatch_time, saved_time),

            Err(..) => (
                SettingsLoader::load_default_mode(
                    &settings.default_mode
                )?,
                WatchTime::default(),
                0.0
            )
        };

        let save_data_path = root_dir.join(&save_data_path);

        Ok(State {
            watch_mode: watch_mode,
            clicked_btn: None,
            dt: Default::default(),
            button_is_pressed: false,

            quit_request: false,

            current_stopwatch_time: stopwatch_time,
            current_clock_time: Default::default(),

            change_skin_request: false,
            current_skin_id: Default::default(),

            saved_time: saved_time,

            default_stopwatch_skin_id:
                fnv_1a_64(settings.default_clock_skin_name.as_bytes()),
            default_clock_skin_id:
                fnv_1a_64(settings.default_clock_skin_name.as_bytes()),

            stopwatch_events: SettingsLoader::load_stopwatch_events(
                &settings.stopwatch_events
            )?,

            clock_events: SettingsLoader::load_clock_events(
                &settings.clock_events
            )?,

            save_data_path: save_data_path
        })
    }

    fn update_core(&mut self, ctx: &Context) -> Result<(), Error> {
        CentralPanel::default().frame(
            GuiFrame::none().fill(Color32::TRANSPARENT)
        ).show::<Result<(), Error>>(ctx, |ui| {
            self.state.dt = Self::get_dt(ui);

            self.model.preproc(&mut self.state)?;

            {
                let mut skin = self.current_skin.try_borrow_mut()?;

                for view in skin.as_mut_slice() {
                    view.ready(ui, &mut self.state)?;
                }
            }

            self.model.update(&mut self.state)?;

            if self.state.change_skin_request {
                self.current_skin = self.skins.get(
                    self.state.current_skin_id
                ).ok_or_else(
                    || Error::NoSkin {id: self.state.current_skin_id}
                )?.clone();
            }

            {
                let mut skin = self.current_skin.try_borrow_mut()?;

                for view in skin.as_mut_slice() {
                    view.show(ui, &mut self.state)?;
                }
            }

            Ok(())
        }).inner
    }

    #[inline]
    fn get_dt(ui: &Ui) -> f32 {
        ui.input(|i_state| i_state.stable_dt)
    }
}

impl App for Application {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        ctx.request_repaint();

        ctx.set_pixels_per_point(1.0);
        frame.set_always_on_top(true);

        frame.set_window_size(self.window_size);

        frame.drag_window();

        if let Err(error) = self.update_core(ctx) {
            error.error_log();

            *self.exit_code.borrow_mut() = ExitCode::FAILURE;
            frame.close();
        }

        if self.state.quit_request {
            *self.exit_code.borrow_mut() = ExitCode::SUCCESS;
            frame.close();
        }
    }

    #[inline]
    fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
        Rgba::TRANSPARENT.to_array()
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        let mut buf = Vec::<u8>::new();

        let mut save_data = SaveData::from_variables(
            self.state.watch_mode,
            self.state.current_stopwatch_time.clone(),
            self.state.saved_time
        );

        save_data.write_bytes(&mut buf);

        let mut file = BufWriter::new(match OpenOptions::new()
            .write(true)
            .create(true)
            .open(&self.state.save_data_path)
        {
            Ok(file) => file,
            Err(..) => {return;}
        });

        let _ = file.write_all(&buf).and_then(|_| file.flush());
    }
}

pub struct DummyApplication;

impl App for DummyApplication {
    #[inline]
    fn update(&mut self, _ctx: &Context, frame: &mut Frame) {
        frame.close();
    }
}
