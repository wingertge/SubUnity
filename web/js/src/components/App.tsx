import { h } from "preact"
import { useState, useEffect, useContext } from "preact/hooks"

import Header from "./Header"
import Player from "./Player"
import CaptionList from "./CaptionList"

import type { Caption, CaptionData, VideoInfo } from "../types"
import { initialCaptionState, timestampify, NotyfContext } from "../utils"

let TOKEN = `${window.VIDEO_ID}-${window.SUBTITLE_LANG}`

export default function App() {
  let message = useContext(NotyfContext)

  // Editor State
  let [error, setError] = useState<string>("")
  let [loading, setLoading] = useState<boolean>(true)
  let [isEditorDirty, setEditorDirty] = useState<boolean>(false)

  // Video Information State
  let [videoInfo, setVideoInfo] = useState<VideoInfo>({
    videoTitle: "",
    videoId: "",
    language: "",
  })

  // Caption State
  let [captions, setCaptions] = useState<Caption[]>([])
  let [activeCaption, setActiveCaption] = useState<Caption>(initialCaptionState)

  /**
   * Fetch video information and captions from the API
   *
   * @param {string} id
   * @param {string} language
   */
  async function fetchCaptions(id: string, language: string): Promise<void> {
    try {
      let response: Response = await fetch(`/subtitles/${id}?lang=${language}`)
      let data: CaptionData = await response.json()

      let { entries, ...videoData } = data

      /**
       * If no caption entries are returned from the API, populate entries
       * with a dummy caption to help users get started
       */
      if (entries.length === 0) {
        entries.push({ startSeconds: 0, endSeconds: 0, text: "" })
      }

      /**
       * Transform the caption entries and add display related metadata
       */
      let fetchedCaptions: Caption[] = entries.map((caption, id) => ({
        id,
        startTimestamp: timestampify(caption.startSeconds),
        endTimestamp: timestampify(caption.endSeconds),
        manuallySelected: false,
        ...caption,
      }))

      setVideoInfo({ ...videoData })
      document.title = `${videoData.videoTitle} | Subtitle Editor`

      localStorage.setItem(
        `videoInfo-${TOKEN}`,
        JSON.stringify({ ...videoData })
      )

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
      /**
       * Strip out display related state and only send
       * necessary data to the API
       */
      let data: CaptionData = {
        entries: captions.map(({ startSeconds, endSeconds, text }) => ({
          startSeconds,
          endSeconds,
          text,
        })),
        ...videoInfo,
      }

      let saveRequest: Response = await fetch("/subtitles/", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(data),
      })

      /**
       * If the request was successful, notify the user and flag
       * the editor as not holding any unsaved changes
       */
      if (saveRequest.ok) {
        message.success("Changes successfully saved!")

        localStorage.removeItem(`captions-${TOKEN}`)
        setEditorDirty(false)
      }
    } catch (error) {
      message.error("Unable to save changes")
      console.log("Error saving captions", error)
    }
  }

  function syncCaptionStorage(): void {
    if (videoInfo.videoId !== "") {
      localStorage.setItem(`captions-${TOKEN}`, JSON.stringify(captions))
    }
  }

  useEffect(() => {
    let hasDirtyChanges: boolean = JSON.parse(
      localStorage.getItem(`isEditorDirty-${TOKEN}`)
    )

    if (hasDirtyChanges) {
      let hydrateConfirmation: boolean = confirm(
        "Would you like to reload your currently drafted edits?"
      )

      if (hydrateConfirmation) {
        let localCaptionState = localStorage.getItem(`captions-${TOKEN}`)
        let localVideoInfoState = localStorage.getItem(`videoInfo-${TOKEN}`)

        setCaptions(JSON.parse(localCaptionState))
        setVideoInfo(JSON.parse(localVideoInfoState))
        setLoading(false)

        message.success("Local draft restored")
      } else {
        fetchCaptions(window.VIDEO_ID, window.SUBTITLE_LANG)
      }
    } else {
      fetchCaptions(window.VIDEO_ID, window.SUBTITLE_LANG)
    }
  }, [])

  // Whenever dirty editor state changes, persist it to local storage
  useEffect(
    () =>
      localStorage.setItem(
        `isEditorDirty-${TOKEN}`,
        JSON.stringify(isEditorDirty)
      ),
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
          isEditorDirty={isEditorDirty}
          setEditorDirty={setEditorDirty}
          syncCaptionStorage={syncCaptionStorage}
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
