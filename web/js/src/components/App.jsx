import { h } from "preact"
import { useState, useEffect } from "preact/hooks"

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
   * @param {string} content
   */
  function updateCaption(id, field, content) {
    let payload = [...captions]
    payload[id][field] = content

    setCaptions(payload)
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

  function captionSelected(id, startTime) {
    updateActiveCaption(startTime, true)
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
      let response = await fetch(`/subtitles/${id}?lang=${language}`, {
        method: "GET",
      })
      let results = await response.json()

      let fetchedCaptions = results.entries.map((caption, index) => ({
        id: index,
        ...caption,
        manuallySelected: false,
      }))

      setCaptions(fetchedCaptions)
    } catch (error) {
      setError("Error fetching captions")
      return console.error("Error fetching captions:", error)
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
      <CaptionList
        captions={captions}
        activeCaption={activeCaption}
        updateActiveCaption={updateActiveCaption}
        updateCaption={updateCaption}
        captionSelected={captionSelected}
      />

      <Player
        videoID={videoID}
        captions={captions}
        activeCaption={activeCaption}
        updateActiveCaption={updateActiveCaption}
      />
    </div>
  )
}
