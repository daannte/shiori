export const ssr = false

import { createClient, type operations } from "@shiori/api-client";
import type { LayoutLoad } from "./$types";
import { error } from "@sveltejs/kit";

type LibrariesResponse = operations["list_libraries"]["responses"]["200"]["content"]["application/json"]

export const load: LayoutLoad = async ({ fetch, depends }) => {
  depends("libraries:create")

  let client = createClient({ fetch })

  let libraries = await loadLibraries(client)

  return { libraries }
};

function loadLibrariesError(status: number): never {
  error(status, { message: "Failed to load libraries", tryAgain: true })
}

async function loadLibraries(client: ReturnType<typeof createClient>): Promise<LibrariesResponse> {
  let res = await client.GET("/api/v1/libraries").catch(() => {
    loadLibrariesError(504)
  })

  if (res.error || !res.data) {
    loadLibrariesError(res.response.status)
  }

  return res.data
}
