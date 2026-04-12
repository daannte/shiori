import createOpenAPIClient, { type ClientOptions } from 'openapi-fetch';

import type { components, operations, paths } from './schema';
export type { components, operations, paths };

const baseUrl = import.meta.env.PROD ? window.location.origin : "http://localhost:3000"

export function createClient(options: ClientOptions) {
  const client = createOpenAPIClient<paths>({
    baseUrl,
    credentials: "include",
    ...options
  })

  // There has to be a better way to do this but
  // this is what i figured out for now
  let refreshPromise: Promise<Response> | null = null

  client.use({
    async onResponse({ response, request }) {
      if (response.status !== 401) {
        return response
      }

      if (!refreshPromise) {
        refreshPromise = options.fetch!(
          new Request(`${baseUrl}/api/v1/auth/refresh-token`, {
            method: "POST",
            credentials: "include",
          })
        ).finally(() => {
          refreshPromise = null
        })
      }

      const res = await refreshPromise
      if (!res.ok) return response

      return options.fetch!(request)
    },
  })

  return client
}

export function get_cover_url(cover_endpoint: string | null | undefined): string | null {
  if (!cover_endpoint) return null
  return `${baseUrl}${cover_endpoint}`
}

