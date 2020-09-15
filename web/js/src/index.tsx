import { h, render } from "preact"
import "preact/devtools"

import App from "./components/App"

import "./styles/main.css"

const root = document.getElementById("root") as Element
render(<App />, root, root.firstElementChild as Element)
