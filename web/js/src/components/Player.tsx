import { h } from "preact"

import YouTubePlayer from "./YouTubePlayer"
import type { VideoInfo, Caption, CaptionState } from "../types"

interface PlayerProps extends CaptionState, VideoInfo {}

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
    let currentCaption: Caption[] = captions.filter(
      caption =>
        currentTime > caption.startSeconds && currentTime < caption.endSeconds
    )

    // Only update the active caption if there are any
    // matching captions, otherwise this will throw an error.
    if (currentCaption.length !== 0) {
      setActiveCaption(currentCaption[0])
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

      {activeCaption && <div class="active-caption">{activeCaption.text}</div>}
    </div>
  )
}
