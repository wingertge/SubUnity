import { h } from "preact"
import { nanoid } from "nanoid"

import CaptionItem from "./Caption"
import { initialCaptionState, secondify } from "../utils"

import type { Ref } from "preact/hooks"
import type {
  Caption,
  CaptionState,
  EditorState,
  EditableCaptionField,
  VideoInfo,
} from "../types"

import "../styles/captions.css"

interface CaptionListProps
  extends CaptionState,
    EditorState,
    Pick<VideoInfo, "isVideoLong"> {
  playerRef: Ref<HTMLDivElement>
}

export default function CaptionList(props: CaptionListProps) {
  let {
    playerRef,
    captions,
    setCaptions,
    isVideoLong,
    activeCaption,
    setActiveCaption,
    isEditorDirty,
    setEditorDirty,
    syncCaptionStorage,
  } = props

  /**
   * Add a new caption below an already existing caption
   *
   * @param {number} id
   */
  function addCaption(id: string): void {
    let captionsCopy: Caption[] = [...captions]
    let captionIndex: number = captionsCopy.findIndex(c => c.id === id)

    let newCaption: Caption = {
      ...initialCaptionState,
      id: nanoid(),
    }

    captionsCopy.splice(captionIndex + 1, 0, newCaption)
    setCaptions(captionsCopy)
  }

  /**
   * Update a specific caption field
   *
   * @param {number} id
   * @param {string} field
   * @param {string} content
   */
  function updateCaptionField(
    id: string,
    field: EditableCaptionField,
    content: string
  ): void {
    let payload: Caption[] = captions.map(caption => {
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

    // Persist changes to local storage and flag the editor as being dirty
    syncCaptionStorage()
    setEditorDirty(true)
  }

  /**
   * Delete a specific caption from state
   *
   * @param {number} id
   */
  function deleteCaption(id: string): void {
    let confirmation = confirm("Are you sure that you want to delete this?")

    if (confirmation) {
      if (activeCaption.id === id) {
        setActiveCaption(initialCaptionState)
      }

      let deletedCaptions: Caption[] = captions.filter(c => c.id !== id)
      setCaptions(deletedCaptions)
    }
  }

  /**
   * Find the caption by specific ID and seek the player
   * to the caption's start time
   *
   * @param {string} id
   */
  function captionSelected(id: string): void {
    if (activeCaption.id !== id) {
      let selectedCaption: Caption[] = captions.filter(c => c.id == id)

      if (playerRef.current) {
        let player = playerRef.current.player

        player.seek(selectedCaption[0].startSeconds)
        player.pause()
      }

      setActiveCaption(selectedCaption[0])
    }
  }

  return (
    <div class={`captions-list ${isVideoLong ? "long" : "short"}`}>
      {props.captions.map(caption => (
        <CaptionItem
          key={caption.id}
          updateCaptionField={updateCaptionField}
          captionSelected={captionSelected}
          addCaption={addCaption}
          deleteCaption={deleteCaption}
          isActive={activeCaption.id === caption.id}
          {...caption}
        />
      ))}
    </div>
  )
}
