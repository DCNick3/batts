import type { SearchResultItem } from "./SearchResultItem";
export interface SearchResults<T> {
    top_hits: Array<SearchResultItem<T>>;
}
