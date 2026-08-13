// Shared plain fetchers for the three Orderly APIs.
// Used by SWR hooks (app/hooks/*) and WebMCP tools (app/webmcp/*) alike so that
// agents and the UI hit the exact same endpoints with the same envelope handling.

type EvmEnvelope<T> = {
  success: boolean;
  data: T;
  code?: string;
  message?: string;
  ts?: number;
};

type QueryEnvelope<T> = {
  success: boolean;
  data: T;
  err_code?: number;
  err_msg?: string | null;
};

const DATA_API_URL = import.meta.env.DATA_API_URL;

// Generic raw GET (no envelope). Used by DATA_API and external (CoinGecko/DefiLlama).
export async function fetchJson<T>(url: string): Promise<T> {
  const res = await fetch(url);
  if (!res.ok) throw new Error(`${res.status} ${res.statusText}`);
  return res.json() as Promise<T>;
}

// The DATA API returns a bare payload ({ rows, data, … } or an array) on success,
// but signals some errors with a 200 body of { success: false, code, message }.
// Surface that envelope as a thrown error so a tool never hands an agent an
// `undefined` derived from `.rows` of an error object.
type DataApiErrorEnvelope = { success: false; code?: string | number; message?: string };

function isDataApiErrorEnvelope(v: unknown): v is DataApiErrorEnvelope {
  if (!v || typeof v !== 'object') return false;
  return (v as Record<string, unknown>).success === false;
}

export async function fetchDataApi<T>(path: string): Promise<T> {
  const json = await fetchJson<unknown>(`${DATA_API_URL}${path}`);
  if (isDataApiErrorEnvelope(json)) {
    throw new Error(`Data API error: ${json.message || json.code || 'request failed'}`);
  }
  return json as T;
}

// EVM REST GET — Orderly envelope `{ success, data, code?, message? }`.
export async function fetchEvmGet<T>(evmApiUrl: string, path: string): Promise<T> {
  const res = await fetch(`${evmApiUrl}${path}`);
  if (!res.ok) throw new Error(`${res.status} ${res.statusText}`);
  const json = (await res.json()) as EvmEnvelope<T>;
  if (!json.success) {
    throw new Error(json.code || json.message || `API error (${res.status})`);
  }
  return json.data;
}

// EVM Public Info POST query — POST /v1/public/query body `{ type, ...params }`.
export async function fetchEvmQuery<T>(
  evmApiUrl: string,
  type: string,
  params: Record<string, unknown>
): Promise<T> {
  const res = await fetch(`${evmApiUrl}/v1/public/query`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ type, ...params })
  });
  if (!res.ok) throw new Error(`${res.status} ${res.statusText}`);
  const json = (await res.json()) as EvmEnvelope<T>;
  if (!json.success) {
    throw new Error(json.code || json.message || `Public Info API error (${res.status})`);
  }
  return json.data;
}

// Query Service GET — envelope `{ success, data, err_code?, err_msg? }`.
export async function fetchQueryGet<T>(queryServiceUrl: string, path: string): Promise<T> {
  const res = await fetch(`${queryServiceUrl}${path}`);
  if (!res.ok) throw new Error(`${res.status} ${res.statusText}`);
  const json = (await res.json()) as QueryEnvelope<T>;
  if (!json.success) {
    throw new Error(json.err_msg || 'Failed to fetch data');
  }
  return json.data;
}
