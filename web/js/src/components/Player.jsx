import { h } from "preact"

import Header from "./Header"
import YouTubePlayer from "./YouTubePlayer"

import "../styles/player.css"

export default function Player(props) {
  let { videoID, activeCaption, resetCaptions, updateActiveCaption } = props
  let hasActiveCaption = Object.entries(activeCaption).length > 1

  return (
    <div class="player">
      <Header />

      <YouTubePlayer
        width="1024px"
        height="576px"
        videoId={videoID}
        activeCaption={activeCaption}
        onPlaying={resetCaptions}
        onTimeUpdate={currentTime => updateActiveCaption(currentTime, false)}
      />

      {hasActiveCaption && (
        <div class="active-caption">{activeCaption.text}</div>
      )}
    </div>
  )
}
