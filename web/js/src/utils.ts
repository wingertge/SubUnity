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
 * Convert seconds to a human friendly timestamp.
 *
 * Timestamps will be formatted to either show the hour portion or omit it.
 * If a video is less than ten minutes long the leading zero will also
 * not be shown.
 *
 * These timestamps are modeled after YouTube and are not standard
 * SMPTE timecodes. They show hours, minutes, seconds, and then
 * milliseconds separated by a decimal.
 *
 * Example of timestamps that will be created:
 *  Long video: H:MM:SS.MS
 *  Short video (>10 mins): MM:SS.MS
 *  Short video (<10 mins): M:SS.MS
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
    // Drop another leading zero because this video is under 10 minutes
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
export const secondify = (timestamp: string): number => {
  let leadingZero: string = ""

  if (timestamp.length === 6) {
    leadingZero = "0:0"
  } else if (timestamp.length === 7) {
    leadingZero = "0:"
  }

  return new Date("1970-01-01T0" + leadingZero + timestamp + "Z").getTime() / 1000
}

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
