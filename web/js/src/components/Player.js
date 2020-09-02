import YouTubePlayer from "./YouTubePlayer"

import "../styles/player.css"

export default function Player(props) {
  let { videoID, activeCaption, updateActiveCaption } = props
  let hasActiveCaption = Object.entries(activeCaption).length > 1

  return (
    <div class="player">
      <YouTubePlayer
        width="1024px"
        height="576px"
        videoId={videoID}
        onTimeUpdate={currentTime => updateActiveCaption(currentTime)}
      />

      {hasActiveCaption && (
        <div class="active-caption">{activeCaption.text}</div>
      )}
    </div>
  )
}
