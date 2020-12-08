import "preact/devtools"
import { h, render } from "preact"

import App from "./components/App"

import "notyf/notyf.min.css"
import "./styles/main.css"

const root = document.getElementById("root")

if (root) {
  render(<App />, root)
}
