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
  id: string
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

export interface EditorState {
  isEditorDirty: boolean
  setEditorDirty: StateUpdater<boolean>
  syncCaptionStorage(): void
}

export interface CaptionItemCallbacks {
  addCaption(id: string): void
  captionSelected(id: string): void
  updateCaptionField(
    id: string,
    field: EditableCaptionField,
    content: string
  ): void
  deleteCaption(id: string): void
}

export interface VideoInfo {
  videoId: string
  language: string
  videoTitle?: string
  uploaderId?: string
  uploaderName?: string
  isVideoLong?: boolean
}
