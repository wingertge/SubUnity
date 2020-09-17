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
            updateCaptionField(id, "startTimestamp", event.currentTarget.value)
          }
        />

        <input
          type="text"
          class="endTimestamp"
          value={endTimestamp}
          onFocus={() => captionSelected(id)}
          onChange={event =>
            updateCaptionField(id, "endTimestamp", event.currentTarget.value)
          }
        />
      </div>

      <textarea
        class="caption-textbox"
        name="editable_text"
        value={text}
        rows={3}
        onFocus={() => captionSelected(id)}
        onInput={event =>
          updateCaptionField(id, "text", event.currentTarget.value)
        }
      />

      <div class="caption-options">
        <button class="caption-button" title="New caption above">
          &uarr;
        </button>
        <button
          class="caption-button delete"
          title="Delete caption"
          onClick={() => deleteCaption(id)}
        >
          &times;
        </button>

        <button class="caption-button" title="New caption below">
          &darr;
        </button>
      </div>
    </div>
  )
}
