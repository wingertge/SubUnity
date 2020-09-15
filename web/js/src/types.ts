export interface BaseCaption {
  startSeconds: number
  endSeconds: number
  text: string
}

export interface Caption extends BaseCaption {
  id: number
  startTimestamp: string
  endTimestamp: string
  manuallySelected: boolean
}

export interface CaptionData extends VideoInfo {
  entries: BaseCaption[]
}

export interface CaptionCallbacks {
  captionSelected(id: number): void
  updateCaptionField(id: number, field: string, content: string): void
  deleteCaption(id: number): void
}

export interface VideoInfo {
  videoId: string
  language: string
  videoTitle?: string
  uploaderId?: string
  uploaderName?: string
}
