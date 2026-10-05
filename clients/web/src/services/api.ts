import { createApi, fetchBaseQuery } from "@reduxjs/toolkit/query/react";

// Single RTK Query api for the whole app. Features add their endpoints with
// `api.injectEndpoints(...)`, so the store only registers one reducer and one
// middleware.
export const api = createApi({
  reducerPath: "api",
  baseQuery: fetchBaseQuery({ baseUrl: import.meta.env.VITE_API_URL }),
  endpoints: () => ({}),
});
