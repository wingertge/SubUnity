import { h } from "preact"
import { useState, useEffect } from "preact/hooks"

import Header from "./Header"
import Player from "./Player"
import CaptionList from "./CaptionList"

import type { Caption, CaptionData, VideoInfo } from "../types"
import { initialCaptionState } from "../utils"

export default function App() {
  let [error, setError] = useState<string>("")
  let [loading, setLoading] = useState<boolean>(true)
  let [message, setMessage] = useState<string>("")

  let [isEditorDirty, setEditorDirty] = useState<boolean>(false)

  let [videoInfo, setVideoInfo] = useState<VideoInfo>({
    videoTitle: "",
    videoId: "",
    language: "",
  })

  let [captions, setCaptions] = useState<Caption[]>([])
  let [activeCaption, setActiveCaption] = useState<Caption>(initialCaptionState)

  /**
   * Fetch video information and captions from the API
   *
   * @todo Better error handling if this step fails
   * @todo Allow users to import SRT files from their desktop
   *
   * @param {string} id
   * @param {string} lang
   */
  async function fetchCaptions(id: string, lang: string): Promise<void> {
    try {
      let response: Response = await fetch(`/subtitles/${id}?lang=${lang}`)
      let data: CaptionData = await response.json()

      let { videoId, language, videoTitle, uploaderId, uploaderName } = data

      // If no caption entries returned from the API, populate with
      // a dummy caption to help users get started
      if (data.entries.length === 0) {
        data.entries = [
          { startSeconds: 0, endSeconds: 0, text: "" },
          ...data.entries,
        ]
      }

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

      setLoading(false)
    } catch (error) {
      setLoading(false)
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
        ...videoInfo,
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
    let hasDirtyChanges: boolean = JSON.parse(
      localStorage.getItem("isEditorDirty")
    )

    if (hasDirtyChanges) {
      let hydrateConfirmation: boolean = confirm(
        "Would you like to reload your currently drafted edits?"
      )

      if (hydrateConfirmation) {
        let localCaptionState = localStorage.getItem("captions")
        setCaptions(JSON.parse(localCaptionState))
        setLoading(false)
      } else {
        fetchCaptions(window.VIDEO_ID, window.SUBTITLE_LANG)
      }
    } else {
      fetchCaptions(window.VIDEO_ID, window.SUBTITLE_LANG)
    }
  }, [])

  // Whenever dirty editor state changes, persist it to local storage
  useEffect(
    () => localStorage.setItem("isEditorDirty", String(isEditorDirty)),
    [isEditorDirty]
  )

  return (
    <div class="app">
      <Header videoTitle={videoInfo.videoTitle} saveCaptions={saveCaptions} />

      {error && <div class="message error">{error}</div>}
      {loading && <div class="message">Loading</div>}

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
