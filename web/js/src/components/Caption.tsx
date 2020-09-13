import { h } from "preact"

interface CaptionProps {
  id: number
  startTimestamp: string
  endTimestamp: string
  text: string
  isActive: boolean
  captionSelected(id: number): void
  updateCaptionField(id: number, field: string, content: string): void
  deleteCaption(id: number): void
}

export default function Caption(props: CaptionProps) {
  let {
    id,
    startTimestamp,
    endTimestamp,
    text,
    isActive,
    captionSelected,
    updateCaptionField,
    deleteCaption,
  } = props

  return (
    <div class={isActive ? "caption caption-highlighted" : "caption"}>
      <div class="timestamps">
        <input
          type="text"
          class="startTimestamp"
          value={startTimestamp}
          onFocus={() => captionSelected(id)}
          onChange={event =>
            updateCaptionField(id, "startTimestamp", event.target.value)
          }
        />

        <input
          type="text"
          class="endTimestamp"
          value={endTimestamp}
          onFocus={() => captionSelected(id)}
          onChange={event =>
            updateCaptionField(id, "endTimestamp", event.target.value)
          }
        />
      </div>

      <textarea
        class="caption-textbox"
        name="editable_text"
        value={text}
        rows={3}
        onFocus={() => captionSelected(id)}
        onInput={event => updateCaptionField(id, "text", event.target.value)}
      />

      <div
        class="caption-delete"
        role="button"
        onClick={() => deleteCaption(id)}
      >
        &times;
      </div>
    </div>
  )
}
