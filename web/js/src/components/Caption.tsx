import { h } from "preact"

import type { Caption, CaptionItemCallbacks } from "../types"

export interface CaptionProps extends Caption, CaptionItemCallbacks {
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
          onChange={(event: Event) =>
            updateCaptionField(id, "startTimestamp", event.target.value)
          }
        />

        <input
          type="text"
          class="endTimestamp"
          value={endTimestamp}
          onFocus={() => captionSelected(id)}
          onChange={(event: Event) =>
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
        onInput={(event: Event) =>
          updateCaptionField(id, "text", event.target.value)
        }
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
