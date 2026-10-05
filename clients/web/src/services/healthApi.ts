import { api } from "./api";

interface HealthResponse {
  status: string;
}

export const healthApi = api.injectEndpoints({
  endpoints: (builder) => ({
    getHealth: builder.query<HealthResponse, void>({
      query: () => "/health",
    }),
  }),
});

export const { useGetHealthQuery } = healthApi;
