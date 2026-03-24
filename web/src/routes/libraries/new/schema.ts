import { z } from "zod"

export const librarySchema = z.object({
  name: z
    .string()
    .min(1, "Name is required")
    .max(100, "Name is too long"),
  path: z
    .string()
    .min(1, "Path is required")
    .refine((p) => p.startsWith("/"), {
      message: "Path must be absolute"
    })
})
