import createOpenAPIClient, { type ClientOptions } from 'openapi-fetch';

import type { components, operations, paths } from './schema';
export type { components, operations, paths };

const baseUrl = import.meta.env.PROD ? window.location.origin : "http://localhost:3000"

export function createClient(options?: ClientOptions) {

  const mergedOptions = {
    baseUrl,
    ...options
  }

  return createOpenAPIClient<paths>(mergedOptions)
}

export function get_cover_url(cover_endpoint: string | null | undefined): string | null {
  if (!cover_endpoint) return null
  return `${baseUrl}${cover_endpoint}`
}

