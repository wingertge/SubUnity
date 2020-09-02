import Caption from "./Caption"

import "../styles/captions.css"

export default function CaptionList(props) {
  return (
    <div class="captions-list">
      {props.captions.map(caption => (
        <Caption
          key={caption.id}
          updateCaption={props.updateCaption}
          isActive={props.activeCaption.id === caption.id}
          {...caption}
        />
      ))}
    </div>
  )
}
