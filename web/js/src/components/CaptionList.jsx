import { h } from "preact"
import Caption from "./Caption"

import { secondify } from "../utils"

import "../styles/captions.css"

export default function CaptionList(props) {
  let { captions, setCaptions, activeCaption, setActiveCaption } = props

  /**
   * Update a specific caption field
   *
   * @param {number} id
   * @param {string} field
   * @param {string} content
   */
  function updateCaptionField(id, field, content) {
    let payload = captions.map(caption => {
      if (caption.id === id) {
        let updatedCaption = {
          ...caption,
          [field]: content,
        }

        // Convert human readable timestamps back to machine friendly seconds
        switch (field) {
          case "startTimestamp":
            updatedCaption["startSeconds"] = secondify(content)
            break
          case "endTimestamp":
            updatedCaption["endSeconds"] = secondify(content)
            break
        }

        // Update the currently active caption while it's being edited
        if (activeCaption.id === id) {
          setActiveCaption(updatedCaption)
        }

        return updatedCaption
      }

      return caption
    })

    setCaptions(payload)
  }

  function deleteCaption(id) {
    let confirmation = confirm("Are you sure that you want to delete this?")

    if (confirmation) {
      if (activeCaption.id === id) {
        setActiveCaption({})
      }

      let deletedCaptions = captions.filter(caption => caption.id !== id)
      setCaptions(deletedCaptions)
    }
  }

  /**
   * Find the caption by specific ID and mark it as being
   * selected by a user instead of being selected from playback
   *
   * @param {number} id
   */
  function captionSelected(id) {
    let selectedCaption = captions.filter(caption => caption.id == id)
    selectedCaption[0].manuallySelected = true

    setActiveCaption(selectedCaption[0])
  }

  return (
    <div class="captions-list">
      {props.captions.map(caption => (
        <Caption
          key={caption.id}
          updateCaptionField={updateCaptionField}
          captionSelected={captionSelected}
          deleteCaption={deleteCaption}
          isActive={activeCaption.id === caption.id}
          {...caption}
        />
      ))}
    </div>
  )
}
