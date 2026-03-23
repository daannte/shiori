import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ params, fetch }) => {
  try {
    const res = await fetch(`/api/v1/libraries/${params.id}/media`)

    if (!res.ok) {
      throw new Error(`HTTP error: ${res.status}`)
    }

    const data = await res.json()
    return { media: data }
  } catch (err) {
    console.error(err);
    return { error: "Unable to fetch libraries" };
  }
};
