import type { APIRoute } from "astro";
import { getCollection } from "astro:content";

type SlugEntry = {
  id: string;
  slug?: string;
};

const postSlug = (post: SlugEntry): string => (post.slug ?? post.id).replace(/\.md$/, "");

const escapeXml = (value: string): string =>
  value
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&apos;");

const formatRssDate = (value: Date): string => value.toUTCString().replace("GMT", "+0000");

export const GET: APIRoute = async (context) => {
  const site = new URL(context.site ?? "https://www.namelint.dev");
  const siteOrigin = site.origin;
  const now = new Date();

  const posts = (await getCollection("posts"))
    .filter((post) => !post.data.draft)
    .map((post) => ({ ...post, normalizedSlug: postSlug(post) }))
    .sort((a, b) => b.normalizedSlug.localeCompare(a.normalizedSlug));

  const itemsXml = posts
    .map((post) => {
      const dateText = post.normalizedSlug.slice(0, 10);
      const [year] = dateText.split("-");
      const link = `${siteOrigin}/blog/${year}/${post.normalizedSlug}.html`;
      const pubDate = formatRssDate(new Date(`${dateText}T00:00:00Z`));

      return `        <item>
            <guid>${escapeXml(link)}</guid>
            <link>${escapeXml(link)}</link>
            <pubDate>${pubDate}</pubDate>
            <title>${escapeXml(post.data.title)}</title>
            <description><![CDATA[${post.body}]]></description>
        </item>`;
    })
    .join("\n");

  const xml = `<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom" xml:base="${siteOrigin}/">
    <script src="https://www.rss.style/js/rss-style.js" xmlns="http://www.w3.org/1999/xhtml"></script>
    <channel>
        <atom:link href="${siteOrigin}/blog/rss.xml" rel="self" type="application/rss+xml" />
        <copyright>Copyright © 2025 by Andrew Marcuse</copyright>
        <description>Namelint Blog</description>
        <docs>https://validator.w3.org/feed/docs/rss2.html</docs>
        <generator>Astro</generator>
        <image>
            <link>${siteOrigin}/blog/</link>
            <title>Namelint Blog</title>
            <url>${siteOrigin}/favicon.png</url>
        </image>
        <language>en</language>
        <lastBuildDate>${formatRssDate(now)}</lastBuildDate>
        <link>${siteOrigin}/blog/</link>
        <managingEditor>fileformat@gmail.com (Andrew Marcuse)</managingEditor>
        <pubDate>${formatRssDate(now)}</pubDate>
        <title>Namelint Blog</title>
        <ttl>1440</ttl>
        <webMaster>fileformat@gmail.com (Andrew Marcuse)</webMaster>
${itemsXml}
    </channel>
</rss>`;

  return new Response(xml, {
    headers: {
      "content-type": "text/xml; charset=utf-8",
      "cache-control": "public, max-age=300"
    }
  });
};
