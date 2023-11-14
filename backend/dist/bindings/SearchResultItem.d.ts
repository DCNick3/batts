export interface SearchResultItem<T> {
    value: T;
    highlights: Record<string, any>;
}
