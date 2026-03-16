import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ fetch }) => {
  try {
    const res = await fetch('/api/books');

    if (!res.ok) {
      throw new Error(`HTTP error: ${res.status}`)
    }

    const data = await res.json();
    return { books: data.results ?? [] };
  } catch (err) {
    console.error(err);
    return { error: "Unable to fetch books" };
  }
};
