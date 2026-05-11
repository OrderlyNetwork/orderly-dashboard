import type { MainDailyRow, TvlChainRow } from '~/types/dashboard';

type SymbolWeeklyRaw = {
  trade_week: string;
  listed_markets_count?: number | null;
  [key: string]: unknown;
};

const BASE = import.meta.env.DATA_API_URL;

async function fetchJson<T>(path: string): Promise<T> {
  try {
    const res = await fetch(`${BASE}${path}`);
    if (!res.ok) return {} as T;
    return res.json();
  } catch {
    return {} as T;
  }
}

function fmtDate(d: Date) {
  return d.toISOString().slice(0, 10);
}

export async function fetchDashboardData(days = 90) {
  const end = new Date();
  const start = new Date();
  start.setDate(start.getDate() - days);

  const [mainRes, tvlRes, symbolRes] = await Promise.all([
    fetchJson<{ rows: MainDailyRow[] }>(
      `/orderly/api/v1/dashboard/orderly/main?start_date=${fmtDate(start)}&end_date=${fmtDate(end)}`
    ),
    fetchJson<{ rows: TvlChainRow[] }>(`/orderly/api/v1/dashboard/orderly/tvl-by-chain`),
    fetchJson<{ rows: SymbolWeeklyRaw[] }>(`/orderly/api/v1/dashboard/orderly/by-symbol/weekly`)
  ]);

  const mainRows = [...(mainRes.rows ?? [])].reverse();

  const tvlTotal = (tvlRes.rows ?? []).find((r) => r.is_total)?.tvl_usd ?? 0;
  const tvlChains = (tvlRes.rows ?? []).filter((r) => !r.is_total);

  const aggregateRows = (symbolRes.rows ?? []).filter((r) => r.listed_markets_count != null);
  const marketRows = aggregateRows
    .sort((a, b) => (b.trade_week > a.trade_week ? 1 : -1))
    .map((r) => ({
      trade_week: r.trade_week,
      markets: r.listed_markets_count as number
    }));

  return { mainRows, tvlChains, tvlTotal, marketRows };
}
