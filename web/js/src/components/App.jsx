import { h } from "preact"
import { useState, useEffect } from "preact/hooks"

import Header from "./Header"
import Player from "./Player"
import CaptionList from "./CaptionList"

export default function App() {
  let [videoID, setVideoID] = useState("")
  let [error, setError] = useState("")
  let [captions, setCaptions] = useState([])
  let [activeCaption, setActiveCaption] = useState({})

  /**
   * Update a specific caption field
   *
   * @todo Populate change to API
   * @param {number} id
   * @param {string} field
   * @param {string|number} content
   */
  function updateCaptionField(id, field, content) {
    let payload = [...captions]
    payload[id][field] = content

    setCaptions(payload)
  }

  /**
   * Whenever the player resumes playback, all captions should be
   * reset to not being manually selected.
   */
  function resetCaptions() {
    let allCaptions = captions.map(caption => ({
      ...caption,
      manuallySelected: false,
    }))

    setCaptions(allCaptions)
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
   * Find the caption that needs to be displayed, and then set that
   * as the active caption.
   *
   * @param {number} currentTime Seconds since video started
   * @param {boolean} manuallySelected Caption was selected by a user
   */
  function updateActiveCaption(currentTime, manuallySelected) {
    let currentCaption = captions.filter(
      caption =>
        currentTime > caption.startSeconds && currentTime < caption.endSeconds
    )

    if (manuallySelected) {
      currentCaption[0].manuallySelected = true
    }

    // Only update the active caption if there are any
    // matching captions, otherwise this will throw an error.
    if (currentCaption.length !== 0) {
      setActiveCaption(currentCaption[0])
    }
  }

  function captionSelected(id) {
    let selectedCaption = captions.filter(caption => caption.id == id)
    selectedCaption[0].manuallySelected = true

    setActiveCaption(selectedCaption[0])
  }

  /**
   * Fetch captions from the API
   *
   * @todo Better error handling if this step fails
   * @todo Allow users to import SRT files from their desktop
   *
   * @param {string} id
   * @param {string} language
   */
  async function fetchCaptions(id, language) {
    try {
      let response = await fetch(`/subtitles/${id}?lang=${language}`)
      let results = await response.json()

      let fetchedCaptions = results.entries.map((caption, index) => ({
        id: index,
        ...caption,
        startTimestamp: new Date(1000 * caption.startSeconds)
          .toISOString()
          .substring(14, 21),
        endTimestamp: new Date(1000 * caption.endSeconds)
          .toISOString()
          .substring(14, 21),
        manuallySelected: false,
      }))

      setCaptions(fetchedCaptions)
    } catch (error) {
      setError("Error fetching captions")
      return console.error("Error fetching captions:", error)
    }
  }

  async function saveCaptions() {
    try {
      let payload = {
        entries: [captions.map(caption => ({ startSeconds }))],
        videoId: videoID,
        language: "en",
      }
      let response = await fetch("/subtitles/", {
        method: "POST",
        body: JSON.stringify(payload),
      })
    } catch (error) {
      console.log("Error saving captions", error)
    }
  }

  useEffect(() => {
    let VIDEO_ID = window.location.pathname.split("/")[2]

    setVideoID(VIDEO_ID)
    fetchCaptions(VIDEO_ID, "en")
  }, [])

  if (error) {
    return <div class="error">{error}</div>
  }

  return (
    <div class="app">
      <Header />

      <div class="editor">
        <CaptionList
          captions={captions}
          activeCaption={activeCaption}
          updateActiveCaption={updateActiveCaption}
          deleteCaption={deleteCaption}
          updateCaptionField={updateCaptionField}
          captionSelected={captionSelected}
        />

        <Player
          videoID={videoID}
          captions={captions}
          activeCaption={activeCaption}
          resetCaptions={resetCaptions}
          updateActiveCaption={updateActiveCaption}
        />
      </div>
    </div>
  )
}
