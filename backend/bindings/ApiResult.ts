export interface ApiError {
    underlying_error: string;
    report: string;
    trace_id: string;
    span_id: string;
}
export type ApiResult<T> = { status: 'Success', payload: T } | { status: 'Error', payload: ApiError };