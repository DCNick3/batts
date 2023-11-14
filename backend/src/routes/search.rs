use crate::api_result::ApiResult;
use crate::domain::group::GroupView;
use crate::domain::ticket::TicketView;
use crate::domain::user::UserView;
use crate::error::MeilisearchSnafu;
use crate::extractors::Query;
use crate::state::ApplicationState;
use axum::extract::State;
use meilisearch_sdk::Selectors;
use serde::{Deserialize, Serialize};
use snafu::ResultExt;
use ts_rs::TS;

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    q: String,
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct SearchResultItem<T> {
    value: T,
    #[ts(type = "Record<string, any>")]
    highlights: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct SearchResults<T> {
    top_hits: Vec<SearchResultItem<T>>,
}

impl<T> SearchResults<T> {
    fn from_meili(results: meilisearch_sdk::SearchResults<T>) -> Self {
        Self {
            top_hits: results
                .hits
                .into_iter()
                .map(|hit| SearchResultItem {
                    value: hit.result,
                    highlights: hit.formatted_result.unwrap(),
                })
                .collect(),
        }
    }
}

pub async fn tickets(
    State(state): State<ApplicationState>,
    Query(SearchQuery { q: query }): Query<SearchQuery>,
) -> ApiResult<SearchResults<TicketView>> {
    ApiResult::from_async_fn(|| async {
        state
            .search
            .ticket_index
            .search()
            .with_query(&query)
            .with_filter(r#"lifecycle_state = "Created""#)
            .with_attributes_to_highlight(Selectors::All)
            .execute()
            .await
            .context(MeilisearchSnafu)
            .map(SearchResults::from_meili)
    })
    .await
}

pub async fn users(
    State(state): State<ApplicationState>,
    Query(SearchQuery { q: query }): Query<SearchQuery>,
) -> ApiResult<SearchResults<UserView>> {
    ApiResult::from_async_fn(|| async {
        state
            .search
            .user_index
            .search()
            .with_query(&query)
            .with_filter(r#"lifecycle_state = "Created""#)
            .with_attributes_to_highlight(Selectors::All)
            .execute()
            .await
            .context(MeilisearchSnafu)
            .map(SearchResults::from_meili)
    })
    .await
}

pub async fn groups(
    State(state): State<ApplicationState>,
    Query(SearchQuery { q: query }): Query<SearchQuery>,
) -> ApiResult<SearchResults<GroupView>> {
    ApiResult::from_async_fn(|| async {
        state
            .search
            .group_index
            .search()
            .with_query(&query)
            .with_filter(r#"lifecycle_state = "Created""#)
            .with_attributes_to_highlight(Selectors::All)
            .execute()
            .await
            .context(MeilisearchSnafu)
            .map(SearchResults::from_meili)
    })
    .await
}
