import { h } from "preact"

export default function Caption(props) {
  let { id, startSeconds, endSeconds, text, updateCaption, isActive } = props

  return (
    <div class={isActive ? "caption caption-highlighted" : "caption"}>
      <div class="timestamps">
        <input type="text" class="startTimestamp" value={startSeconds} />
        <input type="text" class="endTimeStamp" value={endSeconds} />
      </div>

      <textarea
        class="caption-textbox"
        name="editable_text"
        value={text}
        rows="3"
        onInput={event => updateCaption(id, event.target.value)}
      />
    </div>
  )
}
