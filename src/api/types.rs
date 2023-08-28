use std::fmt;

use eyre::Result;
use http::Uri;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Gallery {
    pub id: Id,
    pub media_id: Id,
    pub title: Title,
    pub images: Images,
    pub tags: Vec<Tag>,
    pub num_pages: u32,
    pub num_favorites: u32,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Id {
    Number(u32),
    String(String),
}

#[derive(Debug, Deserialize, Clone)]
pub struct Title {
    pub english: Option<String>,
    pub japanese: Option<String>,
    pub pretty: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Images {
    pub pages: Vec<Image>,
    pub cover: Image,
    pub thumbnail: Image,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Image {
    #[serde(rename = "t")]
    pub ty: ImageType,
    #[serde(rename = "w")]
    pub width: u32,
    #[serde(rename = "h")]
    pub height: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub enum ImageType {
    #[serde(rename = "j")]
    Jpeg,
    #[serde(rename = "p")]
    Png,
    Other(String),
}

#[derive(Debug, Deserialize, Clone)]
pub struct Tag {
    pub id: u32,
    #[serde(flatten)]
    pub ty: TagType,
    #[serde(with = "http_serde::uri")]
    pub url: Uri,
    pub count: u32,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase", tag = "type", content = "name")]
pub enum TagType {
    Artist(String),
    Category(String),
    Character(String),
    Group(String),
    Language(String),
    Parody(String),
    Tag(String),
}

impl Gallery {
    pub fn page_urls(&self) -> impl Iterator<Item = Result<Uri>> + '_ {
        (1..).zip(self.images.pages.iter()).map(|(pg_num, img)| {
            Uri::builder()
                .scheme("https")
                .authority("i.nhentai.net")
                .path_and_query(format!(
                    "/galleries/{}/{}{}",
                    self.media_id, pg_num, img.ty
                ))
                .build()
                .map_err(From::from)
        })
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(num) => write!(f, "{num}"),
            Self::String(s) => write!(f, "{s}"),
        }
    }
}

impl fmt::Display for ImageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            ".{}",
            match self {
                Self::Jpeg => "jpg",
                Self::Png => "png",
                Self::Other(ext) => ext,
            }
        )
    }
}
