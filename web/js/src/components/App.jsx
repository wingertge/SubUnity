import { h } from "preact"
import { useState, useEffect } from "preact/hooks"

import Header from "./Header"
import Player from "./Player"
import CaptionList from "./CaptionList"

export default function App() {
  let [videoInfo, setVideoInfo] = useState({})
  let [error, setError] = useState("")
  let [captions, setCaptions] = useState([])
  let [activeCaption, setActiveCaption] = useState({})

  /**
   * Fetch video information and captions from the API
   *
   * @todo Better error handling if this step fails
   * @todo Allow users to import SRT files from their desktop
   *
   * @param {string} id
   * @param {string} language
   */
  async function fetchCaptions(id, lang) {
    try {
      let response = await fetch(`/subtitles/${id}?lang=${lang}`)
      let data = await response.json()

      let { entries, videoId, language, videoTitle } = data

      let fetchedCaptions = entries.map((caption, index) => ({
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

      setVideoInfo({ id: videoId, title: videoTitle, language })
      setCaptions(fetchedCaptions)
    } catch (error) {
      setError("Error fetching captions")
      return console.error("Error fetching captions:", error)
    }
  }

  /**
   * Propagate changes to the API
   */
  async function saveCaptions() {
    try {
      let payload = {
        entries: captions.map(({ startSeconds, endSeconds, text }) => ({
          startSeconds,
          endSeconds,
          text,
        })),
        videoId: videoInfo.id,
        language: videoInfo.language,
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
    fetchCaptions(window.VIDEO_ID, window.SUBTITLE_LANG)
  }, [])

  if (error) {
    return <div class="error">{error}</div>
  }

  return (
    <div class="app">
      <Header videoTitle={videoInfo.title} saveCaptions={saveCaptions} />

      <div class="editor">
        <CaptionList
          captions={captions}
          setCaptions={setCaptions}
          activeCaption={activeCaption}
          setActiveCaption={setActiveCaption}
        />

        <Player
          videoID={videoInfo.id}
          captions={captions}
          setCaptions={setCaptions}
          activeCaption={activeCaption}
          setActiveCaption={setActiveCaption}
        />
      </div>
    </div>
  )
}
