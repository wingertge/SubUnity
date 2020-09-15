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

export interface CaptionState {
  captions: Caption[]
  activeCaption: Caption
  setCaptions(captions: Caption[]): void
  setActiveCaption(caption: Caption): void
}

export interface CaptionItemCallbacks {
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
