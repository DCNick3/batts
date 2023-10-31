import type { UploadId } from "./UploadId";
export interface InitiatedUpload {
    id: UploadId;
    url: string;
    fields: Record<string, string>;
    expiration: string;
}
