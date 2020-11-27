import { createContext, Context } from "preact"
import { nanoid } from "nanoid"
import { Notyf } from "notyf"

import type { Caption } from "./types"

export const initialCaptionState: Caption = {
  id: nanoid(),
  startTimestamp: "00:00.0",
  endTimestamp: "00:00.0",
  startSeconds: 0,
  endSeconds: 0,
  text: ""
}

/**
 * Convert seconds to a human friendly timestamp
 *
 * @param {number} seconds
 * @param {string} format
 */
export const timestampify = (
  seconds: number,
  format: "short" | "long"
): string => {
  let workingString = new Date(1000 * seconds).toISOString().substring(11, 21)

  if (format === "short") {
    if (seconds > 600) {
      return workingString.substring(3)
    } else {
      return workingString.substring(4)
    }
  } else {
    return workingString.substring(1)
  }
}

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
