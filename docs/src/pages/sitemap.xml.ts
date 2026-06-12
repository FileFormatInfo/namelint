import type { APIRoute } from "astro";
import { getCollection } from "astro:content";

export const GET: APIRoute = async () => {
  const posts = await getCollection("posts");
  const site = "https://www.namelint.dev";

  // Static pages
  const pages = [
    "/",
    "/blog/",
  ];

  // Build sitemap entries
  const urls = [
    // Static pages
    ...pages.map((url) => `\t<url><loc>${site}${url}</loc></url>`),
    // Blog post URLs
    ...posts
      .filter((post) => !post.data.draft)
      .map((post) => {
        const slug = (post.slug ?? post.id).replace(/\.md$/, "");
        const [year] = slug.split("-");
        return `\t<url><loc>${site}/blog/${year}/${slug}.html</loc></url>`;
      }),
  ];

  const xml = `<?xml version="1.0" encoding="UTF-8"?>
<?xml-stylesheet type="text/xsl" href="sitemap.xslt"?>
<urlset
	xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
	xsi:schemaLocation="http://www.sitemaps.org/schemas/sitemap/0.9 http://www.sitemaps.org/schemas/sitemap/0.9/sitemap.xsd"
	xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
${urls.join("\n")}
</urlset>`;

  return new Response(xml, {
    headers: {
      "content-type": "application/xml; charset=utf-8",
      "cache-control": "public, max-age=3600"
    }
  });
};
