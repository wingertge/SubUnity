import { useState, useEffect } from "preact/hooks"

import Player from "./Player"
import CaptionList from "./CaptionList"

export default function App() {
  let [videoID, setVideoID] = useState("9mSpciMOvHU")
  let [captions, setCaptions] = useState([])
  let [activeCaption, setActiveCaption] = useState({})
  let [currentTime, setCurrentTime] = useState(0)

  /**
   * Parse an SRT caption file into a JSON object, adding some
   * useful extra properties so that it works better with the
   * YouTube player, and then set the `captions` state to
   * the returned object.
   *
   * @param {string} captions
   */
  // function parseCaptions(captions) {
  //   let parsedCaptions = parse(captions).map((caption, index) => ({
  //     id: index,
  //     ...caption,
  //     startInSeconds: Number((caption.start * 0.001).toFixed(3)),
  //     endInSeconds: Number((caption.end * 0.001).toFixed(3)),
  //   }))

  //   setCaptions(parsedCaptions)
  // }

  /**
   * @todo Document this function
   * @param {number} id
   * @param {string} content
   */
  function updateCaption(id, content) {
    let payload = [...captions]
    payload[id]["text"] = content

    setCaptions(payload)
  }

  /**
   * Find the caption that needs to be displayed, and then set that
   * as the active caption.
   *
   * @param {number} currentTime Seconds since video started
   */
  function updateActiveCaption(currentTime) {
    let currentCaption = captions.filter(
      caption =>
        currentTime > caption.startInSeconds &&
        currentTime < caption.endInSeconds
    )

    // Only update the active caption if there are any
    // matching captions, otherwise this will throw an error.
    if (currentCaption.length !== 0) {
      setActiveCaption(currentCaption[0])
    }
  }

  /**
   * Fetch an arbitrary URL and attempt to parse it, if there's
   * any error, log it to the console.
   *
   * @todo Better error handling if this step fails
   * @todo Allow users to import SRT files from their desktop
   *
   * @param {string} url
   */
  async function fetchCaptions(url) {
    try {
      const response = await fetch(url, { method: "GET" })
      const text = await response.json()

      setCaptions(text)
    } catch (error) {
      return console.error("Error fetching captions:", error)
    }
  }

  useEffect(
    () => fetchCaptions(`http://localhost:8000/subtitles/${videoID}?lang=en`),
    []
  )

  return (
    <div class="app">
      <CaptionList
        captions={captions}
        activeCaption={activeCaption}
        updateActiveCaption={updateActiveCaption}
        updateCaption={updateCaption}
        currentTime={currentTime}
      />

      <Player
        videoID={videoID}
        captions={captions}
        activeCaption={activeCaption}
        updateActiveCaption={updateActiveCaption}
        currentTime={currentTime}
      />
    </div>
  )
}
