use std::collections::HashMap;

use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::config::ImageQuality;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchMangaResponse {
    pub result: String,
    pub response: String,
    pub data: Vec<Data>,
    pub limit: i32,
    pub offset: u32,
    pub total: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub id: String,
    pub attributes: Attributes,
    pub relationships: Vec<MangaSearchRelationship>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub title: Title,
    pub description: Option<Description>,
    pub status: String,
    pub tags: Vec<Tag>,
    pub content_rating: String,
    pub state: String,
    pub created_at: String,
    pub publication_demographic: Option<String>,
    pub available_translated_languages: Vec<Option<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Title {
    pub en: Option<String>,
    pub ja: Option<String>,
    #[serde(rename = "ja-ro")]
    pub ja_ro: Option<String>,
    pub jp: Option<String>,
    pub zh: Option<String>,
    pub ko: Option<String>,
    #[serde(rename = "zh-ro")]
    pub zh_ro: Option<String>,
    #[serde(rename = "zh-ro")]
    pub ko_ro: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description {
    pub en: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MangaSearchRelationship {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub attributes: Option<MangaSearchAttributes>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MangaSearchAttributes {
    #[serde(rename = "fileName")]
    pub file_name: Option<String>,
    pub name: Option<String>,
    pub locale: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: String,
    pub attributes: TagAtributtes,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagAtributtes {
    pub name: Name,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    pub en: String,
}

// manga chapter structs
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChapterResponse {
    pub result: String,
    pub response: String,
    pub data: Vec<ChapterData>,
    pub limit: i64,
    pub offset: i64,
    pub total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChapterData {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub attributes: ChapterAttribute,
    pub relationships: Vec<Relationship>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChapterAttribute {
    pub volume: Option<String>,
    pub chapter: Option<String>,
    pub title: Option<String>,
    pub translated_language: String,
    pub external_url: Option<String>,
    pub publish_at: String,
    pub readable_at: String,
    pub created_at: String,
    pub updated_at: String,
    pub pages: i64,
    pub version: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Relationship {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub attributes: Option<ChapterRelationshipAttribute>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChapterRelationshipAttribute {
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChapterPagesResponse {
    pub result: String,
    pub base_url: String,
    pub chapter: ChapterPages,
}

impl ChapterPagesResponse {
    /// According to mangadex api the endpoint to get a chapter's panel is built as follows: `base_url`/`data`, data-saver`/`hash`
    pub fn get_image_url_endpoint(&self, quality: ImageQuality) -> String {
        format!("{}/{}/{}", self.base_url, quality.as_param(), self.chapter.hash)
    }

    /// Based on the mangadex api the `data_saver` array is used when image quality is low and
    /// `data` is used when ImageQuality is high
    pub fn get_files_based_on_quality(self, quality: ImageQuality) -> Vec<String> {
        match quality {
            ImageQuality::Low => self.chapter.data_saver,
            ImageQuality::High => self.chapter.data,
        }
    }

    /// Based on the mangadex api the `data_saver` array is used when image quality is low and
    /// `data` is used when ImageQuality is high
    pub fn get_files_based_on_quality_as_url(self, quality: ImageQuality) -> Vec<Url> {
        let base_endpoint = self.get_image_url_endpoint(quality);

        let endpoint_formatted = |raw_url: String| format!("{base_endpoint}/{}", raw_url).parse::<Url>();

        match quality {
            ImageQuality::Low => self
                .chapter
                .data_saver
                .into_iter()
                .map(endpoint_formatted)
                .filter_map(|res| res.ok())
                .collect(),
            ImageQuality::High => self.chapter.data.into_iter().map(endpoint_formatted).filter_map(|res| res.ok()).collect(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChapterPages {
    pub hash: String,
    pub data: Vec<String>,
    pub data_saver: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MangaStatisticsResponse {
    pub result: String,
    pub statistics: HashMap<String, Statistics>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
    pub rating: Rating,
    pub follows: Option<u64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rating {
    pub average: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregateChapterResponse {
    pub result: String,
    pub volumes: HashMap<String, Volumes>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Volumes {
    pub volume: String,
    pub count: i32,
    pub chapters: HashMap<String, Chapters>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chapters {
    pub chapter: String,
    pub id: String,
    pub others: Vec<String>,
    pub count: i32,
}

pub mod feed {
    use serde::{Deserialize, Serialize};

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OneMangaResponse {
        pub result: String,
        pub response: String,
        pub data: super::Data,
    }
}

pub mod tags {
    use serde::{Deserialize, Serialize};

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TagsResponse {
        pub result: String,
        pub response: String,
        pub data: Vec<TagsData>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TagsData {
        pub id: String,
        #[serde(rename = "type")]
        pub type_field: String,
        pub attributes: Attributes,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Attributes {
        pub name: Name,
        pub group: String,
        pub version: i64,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Name {
        pub en: String,
    }
}

pub mod authors {
    use serde::{Deserialize, Serialize};

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AuthorsResponse {
        pub result: String,
        pub response: String,
        pub data: Vec<Data>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Data {
        pub id: String,
        #[serde(rename = "type")]
        pub type_field: String,
        pub attributes: Attributes,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Attributes {
        pub name: String,
        pub created_at: String,
        pub updated_at: String,
        pub version: i64,
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OneChapterResponse {
    pub data: OneChapterData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OneChapterData {
    pub id: String,
    pub attributes: ChapterAttribute,
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use uuid::Uuid;

    use super::*;

    #[test]
    fn it_constructs_manga_panel_endpoint_based_on_image_quality() {
        let mut response = ChapterPagesResponse::default();

        response.chapter.data_saver = vec!["low_quality1.jpg".into(), "low_quality2.jpg".into()];
        response.chapter.data = vec!["high_quality1.jpg".into(), "high_quality2.jpg".into()];

        response.chapter.hash = "the_hash".to_string();
        response.base_url = "http://localhost".to_string();

        let image_quality = ImageQuality::Low;

        let expected: Url =
            format!("{}/{}/{}/low_quality1.jpg", response.base_url, image_quality.as_param(), response.chapter.hash,)
                .parse()
                .unwrap();

        assert_eq!(&expected, response.clone().get_files_based_on_quality_as_url(image_quality).first().unwrap());

        let image_quality = ImageQuality::High;

        let expected: Url =
            format!("{}/{}/{}/high_quality1.jpg", response.base_url, image_quality.as_param(), response.chapter.hash)
                .parse()
                .unwrap();

        assert_eq!(&expected, response.clone().get_files_based_on_quality_as_url(image_quality).first().unwrap());
    }

    #[test]
    fn endpoint_to_obtain_a_chapter_panel_is_built_correctly() {
        let response = ChapterPagesResponse {
            base_url: "http://some_url".to_string(),
            chapter: ChapterPages {
                hash: Uuid::new_v4().to_string(),
                ..Default::default()
            },
            ..Default::default()
        };

        let image_quality = ImageQuality::High;
        assert_eq!(format!("http://some_url/data/{}", response.chapter.hash), response.get_image_url_endpoint(image_quality));

        let image_quality = ImageQuality::Low;
        assert_eq!(format!("http://some_url/data-saver/{}", response.chapter.hash), response.get_image_url_endpoint(image_quality));
    }
}