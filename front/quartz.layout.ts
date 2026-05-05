import { PageLayout, SharedLayout } from "./quartz/cfg"
import * as Component from "./quartz/components"

// components shared across all pages
export const sharedPageComponents: SharedLayout = {
  head: Component.Head(),
  header: [],
  afterBody: [],
  footer: Component.Footer({
    links: {
      "main website": "https://wngyn.net",
    },
  }),
}

// components for pages that display a single page (e.g. a single note)
export const defaultContentPageLayout: PageLayout = {
  beforeBody: [
    Component.Graph({
      localGraph: {
        depth: -1,
        scale: 1.1,
        linkDistance: 80,
        fontSize: 0.9,
        focusOnHover: true,
        enableRadial: true,
      },
      globalGraph: {
        scale: 1.1,
        linkDistance: 80,
        fontSize: 1,
      },
    }),
    Component.ConditionalRender({
      component: Component.Breadcrumbs(),
      condition: (page) => page.fileData.slug !== "index",
    }),
    Component.ConditionalRender({
      component: Component.ArticleTitle(),
      condition: (page) => page.fileData.slug !== "index",
    }),
  ],
  left: [
    Component.ConditionalRender({
      component: Component.PageTitle(),
      condition: (page) => page.fileData.slug !== "index",
    }),
    Component.ConditionalRender({
      component: Component.MobileOnly(Component.Spacer()),
      condition: (page) => page.fileData.slug !== "index",
    }),
    Component.Flex({
      components: [{ Component: Component.Darkmode() }, { Component: Component.ReaderMode() }],
    }),
    Component.Explorer(),
  ],
  right: [],
}

// components for pages that display lists of pages  (e.g. tags or folders)
export const defaultListPageLayout: PageLayout = {
  beforeBody: [
    Component.Graph({
      localGraph: {
        depth: -1,
        scale: 1.1,
        linkDistance: 80,
        fontSize: 0.9,
        focusOnHover: true,
        enableRadial: true,
      },
      globalGraph: {
        scale: 1.1,
        linkDistance: 80,
        fontSize: 1,
      },
    }),
    Component.Breadcrumbs(),
    Component.ArticleTitle(),
  ],
  left: [
    Component.PageTitle(),
    Component.MobileOnly(Component.Spacer()),
    Component.Flex({
      components: [{ Component: Component.Darkmode() }],
    }),
    Component.Explorer(),
  ],
  right: [],
}
