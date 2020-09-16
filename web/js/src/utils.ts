import type { Caption } from "./types"

export const initialCaptionState: Caption = {
  id: -1,
  startTimestamp: "",
  endTimestamp: "",
  startSeconds: 0,
  endSeconds: 0,
  text: "",
  manuallySelected: false,
}

export const secondify = (timestamp: string): number =>
  new Date("1970-01-01T00:" + timestamp + "Z").getTime() / 1000
