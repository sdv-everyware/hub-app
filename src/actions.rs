//! hub-app: every change of the state is an action: the clock's tick, a gesture on a node, the
//! house's data. One apply() for all of them; a gesture goes to the component that lives at
//! the node, which answers with an event or none.

use crate::clock::Clock;
use crate::state::{State, SETPOINT_MAX, SETPOINT_MIN, SETPOINT_STEP};
use everyware_catalog::news::NewsFeed;
use everyware_catalog::weather::Weather;
use everyware_ui::button::ButtonEvent;
use everyware_ui::list_view::ListEvent;
use everyware_ui::segmented::SegmentedEvent;
use everyware_ui::slider::SliderEvent;
use everyware_ui::toggle::ToggleEvent;
use everyware_ui::{part_of, row_of, Gesture};
use harry::side_effects::SideEffects;
use harry::state::StateAction;
use std::time::Instant;

#[derive(Clone, Debug, PartialEq)]
pub enum HubAction {
    /// Once a second from the ticker thread (main.rs).
    Tick(Clock),
    /// A gesture on a node (ui_event_listener.rs).
    Gesture(String, Gesture),
    /// The weather of the house (house_link.rs).
    Weather(Weather),
    /// The news feed (house_link.rs).
    News(NewsFeed),
    /// A story's picture arrived: draw again.
    Redraw,
}

impl StateAction<State, ()> for HubAction {
    fn apply(&self, state: &mut State, _side_effects: &mut Vec<SideEffects<()>>) {
        match self {
            HubAction::Tick(now) => state.clock = now.clone(),
            HubAction::Weather(w) => state.weather = Some(w.clone()),
            HubAction::News(feed) => {
                let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs() as i64);
                state.news.set_rows(crate::news::rows(feed, now));
            }
            HubAction::Redraw => {}
            HubAction::Gesture(node, gesture) => on_gesture(state, node, *gesture),
        }
    }
}

fn on_gesture(state: &mut State, node: &str, gesture: Gesture) {
    // where a finger lands (the node harry hit-tests), for a control that does not react
    if matches!(gesture, Gesture::Down { .. }) {
        log::info!("hub: down on {node}: {gesture:?}");
    }
    // the news list: the slot itself (a drag between rows) or a row, "#hub/news/<row>"
    if node == "#hub/news" || node.starts_with("#hub/news/") {
        if let Some(ListEvent::Chosen(row)) = state.news.on(row_of("#hub/news", node), gesture, Instant::now()) {
            state.news.select(Some(row));
            log::info!("hub: story {row} chosen");
        }
        return;
    }
    // the fan's segments, "#hub/fan/#segmented/segment-<1..3>" (the part with its slot)
    if let Some(n) = part_of("#hub/fan", node).and_then(|p| p.strip_prefix("#segmented/segment-")).and_then(|n| n.parse::<usize>().ok()) {
        if let Some(SegmentedEvent::Selected(i)) = n.checked_sub(1).and_then(|i| state.fan.on(i, gesture)) {
            log::info!("hub: fan {}", ["low", "medium", "high"][i.min(2)]);
        }
        return;
    }
    match node {
        "#hub/lights" => {
            if let Some(ToggleEvent::Toggled(on)) = state.lights.on(gesture) {
                log::info!("hub: lights {}", if on { "on" } else { "off" });
            }
        }
        "#hub/dimmer" => {
            if let Some(SliderEvent::Changed(percent)) = state.dimmer.on(gesture) {
                log::info!("hub: dimmer {percent:.0} %");
            }
        }
        "#hub/climate/minus" => {
            if state.climate_minus.on(gesture) == Some(ButtonEvent::Clicked) {
                state.setpoint = (state.setpoint - SETPOINT_STEP).max(SETPOINT_MIN);
            }
        }
        "#hub/climate/plus" => {
            if state.climate_plus.on(gesture) == Some(ButtonEvent::Clicked) {
                state.setpoint = (state.setpoint + SETPOINT_STEP).min(SETPOINT_MAX);
            }
        }
        "#hub/talk" => {
            if state.talk.on(gesture) == Some(ButtonEvent::Clicked) {
                state.talking = !state.talking;
            }
        }
        "#hub/door" => {
            if state.door.on(gesture) == Some(ButtonEvent::Clicked) {
                state.locked = !state.locked;
            }
        }
        "#hub/nav/home" => {
            if state.nav_home.on(gesture) == Some(ButtonEvent::Clicked) {
                log::info!("hub: home");
            }
        }
        "#hub/nav/scenes" => {
            if state.nav_scenes.on(gesture) == Some(ButtonEvent::Clicked) {
                log::info!("hub: scenes");
            }
        }
        "#hub/nav/wifi" => {
            if state.nav_wifi.on(gesture) == Some(ButtonEvent::Clicked) {
                log::info!("hub: Wi-Fi");
            }
        }
        "#hub/nav/settings" => {
            if state.nav_settings.on(gesture) == Some(ButtonEvent::Clicked) {
                log::info!("hub: settings");
            }
        }
        _ => {}
    }
}
