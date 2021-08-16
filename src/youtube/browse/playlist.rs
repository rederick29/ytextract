use std::time::Duration;

use serde::Deserialize;
use serde_with::serde_as;

use crate::youtube::{ChannelNameRuns, ContinuationItemRenderer, Runs, Thumbnails, TitleRun};

pub type Result = super::Result<Ok>;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ok {
    pub contents: Contents,
    pub microformat: Microformat,
    pub sidebar: Sidebar,
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Contents {
    pub two_column_browse_results_renderer: TwoColumnBrowseResultsRenderer,
}

impl Contents {
    pub fn into_videos(self) -> impl Iterator<Item = PlaylistItem> {
        self.two_column_browse_results_renderer
            .tabs
            .0
            .tab_renderer
            .content
            .section_list_renderer
            .contents
            .0
            .item_section_renderer
            .contents
            .0
            .playlist_video_list_renderer
            .contents
            .into_iter()
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TwoColumnBrowseResultsRenderer {
    pub tabs: (Tab,),
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Tab {
    pub tab_renderer: TabRenderer,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TabRenderer {
    pub content: Content,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub section_list_renderer: SectionListRenderer,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SectionListRenderer {
    pub contents: (Content2,),
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Content2 {
    pub item_section_renderer: ItemSectionRenderer,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemSectionRenderer {
    pub contents: (Content3,),
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Content3 {
    #[serde(default)]
    pub playlist_video_list_renderer: PlaylistVideoListRenderer,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistVideoListRenderer {
    pub contents: Vec<PlaylistItem>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum PlaylistItem {
    PlaylistVideoRenderer(PlaylistVideoRenderer),
    ContinuationItemRenderer(ContinuationItemRenderer),
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase", untagged)]
pub enum PlaylistVideoRenderer {
    Ok(PlaylistVideo),
    Err {
        title: Runs,
        #[serde(rename = "videoId")]
        video_id: crate::video::Id,
    },
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistVideo {
    pub video_id: crate::video::Id,

    pub thumbnail: Thumbnails,
    pub title: Runs,
    pub short_byline_text: ChannelNameRuns,

    #[serde_as(as = "serde_with::DurationSeconds<String>")]
    pub length_seconds: Duration,
}

////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Microformat {
    pub microformat_data_renderer: MicroformatDataRenderer,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MicroformatDataRenderer {
    pub url_canonical: String,
    pub title: String,
    pub description: String,
    pub thumbnail: Thumbnails,
    pub unlisted: bool,
}
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Sidebar {
    pub playlist_sidebar_renderer: PlaylistSidebarRenderer,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistSidebarRenderer {
    pub items: PlaylistSidebarItems,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistSidebarItems(
    pub PlaylistSidebarPrimaryInfo,
    #[serde(default)] pub Option<PlaylistSidebarSecondaryInfo>,
);

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistSidebarPrimaryInfo {
    pub playlist_sidebar_primary_info_renderer: PlaylistSidebarPrimaryInfoRenderer,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistSidebarSecondaryInfo {
    pub playlist_sidebar_secondary_info_renderer: PlaylistSidebarSecondaryInfoRenderer,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistSidebarPrimaryInfoRenderer {
    pub stats: (VideoStats, ViewsStats, DateStats),
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoStats {
    pub runs: Vec<TitleRun>,
}

impl VideoStats {
    pub fn as_number(&self) -> u64 {
        match self.runs[0].text.as_str() {
            "No videos" => 0,
            o => o
                .replace(',', "")
                .parse()
                .expect("VideoStats text was not a number"),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ViewsStats {
    pub simple_text: String,
}

impl ViewsStats {
    pub fn as_number(&self) -> u64 {
        self.simple_text
            .as_str()
            .split_once(' ')
            .expect("No space in ViewsStats text")
            .0
            .replace(',', "")
            .parse()
            .unwrap_or_default()
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DateStats {}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistSidebarSecondaryInfoRenderer {
    pub video_owner: VideoOwner,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoOwner {
    pub video_owner_renderer: VideoOwnerRenderer,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoOwnerRenderer {
    pub title: ChannelNameRuns,
}

impl VideoOwnerRenderer {
    pub fn name(&self) -> &str {
        &self.title.runs[0].text
    }

    pub fn id(&self) -> crate::channel::Id {
        self.title.runs[0]
            .navigation_endpoint
            .browse_endpoint
            .browse_id
    }
}
