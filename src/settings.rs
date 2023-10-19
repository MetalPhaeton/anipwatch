use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Settings {
    pub window_size: WindowSize,

    pub save_data_file: String,

    pub textures: Vec<TextureElement>,

    pub skins: Vec<SkinElement>,
    pub default_mode: String,

    pub default_stopwatch_skin_name: String,
    pub default_clock_skin_name: String,

    pub stopwatch_events: Vec<EventElement>,
    pub clock_events: Vec<EventElement>
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct WindowSize {
    pub width: f32,
    pub height: f32
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TextureElement {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Region {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SkinElement {
    pub name: String,

    pub display: DisplayElement,
    pub switch_button: SwitchBtnElement,
    pub start_stop_button: StartStopBtnElement,
    pub reset_button: ResetBtnElement,
    pub quit_button: QuitBtnElement,
    pub animations: Vec<AnimationElement>
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DisplayElement {
    pub texture_name: String,
    pub depth: i32,
    pub regions: DisplayRegions
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DisplayRegions {
    pub centiseconds: Option<NumberRegion>,
    pub seconds: Option<NumberRegion>,
    pub minutes: Option<NumberRegion>,
    pub hours: Option<NumberRegion>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct NumberRegion {
    pub d1: Region,
    pub d2: Region
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SwitchBtnElement {
    pub texture_name: String,
    pub depth: i32,
    pub region: Region
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct StartStopBtnElement {
    pub texture_name: String,
    pub depth: i32,
    pub region: Region
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ResetBtnElement {
    pub texture_name: String,
    pub depth: i32,
    pub region: Region
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct QuitBtnElement {
    pub texture_name: String,
    pub depth: i32,
    pub region: Region
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AnimationElement {
    pub texture_name: String,
    pub depth: i32,
    pub region: Region,
    pub frames: usize,
    pub fps: f32,
    pub probability: Option<f32>
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TimeElement {
    pub centiseconds: u32,
    pub seconds: u32,
    pub minutes: u32,
    pub hours: u32
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct EventElement {
    pub skin_name: String,

    pub from: TimeElement,
}

#[cfg(test)]
mod tests {
    use super::*;

    const YAML: &str = 
r#"
window_size:
    width: 111
    height: 222

save_data_file: "SaveDataFile!"

textures:
    -
        name: "Texture1!"
        path: "Path1!"
    -
        name: "Texture2!"
        path: "Path2!"
skins:
    -
        name: "Skin1!"
        display:
            texture_name: "Texture1!"
            depth: 0
            regions:
                centiseconds:
                    d1:
                        x: 1.0
                        y: 2.0
                        width: 3.0
                        height: 4.0
                    d2:
                        x: 5.0
                        y: 6.0
                        width: 7.0
                        height: 8.0
                seconds:
                    d1:
                        x: 9.0
                        y: 10.0
                        width: 11.0
                        height: 12.0
                    d2:
                        x: 13.0
                        y: 14.0
                        width: 15.0
                        height: 16.0
                minutes:
                    d1:
                        x: 17.0
                        y: 18.0
                        width: 19.0
                        height: 20.0
                    d2:
                        x: 20.0
                        y: 21.0
                        width: 22.0
                        height: 23.0
                hours:
                    d1:
                        x: 24.0
                        y: 25.0
                        width: 26.0
                        height: 27.0
                    d2:
                        x: 29.0
                        y: 30.0
                        width: 31.0
                        height: 32.0
        switch_button:
            texture_name: "Texture2!"
            depth: 1
            region:
                x: 33.0
                y: 34.0
                width: 35.0
                height: 36.0
        start_stop_button:
            texture_name: "Texture3!"
            depth: 2
            region:
                x: 37.0
                y: 38.0
                width: 39.0
                height: 40.0
        reset_button:
            texture_name: "Texture4!"
            depth: 3
            region:
                x: 41.0
                y: 42.0
                width: 43.0
                height: 44.0
        quit_button:
            texture_name: "Texture5!"
            depth: 4
            region:
                x: 45.0
                y: 46.0
                width: 47.0
                height: 48.0
        animations:
            -
                texture_name: "Texture6!"
                depth: 5
                region:
                    x: 49.0
                    y: 51.0
                    width: 52.0
                    height: 53.0
                frames: 1
                fps: 1.0
                probability: 1.0
            -
                texture_name: "Texture7!"
                depth: 6
                region:
                    x: 54.0
                    y: 55.0
                    width: 56.0
                    height: 57.0
                frames: 2
                fps: 2.0
    -
        name: "Skin2!"
        display:
            texture_name: "Texture8!"
            depth: 7
            regions:
                seconds:
                    d1:
                        x: 58.0
                        y: 59.0
                        width: 60.0
                        height: 61.0
                    d2:
                        x: 62.0
                        y: 63.0
                        width: 64.0
                        height: 65.0
                hours:
                    d1:
                        x: 66.0
                        y: 67.0
                        width: 68.0
                        height: 69.0
                    d2:
                        x: 70.0
                        y: 71.0
                        width: 72.0
                        height: 73.0
        switch_button:
            texture_name: "Texture9!"
            depth: 8
            region:
                x: 74.0
                y: 75.0
                width: 76.0
                height: 77.0
        start_stop_button:
            texture_name: "Texture10!"
            depth: 9
            region:
                x: 78.0
                y: 79.0
                width: 80.0
                height: 81.0
        reset_button:
            texture_name: "Texture11!"
            depth: 10
            region:
                x: 82.0
                y: 83.0
                width: 84.0
                height: 85.0
        quit_button:
            texture_name: "Texture12!"
            depth: 11
            region:
                x: 86.0
                y: 87.0
                width: 88.0
                height: 89.0
        animations:
            -
                texture_name: "Texture13!"
                depth: 12
                region:
                    x: 90.0
                    y: 91.0
                    width: 92.0
                    height: 93.0
                frames: 3
                fps: 3.0
                probability: 3.0

default_mode: "Clock!"
default_stopwatch_skin_name: "Default-Stopwatch-Skin!"
default_clock_skin_name: "Default-Clock-Skin!"
stopwatch_events:
    -
        skin_name: "Skin1!"
        from:
            centiseconds: 0
            seconds: 1
            minutes: 2
            hours: 3
    -
        skin_name: "Skin2!"
        from:
            centiseconds: 8
            seconds: 9
            minutes: 10
            hours: 11
clock_events:
    -
        skin_name: "Skin3!"
        from:
            centiseconds: 16
            seconds: 17
            minutes: 18
            hours: 19
"#;

    #[test]
    fn skin_settings_test_1() {
        let settings: Settings = serde_yaml::from_str(YAML).unwrap();
        assert_eq!(
            settings.window_size.width,
            111.0
        );
        assert_eq!(
            settings.window_size.height,
            222.0
        );

        assert_eq!(
            settings.save_data_file.as_str(),
            "SaveDataFile!"
        );
        assert_eq!(
            settings.textures.len(),
            2
        );
        assert_eq!(
            settings.textures[0].name,
            "Texture1!"
        );
        assert_eq!(
            settings.textures[0].path,
            "Path1!"
        );
        assert_eq!(
            settings.textures[1].name,
            "Texture2!"
        );
        assert_eq!(
            settings.textures[1].path,
            "Path2!"
        );
        assert_eq!(
            settings.skins.len(),
            2
        );
        assert_eq!(
            settings.skins[0].name,
            "Skin1!"
        );
        assert_eq!(
            settings.skins[0].display.texture_name,
            "Texture1!"
        );
        assert_eq!(
            settings.skins[0].display.depth,
            0
        );
        assert_eq!(
            settings
                .skins[0]
                .display
                .regions
                .centiseconds
                .as_ref()
                .unwrap()
                .d1
                .x,
            1.0
        );
        assert_eq!(
            settings
                .skins[0]
                .display
                .regions
                .centiseconds
                .as_ref()
                .unwrap()
                .d1
                .y,
            2.0
        );
        assert_eq!(
            settings
                .skins[0]
                .display
                .regions
                .centiseconds
                .as_ref()
                .unwrap()
                .d1
                .width,
            3.0
        );
        assert_eq!(
            settings
                .skins[0]
                .display
                .regions
                .centiseconds
                .as_ref()
                .unwrap()
                .d1
                .height,
            4.0
        );
        assert_eq!(
            settings
                .skins[0]
                .display
                .regions
                .centiseconds
                .as_ref()
                .unwrap()
                .d2
                .x,
            5.0
        );
        assert_eq!(
            settings
                .skins[0]
                .display
                .regions
                .centiseconds
                .as_ref()
                .unwrap()
                .d2
                .y,
            6.0
        );
        assert_eq!(
            settings
                .skins[0]
                .display
                .regions
                .centiseconds
                .as_ref()
                .unwrap()
                .d2
                .width,
            7.0
        );
        assert_eq!(
            settings
                .skins[0]
                .display
                .regions
                .centiseconds
                .as_ref()
                .unwrap()
                .d2
                .height,
            8.0
        );
        assert_eq!(
            settings
                .skins[0]
                .display
                .regions
                .seconds
                .as_ref()
                .unwrap()
                .d1
                .x,
            9.0
        );
        assert_eq!(
            settings
                .skins[0]
                .display
                .regions
                .seconds
                .as_ref()
                .unwrap()
                .d1
                .y,
            10.0
        );
        assert_eq!(
            settings
                .skins[0]
                .display
                .regions
                .seconds
                .as_ref()
                .unwrap()
                .d1
                .width,
            11.0
        );
        assert_eq!(
            settings
                .skins[0]
                .display
                .regions
                .seconds
                .as_ref()
                .unwrap()
                .d1
                .height,
            12.0
        );
        assert_eq!(
            settings
                .skins[0]
                .display
                .regions
                .seconds
                .as_ref()
                .unwrap()
                .d2
                .x,
            13.0
        );
        assert_eq!(
            settings
                .skins[0]
                .display
                .regions
                .seconds
                .as_ref()
                .unwrap()
                .d2
                .y,
            14.0
        );
        assert_eq!(
            settings
                .skins[0]
                .display
                .regions
                .seconds
                .as_ref()
                .unwrap()
                .d2
                .width,
            15.0
        );
        assert_eq!(
            settings
                .skins[0]
                .display
                .regions
                .seconds
                .as_ref()
                .unwrap()
                .d2
                .height,
            16.0
        );
        assert_eq!(
            settings
                .skins[0]
                .display
                .regions
                .minutes
                .as_ref()
                .unwrap()
                .d1
                .x,
            17.0
        );
        assert_eq!(
            settings
                .skins[0]
                .display
                .regions
                .minutes
                .as_ref()
                .unwrap()
                .d1
                .y,
            18.0
        );
        assert_eq!(
            settings
                .skins[0]
                .display
                .regions
                .minutes
                .as_ref()
                .unwrap()
                .d1
                .width,
            19.0
        );
        assert_eq!(
            settings
                .skins[0]
                .display
                .regions
                .minutes
                .as_ref()
                .unwrap()
                .d1
                .height,
            20.0
        );
        assert_eq!(
            settings
                .skins[0]
                .display
                .regions
                .minutes
                .as_ref()
                .unwrap()
                .d2
                .x,
            20.0
        );
        assert_eq!(
            settings
                .skins[0]
                .display
                .regions
                .minutes
                .as_ref()
                .unwrap()
                .d2
                .y,
            21.0
        );
        assert_eq!(
            settings
                .skins[0]
                .display
                .regions
                .minutes
                .as_ref()
                .unwrap()
                .d2
                .width,
            22.0
        );
        assert_eq!(
            settings
                .skins[0]
                .display
                .regions
                .minutes
                .as_ref()
                .unwrap()
                .d2
                .height,
            23.0
        );
        assert_eq!(
            settings
                .skins[0]
                .display
                .regions
                .hours
                .as_ref()
                .unwrap()
                .d1
                .x,
            24.0
        );
        assert_eq!(
            settings
                .skins[0]
                .display
                .regions
                .hours
                .as_ref()
                .unwrap()
                .d1
                .y,
            25.0
        );
        assert_eq!(
            settings
                .skins[0]
                .display
                .regions
                .hours
                .as_ref()
                .unwrap()
                .d1
                .width,
            26.0
        );
        assert_eq!(
            settings
                .skins[0]
                .display
                .regions
                .hours
                .as_ref()
                .unwrap()
                .d1
                .height,
            27.0
        );
        assert_eq!(
            settings
                .skins[0]
                .display
                .regions
                .hours
                .as_ref()
                .unwrap()
                .d2
                .x,
            29.0
        );
        assert_eq!(
            settings
                .skins[0]
                .display
                .regions
                .hours
                .as_ref()
                .unwrap()
                .d2
                .y,
            30.0
        );
        assert_eq!(
            settings
                .skins[0]
                .display
                .regions
                .hours
                .as_ref()
                .unwrap()
                .d2
                .width,
            31.0
        );
        assert_eq!(
            settings
                .skins[0]
                .display
                .regions
                .hours
                .as_ref()
                .unwrap()
                .d2
                .height,
            32.0
        );
        assert_eq!(
            settings.skins[0].switch_button.texture_name,
            "Texture2!"
        );
        assert_eq!(
            settings.skins[0].switch_button.depth,
            1
        );
        assert_eq!(
            settings.skins[0].switch_button.region.x,
            33.0
        );
        assert_eq!(
            settings.skins[0].switch_button.region.y,
            34.0
        );
        assert_eq!(
            settings.skins[0].switch_button.region.width,
            35.0
        );
        assert_eq!(
            settings.skins[0].switch_button.region.height,
            36.0
        );
        assert_eq!(
            settings.skins[0].start_stop_button.texture_name,
            "Texture3!"
        );
        assert_eq!(
            settings.skins[0].start_stop_button.depth,
            2
        );
        assert_eq!(
            settings.skins[0].start_stop_button.region.x,
            37.0
        );
        assert_eq!(
            settings.skins[0].start_stop_button.region.y,
            38.0
        );
        assert_eq!(
            settings.skins[0].start_stop_button.region.width,
            39.0
        );
        assert_eq!(
            settings.skins[0].start_stop_button.region.height,
            40.0
        );
        assert_eq!(
            settings.skins[0].reset_button.texture_name,
            "Texture4!"
        );
        assert_eq!(
            settings.skins[0].reset_button.depth,
            3
        );
        assert_eq!(
            settings.skins[0].reset_button.region.x,
            41.0
        );
        assert_eq!(
            settings.skins[0].reset_button.region.y,
            42.0
        );
        assert_eq!(
            settings.skins[0].reset_button.region.width,
            43.0
        );
        assert_eq!(
            settings.skins[0].reset_button.region.height,
            44.0
        );
        assert_eq!(
            settings.skins[0].quit_button.texture_name,
            "Texture5!"
        );
        assert_eq!(
            settings.skins[0].quit_button.depth,
            4
        );
        assert_eq!(
            settings.skins[0].quit_button.region.x,
            45.0
        );
        assert_eq!(
            settings.skins[0].quit_button.region.y,
            46.0
        );
        assert_eq!(
            settings.skins[0].quit_button.region.width,
            47.0
        );
        assert_eq!(
            settings.skins[0].quit_button.region.height,
            48.0
        );
        assert_eq!(
            settings.skins[0].animations.len(),
            2
        );
        assert_eq!(
            settings.skins[0].animations[0].texture_name,
            "Texture6!"
        );
        assert_eq!(
            settings.skins[0].animations[0].depth,
            5
        );
        assert_eq!(
            settings.skins[0].animations[0].region.x,
            49.0
        );
        assert_eq!(
            settings.skins[0].animations[0].region.y,
            51.0
        );
        assert_eq!(
            settings.skins[0].animations[0].region.width,
            52.0
        );
        assert_eq!(
            settings.skins[0].animations[0].region.height,
            53.0
        );
        assert_eq!(
            settings.skins[0].animations[0].frames,
            1
        );
        assert_eq!(
            settings.skins[0].animations[0].fps,
            1.0
        );
        assert_eq!(
            *settings.skins[0].animations[0].probability.as_ref().unwrap(),
            1.0
        );
        assert_eq!(
            settings.skins[0].animations[1].texture_name,
            "Texture7!"
        );
        assert_eq!(
            settings.skins[0].animations[1].depth,
            6
        );
        assert_eq!(
            settings.skins[0].animations[1].region.x,
            54.0
        );
        assert_eq!(
            settings.skins[0].animations[1].region.y,
            55.0
        );
        assert_eq!(
            settings.skins[0].animations[1].region.width,
            56.0
        );
        assert_eq!(
            settings.skins[0].animations[1].region.height,
            57.0
        );
        assert_eq!(
            settings.skins[0].animations[1].frames,
            2
        );
        assert_eq!(
            settings.skins[0].animations[1].fps,
            2.0
        );
        assert!(settings.skins[0].animations[1].probability.is_none());
        assert_eq!(
            settings.skins[1].name,
            "Skin2!"
        );
        assert_eq!(
            settings.skins[1].display.texture_name,
            "Texture8!"
        );
        assert_eq!(
            settings.skins[1].display.depth,
            7
        );
        assert!(settings.skins[1].display.regions.centiseconds.is_none());
        assert_eq!(
            settings
                .skins[1]
                .display
                .regions
                .seconds
                .as_ref()
                .unwrap()
                .d1
                .x,
            58.0
        );
        assert_eq!(
            settings
                .skins[1]
                .display
                .regions
                .seconds
                .as_ref()
                .unwrap()
                .d1
                .y,
            59.0
        );
        assert_eq!(
            settings
                .skins[1]
                .display
                .regions
                .seconds
                .as_ref()
                .unwrap()
                .d1
                .width,
            60.0
        );
        assert_eq!(
            settings
                .skins[1]
                .display
                .regions
                .seconds
                .as_ref()
                .unwrap()
                .d1
                .height,
            61.0
        );
        assert_eq!(
            settings
                .skins[1]
                .display
                .regions
                .seconds
                .as_ref()
                .unwrap()
                .d2
                .x,
            62.0
        );
        assert_eq!(
            settings
                .skins[1]
                .display
                .regions
                .seconds
                .as_ref()
                .unwrap()
                .d2
                .y,
            63.0
        );
        assert_eq!(
            settings
                .skins[1]
                .display
                .regions
                .seconds
                .as_ref()
                .unwrap()
                .d2
                .width,
            64.0
        );
        assert_eq!(
            settings
                .skins[1]
                .display
                .regions
                .seconds
                .as_ref()
                .unwrap()
                .d2
                .height,
            65.0
        );
        assert!(settings.skins[1].display.regions.minutes.is_none());
        assert_eq!(
            settings
                .skins[1]
                .display
                .regions
                .hours
                .as_ref()
                .unwrap()
                .d1
                .x,
            66.0
        );
        assert_eq!(
            settings
                .skins[1]
                .display
                .regions
                .hours
                .as_ref()
                .unwrap()
                .d1
                .y,
            67.0
        );
        assert_eq!(
            settings
                .skins[1]
                .display
                .regions
                .hours
                .as_ref()
                .unwrap()
                .d1
                .width,
            68.0
        );
        assert_eq!(
            settings
                .skins[1]
                .display
                .regions
                .hours
                .as_ref()
                .unwrap()
                .d1
                .height,
            69.0
        );
        assert_eq!(
            settings
                .skins[1]
                .display
                .regions
                .hours
                .as_ref()
                .unwrap()
                .d2
                .x,
            70.0
        );
        assert_eq!(
            settings
                .skins[1]
                .display
                .regions
                .hours
                .as_ref()
                .unwrap()
                .d2
                .y,
            71.0
        );
        assert_eq!(
            settings
                .skins[1]
                .display
                .regions
                .hours
                .as_ref()
                .unwrap()
                .d2
                .width,
            72.0
        );
        assert_eq!(
            settings
                .skins[1]
                .display
                .regions
                .hours
                .as_ref()
                .unwrap()
                .d2
                .height,
            73.0
        );
        assert_eq!(
            settings.skins[1].switch_button.texture_name,
            "Texture9!"
        );
        assert_eq!(
            settings.skins[1].switch_button.depth,
            8
        );
        assert_eq!(
            settings.skins[1].switch_button.region.x,
            74.0
        );
        assert_eq!(
            settings.skins[1].switch_button.region.y,
            75.0
        );
        assert_eq!(
            settings.skins[1].switch_button.region.width,
            76.0
        );
        assert_eq!(
            settings.skins[1].switch_button.region.height,
            77.0
        );
        assert_eq!(
            settings.skins[1].start_stop_button.texture_name,
            "Texture10!"
        );
        assert_eq!(
            settings.skins[1].start_stop_button.depth,
            9
        );
        assert_eq!(
            settings.skins[1].start_stop_button.region.x,
            78.0
        );
        assert_eq!(
            settings.skins[1].start_stop_button.region.y,
            79.0
        );
        assert_eq!(
            settings.skins[1].start_stop_button.region.width,
            80.0
        );
        assert_eq!(
            settings.skins[1].start_stop_button.region.height,
            81.0
        );
        assert_eq!(
            settings.skins[1].reset_button.texture_name,
            "Texture11!"
        );
        assert_eq!(
            settings.skins[1].reset_button.depth,
            10
        );
        assert_eq!(
            settings.skins[1].reset_button.region.x,
            82.0
        );
        assert_eq!(
            settings.skins[1].reset_button.region.y,
            83.0
        );
        assert_eq!(
            settings.skins[1].reset_button.region.width,
            84.0
        );
        assert_eq!(
            settings.skins[1].reset_button.region.height,
            85.0
        );
        assert_eq!(
            settings.skins[1].quit_button.texture_name,
            "Texture12!"
        );
        assert_eq!(
            settings.skins[1].quit_button.depth,
            11
        );
        assert_eq!(
            settings.skins[1].quit_button.region.x,
            86.0
        );
        assert_eq!(
            settings.skins[1].quit_button.region.y,
            87.0
        );
        assert_eq!(
            settings.skins[1].quit_button.region.width,
            88.0
        );
        assert_eq!(
            settings.skins[1].quit_button.region.height,
            89.0
        );
        assert_eq!(
            settings.skins[1].animations.len(),
            1
        );
        assert_eq!(
            settings.skins[1].animations[0].texture_name,
            "Texture13!"
        );
        assert_eq!(
            settings.skins[1].animations[0].depth,
            12
        );
        assert_eq!(
            settings.skins[1].animations[0].region.x,
            90.0
        );
        assert_eq!(
            settings.skins[1].animations[0].region.y,
            91.0
        );
        assert_eq!(
            settings.skins[1].animations[0].region.width,
            92.0
        );
        assert_eq!(
            settings.skins[1].animations[0].region.height,
            93.0
        );
        assert_eq!(
            settings.skins[1].animations[0].frames,
            3
        );
        assert_eq!(
            settings.skins[1].animations[0].fps,
            3.0
        );
        assert_eq!(
            *settings.skins[1].animations[0].probability.as_ref().unwrap(),
            3.0
        );
        assert_eq!(
            settings.skins[1].animations[0].region.x,
            90.0
        );
        assert_eq!(
            settings.default_mode,
            "Clock!"
        );
        assert_eq!(
            settings.default_stopwatch_skin_name,
            "Default-Stopwatch-Skin!"
        );
        assert_eq!(
            settings.default_clock_skin_name,
            "Default-Clock-Skin!"
        );
        assert_eq!(
            settings.stopwatch_events.len(),
            2
        );
        assert_eq!(
            settings.stopwatch_events[0].skin_name,
            "Skin1!"
        );
        assert_eq!(
            settings.stopwatch_events[0].from.centiseconds,
            0
        );
        assert_eq!(
            settings.stopwatch_events[0].from.seconds,
            1
        );
        assert_eq!(
            settings.stopwatch_events[0].from.minutes,
            2
        );
        assert_eq!(
            settings.stopwatch_events[0].from.hours,
            3
        );
        assert_eq!(
            settings.stopwatch_events[1].skin_name,
            "Skin2!"
        );
        assert_eq!(
            settings.stopwatch_events[1].from.centiseconds,
            8
        );
        assert_eq!(
            settings.stopwatch_events[1].from.seconds,
            9
        );
        assert_eq!(
            settings.stopwatch_events[1].from.minutes,
            10
        );
        assert_eq!(
            settings.stopwatch_events[1].from.hours,
            11
        );
        assert_eq!(
            settings.clock_events.len(),
            1
        );
        assert_eq!(
            settings.clock_events[0].skin_name,
            "Skin3!"
        );
        assert_eq!(
            settings.clock_events[0].from.centiseconds,
            16
        );
        assert_eq!(
            settings.clock_events[0].from.seconds,
            17
        );
        assert_eq!(
            settings.clock_events[0].from.minutes,
            18
        );
        assert_eq!(
            settings.clock_events[0].from.hours,
            19
        );
    }
}
