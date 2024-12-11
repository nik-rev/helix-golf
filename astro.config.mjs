import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";
import { rehypeHeadingIds } from "@astrojs/markdown-remark";
import rehypeAutolinkHeadings from "rehype-autolink-headings";
import rehypeExternalLinks from "rehype-external-links";

// https://astro.build/config
export default defineConfig({
  site: "https://nikitarevenco.github.io",
  base: "/helix-golf",
  markdown: {
    rehypePlugins: [
      rehypeHeadingIds,

      [
        rehypeExternalLinks,
        {
          content: {
            type: "text",
            value: " ↗",
          },
          properties: {
            target: "_blank",
          },
          rel: ["noopener"],
        },
      ],
      [rehypeAutolinkHeadings, { behavior: "wrap" }],
    ],
  },
  integrations: [
    starlight({
      head: [
        {
          tag: "meta",
          attrs: {
            property: "og:image",
            content: "/helix-golf/og.jpg",
          },
        },
      ],
      title: "Helix Golf ⛳",
      social: {
        github: "https://github.com/nikitarevenco/helix-golf",
      },
      components: {
        ThemeProvider: "./src/ThemeProvider.astro",
        ThemeSelect: "./src/ThemeSelect.astro",
      },
      sidebar: [
        {
          label: "Helix Golf",
          autogenerate: { directory: "helix-golf" },
        },
      ],
      customCss: ["./src/globals.css"],
    }),
  ],
});
