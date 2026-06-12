import type { APIRoute } from "astro";

export const GET: APIRoute = async () => {
  const target = "/blog/";
  const html = `<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta http-equiv="refresh" content="0; url=${target}" />
    <link rel="canonical" href="${target}" />
    <title>Redirecting...</title>
  </head>
  <body>
    <p><a href="${target}">Redirecting to ${target}</a></p>
  </body>
</html>`;

  return new Response(html, {
    headers: {
      "content-type": "text/html; charset=utf-8",
      "cache-control": "public, max-age=3600"
    }
  });
};
