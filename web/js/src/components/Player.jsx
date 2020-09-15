import { h } from "preact"

import YouTubePlayer from "./YouTubePlayer"

import "../styles/player.css"

export default function Player(props) {
  let {
    videoID,
    captions,
    setCaptions,
    activeCaption,
    setActiveCaption,
  } = props

  /**
   * Whenever the player resumes playback, all captions should be
   * reset to not being manually selected.
   */
  function resetCaptions() {
    let allCaptions = captions.map(caption => ({
      ...caption,
      manuallySelected: false,
    }))

    setCaptions(allCaptions)
  }

  /**
   * Checks to see if the active caption has more than one entry
   */
  let hasActiveCaption = Object.entries(activeCaption).length > 1

  /**
   * Find the caption that needs to be displayed, and then set that
   * as the active caption.
   *
   * @param {number} currentTime Seconds since video started
   * @param {boolean} manuallySelected Caption was selected by a user
   */
  function updateActiveCaption(currentTime) {
    let currentCaption = captions.filter(
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
        videoId={videoID}
        activeCaption={activeCaption}
        onPlaying={resetCaptions}
        onTimeUpdate={currentTime => updateActiveCaption(currentTime)}
      />

      {hasActiveCaption && (
        <div class="active-caption">{activeCaption.text}</div>
      )}
    </div>
  )
}
