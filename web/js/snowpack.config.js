/** @type { import("snowpack").SnowpackUserConfig } */
module.exports = {
  mount: {
    public: "/",
    src: "/_dist_",
  },
  plugins: [
    "@snowpack/plugin-dotenv",
    "@snowpack/plugin-typescript",
    "@prefresh/snowpack",
  ],
  installOptions: {
    installTypes: true,
    polyfillNode: true
  },
  devOptions: {
    "open": "none"
  }
}
