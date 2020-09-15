import { h } from "preact"
import CaptionItem from "./Caption"

import { secondify } from "../utils"
import type { Caption, CaptionState, EditableCaptionField } from "../types"

import "../styles/captions.css"

interface CaptionListProps extends CaptionState {}

export default function CaptionList(props: CaptionListProps) {
  let { captions, setCaptions, activeCaption, setActiveCaption } = props

  /**
   * Update a specific caption field
   *
   * @param {number} id
   * @param {string} field
   * @param {string} content
   */
  function updateCaptionField(
    id: number,
    field: EditableCaptionField,
    content: string
  ): void {
    let payload = captions.map(caption => {
      if (caption.id === id) {
        let updatedCaption: Caption = {
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

  function deleteCaption(id: number): void {
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
  function captionSelected(id: number): void {
    let selectedCaption = captions.filter(caption => caption.id == id)[0]
    selectedCaption.manuallySelected = true

    setActiveCaption(selectedCaption)
  }

  return (
    <div class="captions-list">
      {props.captions.map(caption => (
        <CaptionItem
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
