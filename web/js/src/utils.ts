import { createContext, Context } from "preact"
import { Notyf } from "notyf"
import type { Caption } from "./types"

export const initialCaptionState: Caption = {
  id: -1,
  startTimestamp: "00:00.0",
  endTimestamp: "00:00.0",
  startSeconds: 0,
  endSeconds: 0,
  text: "",
  manuallySelected: false,
}

/**
 * Convert seconds to a human friendly timestamp
 *
 * @param {number} seconds
 */
export const timestampify = (seconds: number): string =>
  new Date(1000 * seconds).toISOString().substring(14, 21)

/**
 * Convert a timestamp back to seconds
 * @param {string} timestamp
 */
export const secondify = (timestamp: string): number =>
  new Date("1970-01-01T00:" + timestamp + "Z").getTime() / 1000

export const NotyfContext: Context<Notyf> = createContext(
  new Notyf({
    duration: 3000,
    position: { x: "left", y: "bottom" },
    types: [
      {
        type: "error",
        background: "crimson",
        duration: 0,
        dismissible: true,
      },
    ],
  })
)
