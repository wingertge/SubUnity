import { h } from "preact"
import Caption from "./Caption"

import "../styles/captions.css"

export default function CaptionList(props) {
  return (
    <div class="captions-list">
      {props.captions.map(caption => (
        <Caption
          key={caption.id}
          updateCaptionField={props.updateCaptionField}
          captionSelected={props.captionSelected}
          isActive={props.activeCaption.id === caption.id}
          deleteCaption={props.deleteCaption}
          {...caption}
        />
      ))}
    </div>
  )
}
