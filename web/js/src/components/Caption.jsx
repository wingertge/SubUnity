import { h } from "preact"

export default function Caption(props) {
  return (
    <div class={props.isActive ? "caption caption-highlighted" : "caption"}>
      <div class="timestamps">
        <input
          type="text"
          class="startTimestamp"
          value={props.startTimestamp}
          onFocus={event => props.captionSelected(props.id)}
          onInput={event =>
            props.updateCaption(props.id, "startTimestamp", event.target.value)
          }
        />

        <input
          type="text"
          class="endTimeStamp"
          value={props.endTimeStamp}
          onFocus={event => props.captionSelected(props.id)}
          onInput={event =>
            props.updateCaption(props.id, "startTimestamp", event.target.value)
          }
        />
      </div>

      <textarea
        class="caption-textbox"
        name="editable_text"
        value={props.text}
        rows="3"
        onFocus={event => props.captionSelected(props.id)}
        onInput={event =>
          props.updateCaption(props.id, "text", event.target.value)
        }
      />
    </div>
  )
}
