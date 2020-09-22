import { StateUpdater } from "preact/hooks"

declare global {
  interface Window {
    VIDEO_ID: string
    SUBTITLE_LANG: string
  }
}

type EditableCaptionField = "id" | "startTimestamp" | "endTimestamp" | "text"

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
  setCaptions: StateUpdater<Caption[]>
  setActiveCaption: StateUpdater<Caption>
}

export interface CaptionItemCallbacks {
  addCaption(id: number): void
  captionSelected(id: number): void
  updateCaptionField(
    id: number,
    field: EditableCaptionField,
    content: string
  ): void
  deleteCaption(id: number): void
}

export interface VideoInfo {
  videoId: string
  language: string
  videoTitle?: string
  uploaderId?: string
  uploaderName?: string
}
