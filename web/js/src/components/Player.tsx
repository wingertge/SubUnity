import { h } from "preact"

// @ts-ignore
import YouTubePlayer from "./YouTubePlayer"
import type { VideoInfo, Caption, CaptionState } from "../types"

interface PlayerProps extends CaptionState, Pick<VideoInfo, "videoId"> {}

import "../styles/player.css"

export default function Player(props: PlayerProps) {
  let {
    videoId,
    captions,
    setCaptions,
    activeCaption,
    setActiveCaption,
  } = props

  /**
   * Check to see if activeCaption is not empty
   */
  const hasActiveCaption: boolean = activeCaption.text.length > 0

  /**
   * Whenever the player resumes playback, all captions should be
   * reset to not being manually selected.
   */
  function resetCaptions(): void {
    let allCaptions: Caption[] = captions.map(caption => ({
      ...caption,
      manuallySelected: false,
    }))

    setCaptions(allCaptions)
  }

  /**
   * Find the caption that needs to be displayed, and then set that
   * as the active caption.
   *
   * @param {number} currentTime Seconds since video started
   */
  function updateActiveCaption(currentTime: number): void {
    let currentCaption: Caption = captions.filter(
      caption =>
        currentTime > caption.startSeconds && currentTime < caption.endSeconds
    )[0]

    // Only update the active caption if there are any
    // matching captions, otherwise this will throw an error.
    if (currentCaption) {
      setActiveCaption(currentCaption)
    }
  }

  return (
    <div class="player">
      <YouTubePlayer
        width="1024px"
        height="576px"
        videoId={videoId}
        activeCaption={activeCaption}
        onPlaying={resetCaptions}
        onTimeUpdate={(currentTime: number) => updateActiveCaption(currentTime)}
      />

      {hasActiveCaption && (
        <div class="active-caption">{activeCaption.text}</div>
      )}
    </div>
  )
}
