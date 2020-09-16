import { h } from "preact"

export default function Header(props) {
  return (
    <div class="header">
      <div class="heading">
        <a href="/" class="back-arrow">
          &larr;
        </a>
        <span class="video-title">{props.videoTitle}</span>
      </div>
      <div class="actions">
        <button onClick={() => props.saveCaptions()}>Save</button>
        <button>Import</button>
        <button style={{ marginRight: 0 }}>Export</button>
      </div>
    </div>
  )
}