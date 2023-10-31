export interface UploadPolicy {
    allowed_file_extensions: Array<string>;
    allowed_content_types: Array<string>;
    max_size: bigint;
}
