import { defineConfig } from "vitepress";
import { withPwa } from "@vite-pwa/vitepress";
import { withMermaid } from "vitepress-plugin-mermaid";

const getAnalyticsScripts = () => {
  if (process.env.NODE_ENV === "development") {
    return [];
  }

  return [
    [
      "script",
      {
        defer: "true",
        "data-website-id": "08c68aa8-5e78-42a3-986a-3a44cb686eb8",
        src: "https://umami.zeeland.top/script.js",
      },
    ] as [string, Record<string, string>],
    [
      "script",
      {
        async: "true",
        src: "https://www.googletagmanager.com/gtag/js?id=G-PMSGQDE44B",
      },
    ] as [string, Record<string, string>],
    [
      "script",
      {},
      `window.dataLayer = window.dataLayer || [];
      function gtag(){dataLayer.push(arguments);}
      gtag('js', new Date());
      gtag('config', 'G-PMSGQDE44B');`,
    ] as [string, Record<string, string>, string],
  ];
};

// https://vitepress.dev/reference/site-config
export default withPwa(
  withMermaid(
    defineConfig({
      title: "conftier",
      description: "Multi-level configuration framework",
      sitemap: {
        hostname: "https://conftier.zeeland.top",
      },
      head: [
        ...getAnalyticsScripts(),
        ["link", { rel: "icon", href: "/logo.ico" }],
        [
          "meta",
          {
            property: "description",
            content:
              "Conftier is a powerful multi-level configuration framework.",
          },
        ],
        [
          "meta",
          {
            property: "keywords",
            content: "conftier, config, multi-leve config",
          },
        ],
        [
          "meta",
          {
            property: "og:site_name",
            content:
              "conftier - A powerful multi-level configuration framework",
          },
        ],
        [
          "meta",
          { property: "og:url", content: "https://conftier.zeeland.top" },
        ],
        [
          "meta",
          {
            property: "og:title",
            content:
              "conftier - A powerful multi-level configuration framework",
          },
        ],
        [
          "meta",
          {
            property: "og:description",
            content:
              "Conftier is a powerful multi-level configuration framework.",
          },
        ],
        [
          "meta",
          {
            property: "og:image",
            content:
              "https://r2.zeeland.top/images/2025/04/8ec6af780e829e7c32245713440122a9.png",
          },
        ],
        ["meta", { property: "twitter:card", content: "summary_large_image" }],
        [
          "meta",
          {
            property: "twitter:image",
            content:
              "https://r2.zeeland.top/images/2025/04/8ec6af780e829e7c32245713440122a9.png",
          },
        ],
        [
          "meta",
          {
            property: "twitter:title",
            content:
              "conftier - A powerful multi-level configuration framework",
          },
        ],
        [
          "meta",
          {
            property: "twitter:description",
            content:
              "Conftier is a powerful multi-level configuration framework.",
          },
        ],
      ],
      themeConfig: {
        // https://vitepress.dev/reference/default-theme-config
        logo: "/logo.png",
        nav: [
          { text: "Home", link: "/" },
          { text: "Get Started", link: "/guide/introduction" },
          {
            text: "GitHub",
            link: "https://github.com/Undertone0809/conftier",
          },
        ],
        sidebar: [
          {
            text: "Get Started",
            items: [
              { text: "Introduction", link: "/guide/introduction" },
              { text: "Quick Start", link: "/guide/quick-start" },
            ],
          },
          {
            text: "Other",
            items: [
              { text: "Changelog", link: "/other/changelog" },
              { text: "Contributing", link: "/other/contributing" },
            ],
          },
        ],
        socialLinks: [
          {
            icon: "github",
            link: "https://github.com/Undertone0809/conftier",
          },
        ],
        footer: {
          message: "Released under the MIT License.",
          copyright: "Copyright © 2025 conftier",
        },
      },
      pwa: {
        manifest: {
          name: "conftier",
          short_name: "conftier",
          theme_color: "#2b2a27",
          background_color: "#ffffff",
          display: "standalone",
          orientation: "portrait",
          scope: "/",
          start_url: "/",
          icons: [
            {
              src: "/logo.png",
              sizes: "192x192",
              type: "image/png",
              purpose: "maskable any",
            },
          ],
        },
      },
    })
  )
);
