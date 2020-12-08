import { h } from "preact"
import { useState } from "preact/hooks"

// @ts-ignore
import YouTubePlayer from "./YouTubePlayer"

import type { Ref } from "preact/hooks"
import type { VideoInfo, Caption, CaptionState } from "../types"

interface PlayerProps extends CaptionState, Pick<VideoInfo, "videoId"> {
  playerRef: Ref<HTMLDivElement>
}

import "../styles/player.css"

export default function Player(props: PlayerProps) {
  let {
    playerRef,
    videoId,
    captions,
    activeCaption,
    setActiveCaption,
  } = props

  /**
   * Check to see if activeCaption is not empty
   */
  const hasActiveCaption: boolean = activeCaption.text.length > 0

  /**
   * Find the caption that needs to be displayed, and then set that
   * as the active caption.
   *
   * @param {number} currentTime Seconds since video started
   */
  function updateActiveCaption(currentTime: number): void {
    let currentCaption: Caption[] = captions.filter(
      c => currentTime > c.startSeconds && currentTime < c.endSeconds
    )

    // Only update the active caption if there are any
    // matching captions, otherwise this will throw an error.
    if (currentCaption.length > 0) {
      setActiveCaption(currentCaption[0])
    }
  }

  return (
    <div class="player">
      <YouTubePlayer
        class="player-iframe"
        ref={playerRef}
        videoId={videoId}
        onTimeUpdate={(currentTime: number) => updateActiveCaption(currentTime)}
      />

      {hasActiveCaption && (
        <div class="active-caption">{activeCaption.text}</div>
      )}
    </div>
  )
}
