import {h, render} from "preact"
import App from "./components/app";

render(<App />, document.body)

let root;

const init = () => {
    let App = require('./components/app').default;

    root = render(<App />, document.body, root);
}

init();

if(module.hot) module.hot.accept('./components/app', init);