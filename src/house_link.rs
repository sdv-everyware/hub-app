//! hub-app: the house's data. The dashboard bundle (vendor/sdv-everyware/everyware/bundles/
//! dashboard) pushes it over binder to com.everyware.screen.IScreenLink/default, which the panel
//! app serves; the hub takes the same name (one of the two runs at a time) through the shared
//! link (vendor/sdv-everyware/everyware/interfaces/screen/link). The hub shows the weather and
//! the news; everything else the bundle sends is dropped by the link's defaults.

use crate::actions::HubAction;
use everyware_catalog::news::{NewsFeed, NewsThumbnail};
use everyware_catalog::weather::Weather;
use everyware_screen_link::Screen;
use harry::reducer::Actions;
use std::sync::mpsc::Sender;
use std::sync::Mutex;

struct Hub {
    actions: Mutex<Sender<Actions<HubAction>>>,
}

impl Hub {
    fn act(&self, action: HubAction) {
        let _ = self.actions.lock().expect("actions").send(Actions::CustomAction(action));
    }
}

impl Screen for Hub {
    fn weather(&self, weather: Weather) {
        self.act(HubAction::Weather(weather));
    }

    fn news(&self, news: NewsFeed) {
        self.act(HubAction::News(news));
    }

    fn news_thumbnail(&self, thumbnail: NewsThumbnail) {
        crate::news::set_picture(&thumbnail);
        self.act(HubAction::Redraw);
    }
}

/// Serves the link until the process ends; returns at once.
pub fn spawn(actions: Sender<Actions<HubAction>>) {
    everyware_screen_link::spawn(Hub { actions: Mutex::new(actions) });
}
