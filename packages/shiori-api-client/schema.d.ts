/**
 * This file is auto-generated. Do not edit manually.
 *
 * Run `bun --bun run -F @shiori/api-client gen` to update this file.
 */
export interface paths {
    "/api/v1/auth/login": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        /** Login */
        post: operations["login"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/v1/auth/logout": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        /** Logout */
        post: operations["logout"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/v1/auth/me": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Currently authenticated user */
        get: operations["me"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/v1/auth/refresh-token": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        /** Refresh JWT token */
        post: operations["refresh_token"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/v1/auth/register": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        /** Register */
        post: operations["register"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/v1/filesystem/directories/list": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        /**
         * List filesystem directories.
         * @description The provided path must be **relative** to application's base directory.
         */
        post: operations["list_directories"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/v1/libraries": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** List all libraries. */
        get: operations["list_libraries"];
        put?: never;
        /**
         * Create a new library.
         * @description The directory must already exist on the system.
         */
        post: operations["create_library"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/v1/libraries/{id}/media": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** List library media. */
        get: operations["list_library_media"];
        put?: never;
        /** Upload a new media item to the specified library. */
        post: operations["create_library_media"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/v1/media/{id}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Fetch media item. */
        get: operations["get_media"];
        put?: never;
        post?: never;
        /** Delete a media item. */
        delete: operations["delete_media"];
        options?: never;
        head?: never;
        /** Update media information. */
        patch: operations["patch_media"];
        trace?: never;
    };
    "/api/v1/media/{id}/cover": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Fetch media cover. */
        get: operations["get_media_cover"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/v1/meta": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Info about the server. */
        get: operations["meta"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/v1/metadata/search": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Search for books. */
        get: operations["search_books"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/v1/tokens": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** List all api tokens of authenticated user. */
        get: operations["list_tokens"];
        put?: never;
        /** Create new api token. */
        post: operations["create_token"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/v1/tokens/{key_id}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post?: never;
        /** Delete api token with key id. */
        delete: operations["delete_token"];
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/koreader/{token}/syncs/progress": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        /** Save koreader progress. */
        put: operations["update_progress"];
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/koreader/{token}/syncs/progress/{document}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Retrieve koreader progress. */
        get: operations["get_progress"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/koreader/{token}/users/auth": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Authorize koreader sync user. */
        get: operations["authorize"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
}
export type webhooks = Record<string, never>;
export interface components {
    schemas: {
        ApiToken: {
            /**
             * Format: date-time
             * @description Timestamp of when the token was created.
             * @example 2025-07-25T12:45:19Z
             */
            created_at: string;
            /**
             * Format: date-time
             * @description Timestamp of when the token expires.
             * @example 2026-12-31T00:00:00Z
             */
            expires_at?: string | null;
            /**
             * Format: int32
             * @description Unique identifier for the token.
             * @example 86
             */
            id: number;
            /**
             * @description Short key id for the token.
             * @example 1a2b3c4d
             */
            key_id: string;
            /**
             * Format: date-time
             * @description Timestamp of when the token was last used.
             * @example 2026-03-24T16:33:19Z
             */
            last_used_at?: string | null;
            /**
             * @description Name for the token.
             * @example Koreader Sync
             */
            name: string;
        };
        Media: {
            /**
             * @description Endpoint where the cover is stored.
             * @example /api/v1/media/4/cover
             */
            cover_path?: string | null;
            /**
             * Format: date-time
             * @description Timestamp of when the media was created.
             * @example 2026-03-23T12:00:00Z
             */
            created_at: string;
            /**
             * @description File extension of the media.
             * @example epub
             */
            extension: string;
            /**
             * Format: int32
             * @description Unique identifier for the media item.
             * @example 86
             */
            id: number;
            /**
             * Format: int32
             * @description Library this media belongs to.
             * @example 2
             */
            library_id: number;
            /**
             * @description Name of the media file, excluding extension.
             * @example 86_Volume_1
             */
            name: string;
            /**
             * @description File system path where the media is stored.
             * @example /data/books/light_novels/86_Volume_1.epub
             */
            path: string;
            /**
             * Format: int64
             * @description Size of the media file in bytes.
             * @example 102400
             */
            size: number;
        };
        MediaMetadata: {
            /** @description List of authors associated with the media item. */
            authors: string[];
            /**
             * @description Description of the media item.
             * @example The San Magnolia Republic...
             */
            description?: string | null;
            /** @description List of genres associated with the media item. */
            genres: string[];
            /**
             * @description International Standard Book Number (ISBN).
             *     Typically used for books.
             * @example 1975303121
             */
            isbn?: string | null;
            /**
             * @description Language of the media content.
             * @example English
             */
            language?: string | null;
            /**
             * Format: date
             * @description Date the media was published.
             * @example 2019-03-26
             */
            published?: string | null;
            /**
             * @description Name of the publisher or publishing organization.
             * @example Yen On
             */
            publisher?: string | null;
        };
        PatchMetadata: {
            /** @description List of authors associated with the media item. */
            authors?: string[] | null;
            /**
             * @description Description of the media item.
             * @example The San Magnolia Republic...
             */
            description?: string | null;
            /** @description List of genres associated with the media item. */
            genres?: string[] | null;
            /**
             * @description International Standard Book Number (ISBN).
             *     Typically used for books.
             * @example 1975303121
             */
            isbn?: string | null;
            /**
             * @description Language of the media content.
             * @example English
             */
            language?: string | null;
            /**
             * Format: date
             * @description Date the media was published.
             * @example 2019-03-26
             */
            published?: string | null;
            /**
             * @description Name of the publisher or publishing organization.
             * @example Yen On
             */
            publisher?: string | null;
        };
        ReadingProgress: {
            /**
             * @description Indicates whether this media has been fully read by the user.
             * @example false
             */
            completed: boolean;
            /**
             * Format: date-time
             * @description Timestamp of when the media was completed.
             * @example 2026-05-24T17:21:12Z
             */
            completed_at?: string | null;
            /**
             * Format: double
             * @description Reading progress as a percentage of completion.
             * @example 0.986
             */
            percentage_completed: number;
            /**
             * Format: date-time
             * @description Timestamp of when this progress started.
             * @example 2026-03-24T16:33:19Z
             */
            started_at: string;
            /**
             * Format: date-time
             * @description Timestamp of when this progrses was updated.
             * @example 2026-05-24T17:21:12Z
             */
            updated_at: string;
        };
    };
    responses: never;
    parameters: never;
    requestBodies: never;
    headers: never;
    pathItems: never;
}
export type $defs = Record<string, never>;
export interface operations {
    login: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": {
                    /**
                     * @description Password associated with the account.
                     * @example supercoolpass
                     */
                    password: string;
                    /**
                     * @description Username of the account.
                     * @example Reaper
                     */
                    username: string;
                };
            };
        };
        responses: {
            /** @description Successfully logged in */
            200: {
                headers: {
                    /** @description Sets access_token and refresh_token cookies */
                    "set-cookie"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": {
                        /**
                         * Format: date-time
                         * @description Timestamp of when the user was created.
                         * @example 2025-07-25T12:45:19Z
                         */
                        created_at: string;
                        /**
                         * Format: int32
                         * @description Unique identifier for the user.
                         * @example 86
                         */
                        id: number;
                        /**
                         * @description Indicates whether the user is the owner of the server.
                         * @example false
                         */
                        is_server_owner: boolean;
                        /**
                         * @description Username of the user.
                         * @example Reaper
                         */
                        username: string;
                    };
                };
            };
            /** @description Unauthorized */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Invalid request body */
            422: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Internal server error */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
        };
    };
    logout: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successfully logged out */
            204: {
                headers: {
                    /** @description Removes access_token and refresh_token cookies */
                    "set-cookie"?: string;
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Unauthorized */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Internal server error */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
        };
    };
    me: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successfully retrieved current user */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": {
                        /**
                         * Format: date-time
                         * @description Timestamp of when the user was created.
                         * @example 2025-07-25T12:45:19Z
                         */
                        created_at: string;
                        /**
                         * Format: int32
                         * @description Unique identifier for the user.
                         * @example 86
                         */
                        id: number;
                        /**
                         * @description Indicates whether the user is the owner of the server.
                         * @example false
                         */
                        is_server_owner: boolean;
                        /**
                         * @description Username of the user.
                         * @example Reaper
                         */
                        username: string;
                    };
                };
            };
            /** @description Unauthorized */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Internal server error */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
        };
    };
    refresh_token: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie: {
                /** @description Refresh token */
                refresh_token: string;
            };
        };
        requestBody?: never;
        responses: {
            /** @description Successfully refreshed JWT token */
            204: {
                headers: {
                    /** @description Sets access_token and refresh_token HttpOnly cookies */
                    "set-cookie"?: string;
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Unauthorized */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Internal server error */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
        };
    };
    register: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": {
                    /**
                     * @description Password associated with the account.
                     * @example supercoolpass
                     */
                    password: string;
                    /**
                     * @description Username of the account.
                     * @example Reaper
                     */
                    username: string;
                };
            };
        };
        responses: {
            /** @description Successfully registered */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": {
                        /**
                         * Format: date-time
                         * @description Timestamp of when the user was created.
                         * @example 2025-07-25T12:45:19Z
                         */
                        created_at: string;
                        /**
                         * Format: int32
                         * @description Unique identifier for the user.
                         * @example 86
                         */
                        id: number;
                        /**
                         * @description Indicates whether the user is the owner of the server.
                         * @example false
                         */
                        is_server_owner: boolean;
                        /**
                         * @description Username of the user.
                         * @example Reaper
                         */
                        username: string;
                    };
                };
            };
            /** @description Bad request body */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Insufficient permissions */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Username already taken */
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Invalid request body */
            422: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Internal server error */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
        };
    };
    list_directories: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": {
                    /**
                     * @description Path of the directory to list its subdirectories.
                     * @example
                     */
                    path: string;
                };
            };
        };
        responses: {
            /** @description Successfully listed directories */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": {
                        /** @description Immediate subdirectories of the given path. */
                        directories: string[];
                        /**
                         * @description Parent directory of the given path.
                         * @example null
                         */
                        parent: string | null;
                    };
                };
            };
            /** @description Invalid path */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Access to the request path is not allowed */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Directory does not exist */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Internal server error */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
        };
    };
    list_libraries: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successfully fetched libraries */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": {
                        /**
                         * Format: date-time
                         * @description Timestamp of when the media was created.
                         * @example 2024-11-08T17:23:41Z
                         */
                        created_at: string;
                        /**
                         * Format: int32
                         * @description Unique identifier for the library.
                         * @example 86
                         */
                        id: number;
                        /**
                         * @description Name of the library.
                         * @example Light Novels
                         */
                        name: string;
                        /**
                         * @description File system path to the library's directory.
                         * @example /data/books/light_novels
                         */
                        path: string;
                    }[];
                };
            };
            /** @description Internal server error */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
        };
    };
    create_library: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": {
                    /**
                     * @description Name of the library.
                     * @example Light Novels
                     */
                    name: string;
                    /**
                     * @description File system path to the library's directory, relative to the application's base directory.
                     * @example light_novels
                     */
                    path: string;
                };
            };
        };
        responses: {
            /** @description Successfully created library */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": {
                        /**
                         * Format: date-time
                         * @description Timestamp of when the media was created.
                         * @example 2024-11-08T17:23:41Z
                         */
                        created_at: string;
                        /**
                         * Format: int32
                         * @description Unique identifier for the library.
                         * @example 86
                         */
                        id: number;
                        /**
                         * @description Name of the library.
                         * @example Light Novels
                         */
                        name: string;
                        /**
                         * @description File system path to the library's directory.
                         * @example /data/books/light_novels
                         */
                        path: string;
                    };
                };
            };
            /** @description Invalid request body */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Internal server error */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
        };
    };
    list_library_media: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Id of the library */
                id: number;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successfully fetched library media */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": (components["schemas"]["Media"] & {
                        metadata?: null | components["schemas"]["MediaMetadata"];
                        progress?: null | components["schemas"]["ReadingProgress"];
                    })[];
                };
            };
            /** @description Library not found */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Internal server error */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
        };
    };
    create_library_media: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Id of the library */
                id: number;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "multipart/form-data": {
                    /** @description An array of files to upload. */
                    files: number[];
                };
            };
        };
        responses: {
            /** @description Successfully added media to the library */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": {
                        /**
                         * @description Endpoint where the cover is stored.
                         * @example /api/v1/media/4/cover
                         */
                        cover_path?: string | null;
                        /**
                         * Format: date-time
                         * @description Timestamp of when the media was created.
                         * @example 2026-03-23T12:00:00Z
                         */
                        created_at: string;
                        /**
                         * @description File extension of the media.
                         * @example epub
                         */
                        extension: string;
                        /**
                         * Format: int32
                         * @description Unique identifier for the media item.
                         * @example 86
                         */
                        id: number;
                        /**
                         * Format: int32
                         * @description Library this media belongs to.
                         * @example 2
                         */
                        library_id: number;
                        /**
                         * @description Name of the media file, excluding extension.
                         * @example 86_Volume_1
                         */
                        name: string;
                        /**
                         * @description File system path where the media is stored.
                         * @example /data/books/light_novels/86_Volume_1.epub
                         */
                        path: string;
                        /**
                         * Format: int64
                         * @description Size of the media file in bytes.
                         * @example 102400
                         */
                        size: number;
                    }[];
                };
            };
            /** @description Invalid uploaded media */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Library not found */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Internal server error */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
        };
    };
    get_media: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Id of the media item */
                id: number;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successfully fetched media */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Media"] & {
                        metadata?: null | components["schemas"]["MediaMetadata"];
                        progress?: null | components["schemas"]["ReadingProgress"];
                    };
                };
            };
            /** @description Media not found */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Internal server error */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
        };
    };
    delete_media: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Id of the media item */
                id: number;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successfully deleted media */
            204: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Media not found */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Internal server error */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
        };
    };
    patch_media: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Id of the media item */
                id: number;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": {
                    /**
                     * @description URL of the cover image associated with the media.
                     * @example https://example.com/cover.jpg
                     */
                    cover_url?: string | null;
                    metadata?: null | components["schemas"]["PatchMetadata"];
                    /**
                     * @description Name of the media item.
                     * @example 86—EIGHTY-SIX, Vol. 1
                     */
                    name?: string | null;
                };
            };
        };
        responses: {
            /** @description Successfully updated media information */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Media"] & {
                        metadata?: null | components["schemas"]["MediaMetadata"];
                        progress?: null | components["schemas"]["ReadingProgress"];
                    };
                };
            };
            /** @description Media not found */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Internal server error */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
        };
    };
    get_media_cover: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Id of the media item */
                id: number;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successfully fetched media cover */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Media or cover not found */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Internal server error */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
        };
    };
    meta: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successfully retrieved server metadata */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": {
                        /**
                         * @description Whether the server has been initialized and an owner account exists.
                         * @example true
                         */
                        initialized: boolean;
                    };
                };
            };
            /** @description Internal server error */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
        };
    };
    search_books: {
        parameters: {
            query: {
                /** @description A search query string. */
                q: string;
                /**
                 * @description The provider to use for the search.
                 *     Defaults to "goodreads" if not provided in the query.
                 */
                provider?: string;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successfully found books */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": {
                        /** @description List of authors associated with the media item. */
                        authors: string[];
                        /**
                         * @description URL of the cover image associated with the media.
                         * @example https://example.com/cover.jpg
                         */
                        cover_url?: string | null;
                        /**
                         * @description Description of the media item.
                         * @example The San Magnolia Republic...
                         */
                        description?: string | null;
                        /** @description List of genres associated with the media item. */
                        genres: string[];
                        /**
                         * @description International Standard Book Number (ISBN).
                         *     Typically used for books.
                         * @example 1975303121
                         */
                        isbn?: string | null;
                        /**
                         * @description Language of the media content.
                         * @example English
                         */
                        language?: string | null;
                        /**
                         * Format: int32
                         * @description Provider id of the media item.
                         * @example 41825371
                         */
                        provider_id: number;
                        /**
                         * Format: date
                         * @description Date the media was published.
                         * @example 2019-03-26
                         */
                        published?: string | null;
                        /**
                         * @description Name of the publisher or publishing organization.
                         * @example Yen On
                         */
                        publisher?: string | null;
                        /**
                         * @description Title of the media item.
                         * @example 86—EIGHTY-SIX, Vol. 1
                         */
                        title: string;
                    }[];
                };
            };
            /** @description Invalid query parameters */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Internal server error */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
        };
    };
    list_tokens: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successfully fetched api tokens */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": {
                        /**
                         * Format: date-time
                         * @description Timestamp of when the token was created.
                         * @example 2025-07-25T12:45:19Z
                         */
                        created_at: string;
                        /**
                         * Format: date-time
                         * @description Timestamp of when the token expires.
                         * @example 2026-12-31T00:00:00Z
                         */
                        expires_at?: string | null;
                        /**
                         * Format: int32
                         * @description Unique identifier for the token.
                         * @example 86
                         */
                        id: number;
                        /**
                         * @description Short key id for the token.
                         * @example 1a2b3c4d
                         */
                        key_id: string;
                        /**
                         * Format: date-time
                         * @description Timestamp of when the token was last used.
                         * @example 2026-03-24T16:33:19Z
                         */
                        last_used_at?: string | null;
                        /**
                         * @description Name for the token.
                         * @example Koreader Sync
                         */
                        name: string;
                    }[];
                };
            };
            /** @description Unauthorized */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Internal server error */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
        };
    };
    create_token: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": {
                    /**
                     * Format: date-time
                     * @description Timestamp of when the token expires.
                     * @example 2026-12-31T00:00:00Z
                     */
                    expires_at?: string | null;
                    /**
                     * @description Name of the token
                     * @example Koreader Sync
                     */
                    name: string;
                };
            };
        };
        responses: {
            /** @description Successfully created api token */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiToken"] & {
                        /**
                         * @description The plaintext representation of the API token.
                         * @example shiori_1a2b3c4d_1a2b3c4d5e6f7g8h9i0j1234567890ab
                         */
                        plaintoken: string;
                    };
                };
            };
            /** @description Bad Request */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Unauthorized */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Invalid request body */
            422: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Internal server error */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
        };
    };
    delete_token: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Key id of the api token */
                key_id: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successfully deleted api token */
            204: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Unauthorized */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Not found */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Internal server error */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
        };
    };
    update_progress: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": {
                    /** @description Name of the Koreader device that the progress is being tracked on. */
                    device: string;
                    /** @description Unique device identifier (UUID) assigned by Koreader to the specific device. */
                    device_id: string;
                    /** @description Hash of the book in the Koreader system. */
                    document: string;
                    /**
                     * Format: double
                     * @description Reading progress as a percentage of completion.
                     */
                    percentage: number;
                    /**
                     * @description Current position in the book. There are two types of progress measurement:
                     *         - A page number (for paginated books).
                     *         - An x-pointer (for DOM-based books, using their scrolling reader).
                     */
                    progress: string;
                };
            };
        };
        responses: {
            /** @description Successfully saved progress */
            204: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Unauthorized */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Media not found */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
        };
    };
    get_progress: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Hash of the book in the Koreader system */
                document: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successfully retrieved progress */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": {
                        device: string;
                        device_id: string;
                        percentage: string;
                        progress: string;
                        /**
                         * Format: int64
                         * @description Timestamp of the progress update, in Unix epoch time.
                         */
                        timestamp: number;
                    };
                };
            };
            /** @description Unauthorized */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
        };
    };
    authorize: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successfully authorized user */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Unauthorized */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
        };
    };
}
