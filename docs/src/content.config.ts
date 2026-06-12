import { defineCollection, z } from "astro:content";
import { glob } from "astro/loaders";

const posts = defineCollection({
  loader: glob({ pattern: "**/*.md", base: "./src/content/posts" }),
  schema: z.object({
    title: z.string(),
    h1: z.string().optional(),
    draft: z.boolean().optional(),
    tags: z.array(z.string()).optional(),
    redirect_from: z.array(z.string()).optional(),
    cases: z
      .array(
        z.object({
          name: z.string(),
          example: z.string(),
          description: z.string()
        })
      )
      .optional()
  })
});

export const collections = { posts };
