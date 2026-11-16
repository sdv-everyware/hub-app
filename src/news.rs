//! hub-app: the news column: one #list-item row (image, three lines, column) per story.

use everyware_dashboard_grpc::news::{NewsFeed, NewsStory, NewsThumbnail};
use everyware_ui::images;
use everyware_ui::list_item::{ListItem, ListItemKind, ListItemLines, ListItemSize};
use everyware_ui::list_view::ListView;

/// The rows of the list's slot in the design (hub-patch.js): 128 + 8 apart.
pub const ROW_PITCH: f32 = 128.0 + 8.0;

/// The feed before the first message: no rows, as high as the design's slot (#hub/news).
pub fn empty(height: f32) -> ListView {
    ListView::new([], height, ROW_PITCH)
}

pub fn rows(feed: &NewsFeed, now: i64) -> Vec<ListItem> {
    feed.stories.iter().map(|s| row(s, now)).collect()
}

fn row(story: &NewsStory, now: i64) -> ListItem {
    let supporting = if story.summary.is_empty() { format!("{} · {} points", story.site, story.points) } else { story.summary.clone() };
    ListItem::new(ListItemKind::Image, story.title.clone(), supporting)
        .lines(ListItemLines::Three)
        .size(ListItemSize::Column)
        .image(picture_key(story.id))
        .trailing_text(age(now - story.time))
}

fn picture_key(id: u64) -> String {
    format!("news/{id}")
}

/// A story's picture, registered under its key: the row shows it by that key.
pub fn set_picture(thumbnail: &NewsThumbnail) {
    if let Err(err) = images::register_rgb(picture_key(thumbnail.id), &thumbnail.rgb, thumbnail.width, thumbnail.height) {
        log::warn!("hub: thumbnail of story {}: {err}", thumbnail.id);
    }
}

fn age(secs: i64) -> String {
    let min = secs.max(0) / 60;
    match min {
        0..=59 => format!("{} min", min.max(1)),
        60..=1439 => format!("{} h", min / 60),
        _ => format!("{} d", min / 1440),
    }
}
