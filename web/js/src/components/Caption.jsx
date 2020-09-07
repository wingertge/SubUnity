import { h } from "preact"

export default function Caption(props) {
  let {
    id,
    startSeconds,
    endSeconds,
    text,
    captionSelected,
    updateCaption,
    isActive,
  } = props

  return (
    <div class={isActive ? "caption caption-highlighted" : "caption"}>
      <div class="timestamps">
        <input
          type="text"
          class="startTimestamp"
          value={startSeconds}
          onInput={event =>
            updateCaption(id, "startSeconds", event.target.value)
          }
        />

        <input
          type="text"
          class="endTimeStamp"
          value={endSeconds}
          onInput={event => updateCaption(id, "endSeconds", event.target.value)}
        />
      </div>

      <textarea
        class="caption-textbox"
        name="editable_text"
        value={text}
        rows="3"
        onInput={event => updateCaption(id, "text", event.target.value)}
        onFocus={event => captionSelected(id, startSeconds)}
      />
    </div>
  )
}
