import { h } from "preact"

export default function Header(props) {
  return (
    <div class="header">
      <div class="heading">
        <a href="/" class="back-arrow">
          &larr;
        </a>
        <span class="video-title">Video Title</span>
      </div>
      <div class="actions">
        <button onClick={event => props.saveCaptions()}>Save</button>
        <button>Import</button>
        <button style={{ marginRight: 0 }}>Export</button>
      </div>
    </div>
  )
}
