import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ fetch, params }) => {
  try {
    const res = await fetch(`/api/v1/books/${params.id}`);

    if (!res.ok) {
      throw new Error(`HTTP error: ${res.status}`)
    }

    const data = await res.json();
    return data;
  } catch (err) {
    console.error(err);
    return { error: "Unable to fetch books" };
  }
};
