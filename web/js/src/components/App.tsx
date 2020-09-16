import { h } from "preact"
import { useState, useEffect } from "preact/hooks"

import Header from "./Header"
import Player from "./Player"
import CaptionList from "./CaptionList"

import type { Caption, CaptionData, VideoInfo } from "../types"
import { initialCaptionState } from "../utils"

export default function App() {
  let [videoInfo, setVideoInfo] = useState<VideoInfo>({
    videoTitle: "",
    videoId: "",
    language: "",
  })
  let [error, setError] = useState<string>("")
  let [captions, setCaptions] = useState<Caption[]>([])
  let [activeCaption, setActiveCaption] = useState<Caption>(initialCaptionState)

  /**
   * Fetch video information and captions from the API
   *
   * @todo Better error handling if this step fails
   * @todo Allow users to import SRT files from their desktop
   *
   * @param {number} id
   * @param {string} lang
   */
  async function fetchCaptions(id: number, lang: string): Promise<void> {
    try {
      let response: Response = await fetch(`/subtitles/${id}?lang=${lang}`)
      let data: CaptionData = await response.json()

      let { videoId, language, videoTitle, uploaderId, uploaderName } = data

      let fetchedCaptions: Caption[] = data.entries.map((caption, id) => ({
        id,
        startTimestamp: new Date(1000 * caption.startSeconds)
          .toISOString()
          .substring(14, 21),
        endTimestamp: new Date(1000 * caption.endSeconds)
          .toISOString()
          .substring(14, 21),
        manuallySelected: false,
        ...caption,
      }))

      setVideoInfo({ videoId, videoTitle, language, uploaderId, uploaderName })
      setCaptions(fetchedCaptions)
    } catch (error) {
      setError("Error fetching captions")
      return console.error("Error fetching captions:", error)
    }
  }

  /**
   * Save changes to the API
   */
  async function saveCaptions(): Promise<void> {
    try {
      let data: CaptionData = {
        entries: captions.map(({ startSeconds, endSeconds, text }) => ({
          startSeconds,
          endSeconds,
          text,
        })),
        videoId: videoInfo.videoId,
        language: videoInfo.language,
      }

      let response = await fetch("/subtitles/", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(data),
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
      <Header videoTitle={videoInfo.videoTitle} saveCaptions={saveCaptions} />

      <div class="editor">
        <CaptionList
          captions={captions}
          setCaptions={setCaptions}
          activeCaption={activeCaption}
          setActiveCaption={setActiveCaption}
        />

        <Player
          videoId={videoInfo.videoId}
          captions={captions}
          setCaptions={setCaptions}
          activeCaption={activeCaption}
          setActiveCaption={setActiveCaption}
        />
      </div>
    </div>
  )
}
