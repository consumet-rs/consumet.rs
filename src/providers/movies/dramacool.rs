use super::dramacool_html::{create_html_fragment, Page, Search};
use crate::models::{IMovieEpisode, IMovieInfo, IMovieResult, ISearch, ISource, StreamingServers};

use serde::Deserialize;

// Contains all the DramaCool Info
pub struct DramaCool;

#[derive(Debug, Deserialize)]
pub struct DramaCoolServerInfo {
    link: String,
}

#[derive(Debug)]
pub struct DramaCoolInfo {
    pub base: IMovieResult,
    pub info: IMovieInfo,
}

const BASE_URL: &'static str = "https://dramacool.hr";

impl DramaCool {
    pub async fn search(
        &self,
        query: &str,
        page: Option<usize>,
    ) -> anyhow::Result<ISearch<IMovieResult>> {
        let page = page.unwrap_or(1);

        let parsed_query = query.replace(' ', "-");
        let page_html = reqwest::Client::new()
            .get(format!(
                "{}/search?keyword={}&page={}",
                BASE_URL, parsed_query, page
            ))
            .send()
            .await?
            .text()
            .await?;

        let fragment = create_html_fragment(&page_html);

        let page_parser = Page { elements: fragment };

        let ids = page_parser.page_ids();

        let mut results = vec![];

        for id in ids.iter().flatten() {
            let result = self.fetch_search_result(id).await?;

            results.push(result);
        }

        Ok(ISearch {
            current_page: Some(page),
            has_next_page: page_parser.has_next_page(),
            total_pages: page_parser.total_pages(),
            total_results: results.len(),
            results,
        })
    }

    /// Returns a future which resolves into an movie result object (*[`impl Future<Output = Result<IMovieResult>>`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/types.rs#L452-L462)*)\
    /// # Parameters
    /// * `id` - the id of the provided drama
    async fn fetch_search_result(&self, id: &str) -> anyhow::Result<IMovieResult> {
        let url = format!("{}/{}", BASE_URL, id);

        let media_html = reqwest::Client::new()
            .get(&url)
            .send()
            .await?
            .text()
            .await?;

        let fragment = create_html_fragment(&media_html);

        let search_parser = Search {
            elements: &fragment,
            id,
        };

        Ok(IMovieResult {
            cover: None,
            title: search_parser.search_title(),
            other_names: search_parser.search_other_names(),
            url: Some(url),
            image: search_parser.search_image(),
            release_date: search_parser.search_release_date(),
            media_type: None,
            id: Some(id.to_string()),
        })
    }

    pub async fn info(&self, _media_id: &str) -> anyhow::Result<DramaCoolInfo> {
        todo!()
    }

    pub async fn servers(
        &self,
        _episode_id: &str,
        _media_id: &str,
    ) -> anyhow::Result<Vec<IMovieEpisode>> {
        todo!()
    }

    pub async fn sources(
        &self,
        _episode_id: &str,
        _media_id: &str,
        _server: Option<StreamingServers>,
    ) -> anyhow::Result<ISource> {
        todo!()
    }
}
