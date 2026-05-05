import { QuartzConfig } from "./quartz/cfg"
import * as Plugin from "./quartz/plugins"

/**
 * Quartz 4 Configuration
 *
 * See https://quartz.jzhao.xyz/configuration for more information.
 */
const config: QuartzConfig = {
  configuration: {
    pageTitle: "last.fm",
    pageTitleSuffix: "",
    enableSPA: false,
    enablePopovers: false,
    analytics: null,
    locale: "en-US",
    baseUrl: "wngyn.net",
    ignorePatterns: ["private", "templates", ".obsidian"],
    defaultDateType: "modified",
    theme: {
      fontOrigin: "local",
      cdnCaching: false,
      typography: {
        header: "system-ui",
        body: "system-ui",
        code: "ui-monospace",
      },
      colors: {
        lightMode: {
          light: "#fbf7f7",
          lightgray: "#eadede",
          gray: "#c4b1b1",
          darkgray: "#6a5656",
          dark: "#2f2323",
          secondary: "#b53434",
          tertiary: "#d86a6a",
          highlight: "rgba(181, 52, 52, 0.12)",
          textHighlight: "#ffd7d780",
        },
        darkMode: {
          light: "#191314",
          lightgray: "#3a2c2f",
          gray: "#72585d",
          darkgray: "#decfd2",
          dark: "#f5eded",
          secondary: "#d84a4a",
          tertiary: "#f08a8a",
          highlight: "rgba(216, 74, 74, 0.16)",
          textHighlight: "#7a1f2880",
        },
      },
    },
  },
  plugins: {
    transformers: [
      Plugin.FrontMatter(),
      Plugin.ObsidianFlavoredMarkdown({ enableInHtmlEmbed: false }),
      Plugin.GitHubFlavoredMarkdown(),
      Plugin.CrawlLinks({ markdownLinkResolution: "shortest" }),
    ],
    filters: [Plugin.RemoveDrafts()],
    emitters: [
      Plugin.ComponentResources(),
      Plugin.ContentPage(),
      Plugin.FolderPage(),
      Plugin.ContentIndex({
        enableSiteMap: false,
        enableRSS: false,
      }),
      Plugin.Assets(),
      Plugin.Static(),
      Plugin.Favicon(),
      Plugin.NotFoundPage(),
    ],
  },
}

export default config
