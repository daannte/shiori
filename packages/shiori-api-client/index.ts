import createOpenAPIClient, { type ClientOptions } from 'openapi-fetch';

import type { components, operations, paths } from './schema';
export type { components, operations, paths };

export function createClient(options?: ClientOptions) {
  let baseUrl = import.meta.env.PROD ? window.location.origin : "http://localhost:3000"

  const mergedOptions = {
    baseUrl,
    ...options
  }

  return createOpenAPIClient<paths>(mergedOptions)
}
