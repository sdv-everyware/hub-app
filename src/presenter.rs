//! hub-app: state -> UI model, after every action.

use crate::state::State;
use crate::ui_model::UiModel;
use crate::weather::{bars, words, Picture};
use everyware_ui::{list, scrollbar, slot};
use harry::presentation::Presenter;

pub struct UiModelPresenter;

impl Presenter<State, UiModel> for UiModelPresenter {
    fn present(model: &mut UiModel, state: &State) {
        model.time = state.clock.time.clone();
        model.date = state.clock.date.clone();
        present_weather(model, state);

        model.nav_home = slot("#hub/nav/home", &state.nav_home);
        model.nav_scenes = slot("#hub/nav/scenes", &state.nav_scenes);
        model.nav_wifi = slot("#hub/nav/wifi", &state.nav_wifi);
        model.nav_settings = slot("#hub/nav/settings", &state.nav_settings);

        model.climate_minus = slot("#hub/climate/minus", &state.climate_minus);
        model.climate_plus = slot("#hub/climate/plus", &state.climate_plus);
        model.climate_setpoint = format!("{:.1}°", state.setpoint).replace(".0°", "°");

        model.lights = slot("#hub/lights", &state.lights);
        model.lights_state = if state.lights.value { "Lights on" } else { "Lights off" }.to_string();
        model.dimmer = slot("#hub/dimmer", &state.dimmer);
        model.dimmer_value = format!("Lights {:.0}%", state.dimmer.value);
        model.fan = slot("#hub/fan", &state.fan);
        model.fan_state = format!("On · {}", state.fan.value());

        model.talk = slot("#hub/talk", &state.talk);
        model.door = slot("#hub/door", &state.door);
        model.door_state = if state.locked { "Locked" } else { "Unlocked" }.to_string();

        model.news = list("#hub/news", &state.news);
        model.news_scrollbar = scrollbar("#hub/news/scrollbar", &state.news);
    }
}

/// The weather now (top row) and the next 24 hours (tile). Before the first message the design's
/// preview stays and the condition says the weather is on its way.
fn present_weather(model: &mut UiModel, state: &State) {
    let Some(w) = &state.weather else {
        model.weather_temp = "–".to_string();
        model.weather_condition = "Waiting for the weather".to_string();
        model.weather_place = String::new();
        return;
    };
    model.weather_icon = slot("#hub/weather/icon", &Picture::of(w.code));
    model.weather_temp = format!("{:.0}°", w.temperature_c);
    model.weather_condition = words(w.code).to_string();
    model.weather_place = format!("{} · feels {:.0}° · {:.0} km/h", w.place, w.apparent_c, w.wind_kmh);
    let chart = bars(w.hourly.iter().map(|h| (h.temperature_c, h.precipitation_percent)));
    model.weather_chart = slot("#hub/weather/chart", &chart);
    let label = |i: usize| w.hourly.get(i).map(|h| format!("{:02}:00", h.hour)).unwrap_or_default();
    model.weather_t1 = label(0);
    model.weather_t2 = label(12);
    model.weather_t3 = label(23);
}
