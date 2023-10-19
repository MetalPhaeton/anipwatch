use crate::{
    error::Error,
    settings::*,
    state::*,
    skin::Skin,
    display_view::{DisplayView, DisplayNumber},
    switch_btn_view::SwitchBtnView,
    button_view::ButtonView,
    animation_view::AnimationView,
    save_data::SaveData
};

use chobitlibs::{
    chobit_hash::fnv_1a_64,
    chobit_map::ChobitMap,
    chobit_rand::ChobitRand,
    chobit_ani_value::ChobitAniValue
};

use image;

use eframe::egui::{
    Context,
    TextureHandle,
    ColorImage,
    Rect,
    Pos2,
    Vec2
};

use std::{
    fmt,
    path::Path,
    rc::Rc,
    cell::RefCell,
    fs::OpenOptions,
    io::{BufReader, Read}
};

use chrono::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum LoadError {
    //ConflictedTextureName,
    StopwatchEventError(EventError),
    ClockEventError(EventError),
    TextureNotFound(String),
    InvalidDefaultMode(String),
    CorruptedSaveData(String)
}

impl fmt::Display for LoadError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, r#"{{"error": "LoadError", "type": "#)?;

        match self {
            //Self::ConflictedTextureName => {
            //    write!(formatter, r#""ConflictedTextureName""#)?;
            //},

            Self::StopwatchEventError(error) => {
                write!(formatter, r#""StopwatchEventError", "data": "#)?;
                <EventError as fmt::Display>::fmt(error, formatter)?;
            },

            Self::ClockEventError(error) => {
                write!(formatter, r#""ClockEventError", "data": "#)?;
                <EventError as fmt::Display>::fmt(error, formatter)?;
            },

            Self::TextureNotFound(name) => {
                write!(formatter, r#""TextureNotFound", "name": {}"#, name)?;
            },

            Self::InvalidDefaultMode(mode) => {
                write!(
                    formatter,
                    r#""InvalidDefaultMode", "mode": {}"#,
                    mode
                )?;
            },

            Self::CorruptedSaveData(file_path) => {
                write!(
                    formatter,
                    r#""CorruptedSaveData", "file_path": {}"#,
                    file_path
                )?;
            },
        }

        write!(formatter, "}}")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EventError {
    //SkinID(u64),
    Centiseconds(u32),
    Seconds(u32),
    Minutes(u32),
    Hours(u32)
}

impl fmt::Display for EventError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, r#"{{"error": "EventError", "type": "#)?;

        match self {
            //Self::SkinID(id) => {
            //    write!(formatter, r#""SkinID", "id": {}"#, id)?;
            //},

            Self::Centiseconds(time) => {
                write!(formatter, r#""Centiseconds", "time": {}"#, time)?;
            },

            Self::Seconds(time) => {
                write!(formatter, r#""Seconds", "time": {}"#, time)?;
            },

            Self::Minutes(time) => {
                write!(formatter, r#""Minutes", "time": {}"#, time)?;
            },

            Self::Hours(time) => {
                write!(formatter, r#""Hours", "time": {}"#, time)?;
            },
        }

        write!(formatter, "}}")
    }
}

pub struct SettingsLoader;

const TABLE_SIZE: usize = 32;

impl SettingsLoader {
    #[inline]
    pub fn load_window_size(window_size: &WindowSize) -> Result<Vec2, Error> {
        Ok(Vec2::new(window_size.width, window_size.height))
    }

    pub fn load_save_data_file<P1: AsRef<Path>, P2: AsRef<Path>>(
        root_dir: &P1,
        save_data_file: &P2
    ) -> Result<SaveData, Error> {
        let path = root_dir.as_ref().join(save_data_file.as_ref());

        let mut file =
            BufReader::new(OpenOptions::new().read(true).open(&path)?);

        let mut data = Vec::<u8>::new();
        file.read_to_end(&mut data)?;

        SaveData::from_bytes(data.as_slice()).ok_or_else (
            || Error::from(LoadError::CorruptedSaveData(
                path.to_string_lossy().into_owned()
            ))
        )
    }

    pub fn load_textures<P: AsRef<Path>>(
        ctx: &Context,
        root_dir: &P,
        textures: &Vec<TextureElement>
    ) -> Result<ChobitMap<TextureHandle>, Error> {
        let mut map = ChobitMap::<TextureHandle>::new(TABLE_SIZE);

        for elm in textures.as_slice() {
            let key = fnv_1a_64(elm.name.as_bytes());
            let texture =
                Self::load_texture_from_path(ctx, root_dir, &elm.path)?;

            map.add(key, texture)?;
        }

        Ok(map)
    }

    fn load_texture_from_path<P1: AsRef<Path>, P2: AsRef<Path>>(
        ctx: &Context,
        root_dir: &P1,
        path: &P2
    ) -> Result<TextureHandle, Error> {
        let path = root_dir.as_ref().join(path.as_ref());

        let img = image::io::Reader::open(&path)?.decode()?;
        let size = [img.width() as usize, img.height() as usize];

        let img_buf = img.to_rgba8();
        let pixels = img_buf.as_flat_samples();

        let color_img = ColorImage::from_rgba_unmultiplied(
            size,
            pixels.as_slice()
        );

        Ok(ctx.load_texture("textures", color_img, Default::default()))
    }

    pub fn load_skins(
        skins: &Vec<SkinElement>,
        textures: &ChobitMap<TextureHandle>
    ) -> Result<ChobitMap<Rc<RefCell<Skin>>>, Error> {
        let mut ret = ChobitMap::<Rc<RefCell<Skin>>>::new(TABLE_SIZE);

        for elm in skins.as_slice() {
            let skin_id = fnv_1a_64(elm.name.as_bytes());

            ret.add(
                skin_id,
                Rc::new(RefCell::new(
                    Self::load_one_skin(elm, textures)?
                ))
            )?;
        }

        Ok(ret)
    }

    fn load_one_skin(
        skin_elm: &SkinElement,
        textures: &ChobitMap<TextureHandle>
    ) -> Result<Skin, Error> {
        let mut skin = Skin::new();

        skin.add(Box::new(Self::load_display_view(
            &skin_elm.display,
            textures
        )?));

        skin.add(Box::new(Self::load_switch_btn_view(
            &skin_elm.switch_button,
            textures
        )?));

        skin.add(Box::new(Self::load_start_stop_btn_view(
            &skin_elm.start_stop_button,
            textures
        )?));

        skin.add(Box::new(Self::load_reset_btn_view(
            &skin_elm.reset_button,
            textures
        )?));

        skin.add(Box::new(Self::load_quit_btn_view(
            &skin_elm.quit_button,
            textures
        )?));

        let rng = Rc::new(RefCell::new(Self::gen_chobit_rand()));

        for elm in skin_elm.animations.as_slice() {
            skin.add(Box::new(Self::load_animation_view(
                elm,
                textures,
                rng.clone()
            )?));
        }

        Ok(skin)
    }

    fn region_to_rect(region: &Region) -> Result<Rect, Error> {
        Ok(Rect::from_min_size(
            Pos2::new(region.x, region.y),
            Vec2::new(region.width, region.height),
        ))
    }

    fn load_display_view(
        display_elm: &DisplayElement,
        textures: &ChobitMap<TextureHandle>
    ) -> Result<DisplayView, Error> {
        let key = fnv_1a_64(display_elm.texture_name.as_bytes());

        let texture_id = textures.get(key).ok_or_else(|| 
            LoadError::TextureNotFound(display_elm.texture_name.clone())
        )?.id();

        let depth = display_elm.depth;

        Ok(DisplayView::new(
            depth,
            texture_id,
            match &display_elm.regions.centiseconds {
                Some(number_region) =>
                    Some(Self::load_display_number(&number_region)?),
                None => None
            },
            match &display_elm.regions.seconds {
                Some(number_region) =>
                    Some(Self::load_display_number(&number_region)?),
                None => None
            },
            match &display_elm.regions.minutes {
                Some(number_region) =>
                    Some(Self::load_display_number(&number_region)?),
                None => None
            },
            match &display_elm.regions.hours {
                Some(number_region) =>
                    Some(Self::load_display_number(&number_region)?),
                None => None
            }
        ))
    }

    fn load_display_number(
        number_region: &NumberRegion
    ) -> Result<DisplayNumber, Error> {
        Ok(DisplayNumber::new(
            Self::region_to_rect(&number_region.d1)?,
            Self::region_to_rect(&number_region.d2)?
        ))
    }

    fn load_switch_btn_view(
        switch_elm: &SwitchBtnElement,
        textures: &ChobitMap<TextureHandle>
    ) -> Result<SwitchBtnView, Error> {
        let key = fnv_1a_64(switch_elm.texture_name.as_bytes());

        let texture_id = textures.get(key).ok_or_else(|| 
            LoadError::TextureNotFound(switch_elm.texture_name.clone())
        )?.id();

        let depth = switch_elm.depth;

        Ok(SwitchBtnView::new(
            depth,
            texture_id,
            Self::region_to_rect(&switch_elm.region)?
        ))
    }

    fn load_start_stop_btn_view(
        start_stop_elm: &StartStopBtnElement,
        textures: &ChobitMap<TextureHandle>
    ) -> Result<ButtonView, Error> {
        let key = fnv_1a_64(start_stop_elm.texture_name.as_bytes());

        let texture_id = textures.get(key).ok_or_else(|| 
            LoadError::TextureNotFound(start_stop_elm.texture_name.clone())
        )?.id();

        let depth = start_stop_elm.depth;

        Ok(ButtonView::new(
            depth,
            texture_id,
            Self::region_to_rect(&start_stop_elm.region)?,
            Button::StartStop
        ))
    }

    fn load_reset_btn_view(
        reset_elm: &ResetBtnElement,
        textures: &ChobitMap<TextureHandle>
    ) -> Result<ButtonView, Error> {
        let key = fnv_1a_64(reset_elm.texture_name.as_bytes());

        let texture_id = textures.get(key).ok_or_else(|| 
            LoadError::TextureNotFound(reset_elm.texture_name.clone())
        )?.id();

        let depth = reset_elm.depth;

        Ok(ButtonView::new(
            depth,
            texture_id,
            Self::region_to_rect(&reset_elm.region)?,
            Button::Reset
        ))
    }

    fn load_quit_btn_view(
        quit_elm: &QuitBtnElement,
        textures: &ChobitMap<TextureHandle>
    ) -> Result<ButtonView, Error> {
        let key = fnv_1a_64(quit_elm.texture_name.as_bytes());

        let texture_id = textures.get(key).ok_or_else(|| 
            LoadError::TextureNotFound(quit_elm.texture_name.clone())
        )?.id();

        let depth = quit_elm.depth;

        Ok(ButtonView::new(
            depth,
            texture_id,
            Self::region_to_rect(&quit_elm.region)?,
            Button::Quit
        ))
    }

    #[inline]
    fn gen_chobit_rand() -> ChobitRand {
        let date: DateTime<Utc> = Utc::now();
        let timestamp = date.timestamp();

        ChobitRand::new(&timestamp.to_ne_bytes())
    }

    fn load_animation_view(
        anim_elm: &AnimationElement,
        textures: &ChobitMap<TextureHandle>,
        rng: Rc<RefCell<ChobitRand>>
    ) -> Result<AnimationView, Error> {
        let key = fnv_1a_64(anim_elm.texture_name.as_bytes());

        let texture_id = textures.get(key).ok_or_else(|| 
            LoadError::TextureNotFound(anim_elm.texture_name.clone())
        )?.id();

        Ok(AnimationView::new(
            anim_elm.depth,
            texture_id,
            Self::region_to_rect(&anim_elm.region)?,
            ChobitAniValue::new(
                anim_elm.frames,
                &[anim_elm.frames],
                anim_elm.fps
            )?,
            anim_elm.probability,
            rng
        ))
    }

    pub fn load_default_mode(mode: &str) -> Result<WatchMode, Error> {
        if mode == "stopwatch" {
            Ok(WatchMode::Stopwatch(StopwatchMode::Stopped))
        } else if mode == "clock" {
            Ok(WatchMode::Clock)
        } else {
            Err(Error::from(LoadError::InvalidDefaultMode(String::from(mode))))
        }
    }

    pub fn load_stopwatch_events(
        events: &Vec<EventElement>
    ) -> Result<Vec<SkinSwitchEvent>, Error> {
        let mut ret = Vec::<SkinSwitchEvent>::with_capacity(
            events.len()
        );

        for elm in events.as_slice() {
            let skin_id = fnv_1a_64(elm.skin_name.as_bytes());

            let from = {
                let cents = elm.from.centiseconds;
                if cents > 99 {
                    return Err(Error::from(
                        LoadError::StopwatchEventError(
                            EventError::Centiseconds(cents)
                        )
                    ));
                }

                let seconds = elm.from.seconds;
                if seconds > 59 {
                    return Err(Error::from(
                        LoadError::StopwatchEventError(
                            EventError::Seconds(seconds)
                        )
                    ));
                }

                let minutes = elm.from.minutes;
                if minutes > 59 {
                    return Err(Error::from(
                        LoadError::StopwatchEventError(
                            EventError::Minutes(minutes)
                        )
                    ));
                }

                let hours = elm.from.hours;
                if hours > 99 {
                    return Err(Error::from(
                        LoadError::StopwatchEventError(
                            EventError::Hours(hours)
                        )
                    ));
                }

                WatchTime {
                    cents: cents,
                    seconds: seconds,
                    minutes: minutes,
                    hours: hours
                }
            };

            ret.push(SkinSwitchEvent {
                skin_id: skin_id,
                from_time: from
            })
        }

        Ok(ret)
    }

    pub fn load_clock_events(
        events: &Vec<EventElement>
    ) -> Result<Vec<SkinSwitchEvent>, Error> {
        let mut ret = Vec::<SkinSwitchEvent>::with_capacity(
            events.len()
        );

        for elm in events.as_slice() {
            let skin_id = fnv_1a_64(elm.skin_name.as_bytes());

            let from = {
                let cents = elm.from.centiseconds;
                if cents > 99 {
                    return Err(Error::from(
                        LoadError::ClockEventError(
                            EventError::Centiseconds(cents)
                        )
                    ));
                }

                let seconds = elm.from.seconds;
                if seconds > 59 {
                    return Err(Error::from(
                        LoadError::ClockEventError(
                            EventError::Seconds(seconds)
                        )
                    ));
                }

                let minutes = elm.from.minutes;
                if minutes > 59 {
                    return Err(Error::from(
                        LoadError::ClockEventError(
                            EventError::Minutes(minutes)
                        )
                    ));
                }

                let hours = elm.from.hours;
                if hours > 99 {
                    return Err(Error::from(
                        LoadError::ClockEventError(
                            EventError::Hours(hours)
                        )
                    ));
                }

                WatchTime {
                    cents: cents,
                    seconds: seconds,
                    minutes: minutes,
                    hours: hours
                }
            };

            ret.push(SkinSwitchEvent {
                skin_id: skin_id,
                from_time: from
            })
        }

        Ok(ret)
    }
}
