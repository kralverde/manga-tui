use strum::{Display, IntoEnumIterator};

pub trait IntoParam {
    fn into_param(self) -> String;
}

#[derive(Display, Clone)]
pub enum ContentRating {
    #[strum(to_string = "safe")]
    Safe,
    #[strum(to_string = "suggestive")]
    Suggestive,
    #[strum(to_string = "erotica")]
    Erotic,
    #[strum(to_string = "pornographic")]
    Pornographic,
}

impl From<&str> for ContentRating {
    fn from(value: &str) -> Self {
        match value {
            "safe" => Self::Safe,
            "suggestive" => Self::Suggestive,
            "erotica" => Self::Erotic,
            "pornographic" => Self::Pornographic,
            _ => Self::Safe,
        }
    }
}

#[derive(Display, Clone, strum_macros::EnumIter, PartialEq, Eq)]
pub enum SortBy {
    #[strum(to_string = "Best match")]
    BestMatch,
    #[strum(to_string = "Latest upload")]
    LatestUpload,
    #[strum(to_string = "Oldest upload")]
    OldestUpload,
    #[strum(to_string = "Highest rating")]
    HighestRating,
    #[strum(to_string = "Lowest rating")]
    LowestRating,
    #[strum(to_string = "Title ascending")]
    TitleAscending,
    #[strum(to_string = "Title descending")]
    TitleDescending,
    #[strum(to_string = "Oldest added")]
    OldestAdded,
    #[strum(to_string = "Recently added")]
    RecentlyAdded,
    #[strum(to_string = "Most follows")]
    MostFollows,
    #[strum(to_string = "Fewest follows")]
    FewestFollows,
    #[strum(to_string = "Year descending")]
    YearDescending,
    #[strum(to_string = "Year ascending")]
    YearAscending,
}

impl IntoParam for Vec<ContentRating> {
    fn into_param(self) -> String {
        let mut result = String::new();

        if self.is_empty() {
            return format!("contentRating[]={}", ContentRating::Safe);
        }

        for cont in self {
            result.push_str(format!("contentRating[]={}&", cont).as_str());
        }

        result.pop();

        result
    }
}

impl From<&str> for SortBy {
    fn from(value: &str) -> Self {
        SortBy::iter()
            .find(|sort_by| sort_by.to_string() == value)
            .unwrap()
    }
}

impl IntoParam for SortBy {
    fn into_param(self) -> String {
        match self {
            Self::BestMatch => "order[relevance]=desc".to_string(),
            Self::LatestUpload => "order[latestUploadedChapter]=desc".to_string(),
            Self::OldestUpload => "order[latestUploadedChapter]=asc".to_string(),
            Self::OldestAdded => "order[createdAt]=asc".to_string(),
            Self::MostFollows => "order[followedCount]=desc".to_string(),
            Self::LowestRating => "order[rating]=asc".to_string(),
            Self::HighestRating => "order[rating]=desc".to_string(),
            Self::RecentlyAdded => "order[createdAt]=desc".to_string(),
            Self::FewestFollows => "order[followedCount]=asc".to_string(),
            Self::TitleAscending => "order[title]=asc".to_string(),
            Self::TitleDescending => "order[title]=desc".to_string(),
            Self::YearAscending => "order[year]=asc".to_string(),
            Self::YearDescending => "order[year]=desc".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct Filters {
    pub content_rating: Vec<ContentRating>,
    pub sort_by: SortBy,
}

impl IntoParam for Filters {
    fn into_param(self) -> String {
        format!(
            "{}&{}",
            self.content_rating.into_param(),
            self.sort_by.into_param()
        )
    }
}

impl Default for Filters {
    fn default() -> Self {
        Self {
            content_rating: vec![ContentRating::Safe, ContentRating::Suggestive],
            sort_by: SortBy::BestMatch,
        }
    }
}

impl Filters {
    pub fn set_content_rating(&mut self, ratings: Vec<ContentRating>) {
        self.content_rating = ratings;
    }

    pub fn set_sort_by(&mut self, sort_by: SortBy) {
        self.sort_by = sort_by;
    }
}