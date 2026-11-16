//! hub-app: everything the screen knows. One component value per control on the screen
//! (everyware_ui), the data from the house, the clock.

use crate::clock::Clock;
use everyware_dashboard_grpc::weather::Weather;
use everyware_ui::design::Sizes;
use everyware_ui::button::{Button, ButtonKind, Icon, IconButton, IconButtonKind};
use everyware_ui::list_view::ListView;
use everyware_ui::segmented::Segmented;
use everyware_ui::slider::Slider;
use everyware_ui::toggle::Toggle;

/// The climate setpoint's range and step (°C).
pub const SETPOINT_MIN: f32 = 16.0;
pub const SETPOINT_MAX: f32 = 28.0;
pub const SETPOINT_STEP: f32 = 0.5;

pub struct State {
    pub clock: Clock,
    pub weather: Option<Weather>,
    pub nav_home: IconButton,
    pub nav_scenes: IconButton,
    pub nav_wifi: IconButton,
    pub nav_settings: IconButton,
    pub climate_minus: IconButton,
    pub climate_plus: IconButton,
    pub setpoint: f32,
    pub lights: Toggle,
    pub dimmer: Slider,
    pub fan: Segmented,
    pub talk: Button,
    pub talking: bool,
    pub door: IconButton,
    pub locked: bool,
    pub news: ListView,
}

impl State {
    /// `sizes`: the design's frames (main.rs reads them from the .dcf): the dimmer's touch area
    /// is where the design puts its slot.
    pub fn new(sizes: &Sizes) -> Self {
        let dimmer = sizes.frame("#hub/dimmer").map_or((0.0, 0.0), |f| (f.x, f.width));
        State {
            clock: Clock::default(),
            weather: None,
            nav_home: IconButton::new(IconButtonKind::Tonal, Icon::House),
            nav_scenes: IconButton::new(IconButtonKind::Tonal, Icon::Sliders),
            nav_wifi: IconButton::new(IconButtonKind::Tonal, Icon::Wifi),
            nav_settings: IconButton::new(IconButtonKind::Tonal, Icon::Settings),
            climate_minus: IconButton::new(IconButtonKind::Tonal, Icon::Minus),
            climate_plus: IconButton::new(IconButtonKind::Tonal, Icon::Plus),
            setpoint: 22.0,
            lights: Toggle::new(true),
            dimmer: Slider::in_slot(65.0, dimmer.0, dimmer.1),
            fan: Segmented::new(["Low", "Med", "High"]),
            talk: Button::new(ButtonKind::Primary, "Talk").compact(),
            talking: false,
            door: IconButton::new(IconButtonKind::Tonal, Icon::ChevronRight),
            locked: true,
            news: crate::news::empty(sizes.frame("#hub/news").map_or(0.0, |f| f.height)),
        }
    }
}
