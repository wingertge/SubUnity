import { h } from "preact"

export default function Caption(props) {
  function secondify(timestamp) {
    return new Date("1970-01-01T00:" + timestamp + "Z").getTime() / 1000
  }

  return (
    <div class={props.isActive ? "caption caption-highlighted" : "caption"}>
      <div class="timestamps">
        <input
          type="text"
          class="startTimestamp"
          value={props.startTimestamp}
          onFocus={event => props.captionSelected(props.id)}
          onInput={event => {
            props.updateCaptionField(
              props.id,
              "startTimestamp",
              event.target.value
            )
            props.updateCaptionField(
              props.id,
              "startSeconds",
              secondify(event.target.value)
            )
          }}
        />

        <input
          type="text"
          class="endTimestamp"
          value={props.endTimestamp}
          onFocus={event => props.captionSelected(props.id)}
          onInput={event => {
            props.updateCaptionField(
              props.id,
              "endTimestamp",
              event.target.value
            )
            props.updateCaptionField(
              props.id,
              "endSeconds",
              secondify(event.target.value)
            )
          }}
        />
      </div>

      <textarea
        class="caption-textbox"
        name="editable_text"
        value={props.text}
        rows="3"
        onFocus={event => props.captionSelected(props.id)}
        onInput={event =>
          props.updateCaptionField(props.id, "text", event.target.value)
        }
      />

      <div
        class="caption-delete"
        role="button"
        onClick={event => props.deleteCaption(props.id)}
      >
        &times;
      </div>
    </div>
  )
}
