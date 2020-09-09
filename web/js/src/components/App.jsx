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
        startTimestamp: new Date(1000 * caption.startSeconds)
          .toISOString()
          .substring(14, 21),
        endTimestamp: new Date(1000 * caption.endSeconds)
          .toISOString()
          .substring(14, 21),
        manuallySelected: false,
        ...caption,
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
        entries: captions.map(({ startSeconds, endSeconds, text }) => ({
          startSeconds,
          endSeconds,
          text,
        })),
        videoId: videoID,
        language: "en",
      }
      let response = await fetch("/subtitles/", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(payload),
      })
    } catch (error) {
      console.log("Error saving captions", error)
    }
  }

  useEffect(() => {
    setVideoID(window.VIDEO_ID)
    fetchCaptions(window.VIDEO_ID, "en")
  }, [])

  if (error) {
    return <div class="error">{error}</div>
  }

  return (
    <div class="app">
      <Header saveCaptions={saveCaptions} />

      <div class="editor">
        <CaptionList
          captions={captions}
          activeCaption={activeCaption}
          setCaptions={setCaptions}
        />

        <Player
          videoID={videoID}
          captions={captions}
          setCaptions={setCaptions}
          activeCaption={activeCaption}
          setActiveCaption={setActiveCaption}
        />
      </div>
    </div>
  )
}
