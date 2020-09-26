import { h } from "preact"

import type { Caption, CaptionItemCallbacks } from "../types"

interface CaptionProps extends Caption, CaptionItemCallbacks {
  isActive: boolean
}

export default function CaptionItem(props: CaptionProps) {
  let {
    id,
    startTimestamp,
    endTimestamp,
    text,
    isActive,
    addCaption,
    captionSelected,
    updateCaptionField,
    deleteCaption,
  } = props

  return (
    <div
      class={isActive ? "caption caption-highlighted" : "caption"}
      onClick={() => captionSelected(id)}
    >
      <div class="timestamps">
        <input
          type="text"
          class="startTimestamp"
          value={startTimestamp}
          onChange={event =>
            updateCaptionField(id, "startTimestamp", event.currentTarget.value)
          }
        />

        <input
          type="text"
          class="endTimestamp"
          value={endTimestamp}
          onChange={event =>
            updateCaptionField(id, "endTimestamp", event.currentTarget.value)
          }
        />
      </div>

      <textarea
        class="caption-textbox"
        title="Caption text"
        value={text}
        rows={3}
        onChange={event =>
          updateCaptionField(id, "text", event.currentTarget.value)
        }
      />

      <div class="caption-options">
        <button
          class="caption-button delete"
          title="Delete this caption"
          onClick={() => deleteCaption(id)}
        >
          &times;
        </button>
        <button
          class="caption-button"
          title="Add new caption"
          onClick={() => addCaption(id)}
        >
          +
        </button>
      </div>
    </div>
  )
}
