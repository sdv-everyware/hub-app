//! hub-app: the UI model, one field per layer of #hub-root the app writes to or taps
//! (design: design/hub-patch.js). The names are the
//! contract with the design: rename both or neither.

use everyware_ui::model::prelude::*;

everyware_ui::ui_model! {
    time: String = "#hub/time";
    date: String = "#hub/date";
    weather_icon: Slot = "#hub/weather/icon" ["SlotCustomizer"];
    weather_temp: String = "#hub/weather/temp";
    weather_condition: String = "#hub/weather/condition";
    weather_place: String = "#hub/weather/place";
    weather_chart: Slot = "#hub/weather/chart" ["SlotCustomizer"];
    weather_t1: String = "#hub/weather/t1";
    weather_t2: String = "#hub/weather/t2";
    weather_t3: String = "#hub/weather/t3";
    nav_home: Slot = "#hub/nav/home" ["SlotCustomizer"];
    nav_scenes: Slot = "#hub/nav/scenes" ["SlotCustomizer"];
    nav_wifi: Slot = "#hub/nav/wifi" ["SlotCustomizer"];
    nav_settings: Slot = "#hub/nav/settings" ["SlotCustomizer"];
    climate_minus: Slot = "#hub/climate/minus" ["SlotCustomizer"];
    climate_plus: Slot = "#hub/climate/plus" ["SlotCustomizer"];
    climate_setpoint: String = "#hub/climate/setpoint";
    lights: Slot = "#hub/lights" ["SlotCustomizer"];
    lights_state: String = "#hub/lights/state";
    dimmer: Slot = "#hub/dimmer" ["SlotCustomizer"];
    dimmer_value: String = "#hub/dimmer/value";
    fan: Slot = "#hub/fan" ["SlotCustomizer"];
    fan_state: String = "#hub/fan/state";
    talk: Slot = "#hub/talk" ["SlotCustomizer"];
    door: Slot = "#hub/door" ["SlotCustomizer"];
    door_state: String = "#hub/door/state";
    news: Slot = "#hub/news" interactive ["SlotCustomizer"];
    news_scrollbar: Slot = "#hub/news/scrollbar" ["SlotCustomizer"];
}
