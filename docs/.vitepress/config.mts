import { defineConfig } from "vitepress";
import { withPwa } from "@vite-pwa/vitepress";

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
  defineConfig({
    title: "conftier",
    description: "Multi-level configuration framework",
    sitemap: {
      hostname: "https://conftier.zeeland.top",
    },
    head: [
      ...getAnalyticsScripts(),
    ],
    themeConfig: {
      // https://vitepress.dev/reference/default-theme-config
      logo: "/logo.png",
      nav: [
        { text: "Home", link: "/" },
        { text: "Guide", link: "/guide/" },
        {
          text: "GitHub",
          link: "https://github.com/Undertone0809/conftier",
        },
      ],
      sidebar: [
        {
          text: "Introduction",
          items: [
            { text: "Getting Started", link: "/guide/" },
            { text: "Installation", link: "/guide/installation" },
          ],
        },
        {
          text: "Other",
          items: [
            { text: "Overview", link: "/api/" },
            { text: "Examples", link: "/api/examples" },
            { text: "Changelog", link: "/other/changelog" },
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
);
