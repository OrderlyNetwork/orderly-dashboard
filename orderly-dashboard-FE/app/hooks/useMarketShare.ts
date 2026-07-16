import useSWR from 'swr';

type CoinGeckoDerivative = {
  market: string;
  symbol: string;
  index_id: string;
  price: string;
  price_percentage_change_24h: number;
  contract_type: string;
  index: number;
  basis: number;
  spread: number;
  funding_rate: number;
  open_interest: number;
  volume_24h: number;
  last_traded_at: number;
  expired_at: string | null;
};

type DefiLlamaProtocol = {
  name: string;
  displayName: string;
  module: string;
  slug: string;
  chains: string[];
  logo: string;
  total24h: number | null;
  total7d: number | null;
  total30d: number | null;
  totalAllTime: number | null;
};

type DefiLlamaOIResponse = {
  protocols: DefiLlamaProtocol[];
  [key: string]: unknown;
};

export type DexMarketShare = {
  name: string;
  slug: string;
  logo: string;
  volume24h: number;
  openInterest: number;
  marketShare: number;
  isOrderly: boolean;
};

export type MarketShareData = {
  dexProtocols: DexMarketShare[];
  totalDexVolume: number;
  totalDexOI: number;
  orderlyRank: number;
  lastUpdated: number;
};

const COINGECKO_URL = 'https://api.coingecko.com/api/v3/derivatives';
const DEFILLAMA_OI_URL =
  'https://api.llama.fi/overview/open-interest?excludeTotalDataChart=true&excludeTotalDataChartBreakdown=true';

const CEX_BLOCKLIST = new Set([
  'binance',
  'okx',
  'bybit',
  'bitget',
  'gate',
  'htx',
  'huobi',
  'kucoin',
  'mexc',
  'bitmart',
  'coinbase',
  'kraken',
  'bitfinex',
  'bitmex',
  'deribit',
  'phemex',
  'wootrade',
  'woo',
  'ascendex',
  'bitunix',
  'bingx',
  'blofin',
  'btse',
  'coinex',
  'deepcoin',
  'delta exchange',
  'flipster',
  'hotcoin',
  'lbank',
  'toobit',
  'weex',
  'whitebit',
  'xt',
  'bitrue',
  'bitunix',
  'bvoX',
  'bydfi',
  'coinw',
  'hashkey',
  'hitbtc',
  'kceX',
  'orangex',
  'ourbit',
  'aivora',
  'alphax',
  'antartic',
  'astros',
  'bigone',
  'bitkan',
  'bitflyer',
  'btse',
  'bullish',
  'crypto.com',
  'dipcoin',
  'gmo coin',
  'gemini'
]);

async function fetchJson<T>(url: string): Promise<T> {
  const res = await fetch(url);
  if (!res.ok) throw new Error(`${res.status} ${res.statusText}`);
  return res.json() as Promise<T>;
}

function normalizeMarketName(market: string): string {
  return market
    .replace(/\s*\(Futures\)\s*/gi, '')
    .replace(/\s*\(Derivatives?\)\s*/gi, '')
    .replace(/\s*\(Derivative\)\s*/gi, '')
    .replace(/\s*\(Linea\)\s*/gi, '')
    .replace(/\s*\(Base\)\s*/gi, '')
    .replace(/\s*\(BSC\)\s*/gi, '')
    .replace(/\s*\(opBnb\)\s*/gi, '')
    .replace(/\s*\(Arbitrum\)\s*/gi, '')
    .replace(/\s*\(Polygon\)\s*/gi, '')
    .replace(/\s+Futures?\s*$/gi, '')
    .replace(/\s*Perps?\s*$/gi, '')
    .replace(/\s*Perpetual Exchange\s*$/gi, '')
    .replace(/\s*Perpetuals?\s*$/gi, '')
    .replace(/\s+Trade\s*$/gi, '')
    .replace(/\s*International\s*$/gi, '')
    .replace(/\s*DeFutures?\s*$/gi, '')
    .replace(/\s*Classic\s*$/gi, '')
    .replace(/\s*Omni\s*$/gi, '')
    .replace(/\s*DEX\s*$/gi, '')
    .replace(/\s+Exchange\s*$/gi, '')
    .replace(/\s*Finance\s*$/gi, '')
    .replace(/\s+Chain\s*$/gi, '')
    .replace(/\s*Derivatives?\s*$/gi, '')
    .trim()
    .toLowerCase();
}

function normalizeDefiLlamaName(name: string): string {
  return name
    .replace(/\s*Perps?\s*$/gi, '')
    .replace(/\s*Perpetuals?\s*$/gi, '')
    .replace(/\s*Perpetual Exchange\s*$/gi, '')
    .replace(/\s*V[1-4]\s*$/gi, '')
    .replace(/\s*v[1-4]\+?[v1-4]*\s*$/gi, '')
    .replace(/\s+Trade\s*$/gi, '')
    .replace(/\s*International\s*$/gi, '')
    .replace(/\s*DeFutures?\s*$/gi, '')
    .replace(/\s*Classic\s*$/gi, '')
    .replace(/\s*Omni\s*$/gi, '')
    .replace(/\s*DEX\s*$/gi, '')
    .replace(/\s+Exchange\s*$/gi, '')
    .replace(/\s*Markets?\s*$/gi, '')
    .replace(/\s*Finance\s*$/gi, '')
    .replace(/\s+Chain\s*$/gi, '')
    .replace(/\s*Derivatives?\s*$/gi, '')
    .trim()
    .toLowerCase();
}

export function useCoinGeckoDerivatives() {
  return useSWR<CoinGeckoDerivative[]>(COINGECKO_URL, fetchJson, {
    revalidateOnFocus: false,
    revalidateOnReconnect: false,
    dedupingInterval: 5 * 60 * 1000,
    refreshInterval: 5 * 60 * 1000,
    shouldRetryOnError: false
  });
}

export function useDefiLlamaOI() {
  return useSWR<DefiLlamaOIResponse>(DEFILLAMA_OI_URL, fetchJson, {
    revalidateOnFocus: false,
    revalidateOnReconnect: false,
    dedupingInterval: 60 * 60 * 1000,
    refreshInterval: 60 * 60 * 1000,
    shouldRetryOnError: false
  });
}

export function useMarketShare(): {
  data: MarketShareData | undefined;
  isLoading: boolean;
  error: Error | undefined;
} {
  const { data: cgData, isLoading: cgLoading, error: cgError } = useCoinGeckoDerivatives();
  const { data: dlData, isLoading: dlLoading, error: dlError } = useDefiLlamaOI();

  const isLoading = cgLoading || dlLoading;
  const error = cgError || dlError;

  const data: MarketShareData | undefined =
    cgData && dlData ? computeMarketShare(cgData, dlData) : undefined;

  return { data, isLoading, error };
}

function computeMarketShare(
  cgDerivatives: CoinGeckoDerivative[],
  dlOI: DefiLlamaOIResponse
): MarketShareData {
  const dexProtocolNames = new Set<string>();
  const dexProtocolMap = new Map<string, DefiLlamaProtocol>();

  for (const proto of dlOI.protocols) {
    const normalized = normalizeDefiLlamaName(proto.name);
    if (!CEX_BLOCKLIST.has(normalized)) {
      dexProtocolNames.add(normalized);
      dexProtocolMap.set(normalized, proto);
    }
  }

  const marketVolumes = new Map<string, { volume: number; oi: number }>();
  for (const item of cgDerivatives) {
    const market = item.market;
    const normalized = normalizeMarketName(market);
    if (CEX_BLOCKLIST.has(normalized)) continue;
    const existing = marketVolumes.get(normalized) ?? { volume: 0, oi: 0 };
    existing.volume += Number(item.volume_24h) || 0;
    existing.oi += Number(item.open_interest) || 0;
    marketVolumes.set(normalized, existing);
  }

  const dexProtocols: DexMarketShare[] = [];
  let totalDexVolume = 0;
  let totalDexOI = 0;

  for (const [normalized, volData] of marketVolumes) {
    if (dexProtocolNames.has(normalized)) {
      const proto = dexProtocolMap.get(normalized);
      if (proto && volData.volume > 0) {
        dexProtocols.push({
          name: proto.displayName,
          slug: proto.slug,
          logo: proto.logo,
          volume24h: volData.volume,
          openInterest: volData.oi,
          marketShare: 0,
          isOrderly: normalized === 'orderly'
        });
        totalDexVolume += volData.volume;
        totalDexOI += volData.oi;
      }
    }
  }

  for (const p of dexProtocols) {
    p.marketShare = totalDexVolume > 0 ? (p.volume24h / totalDexVolume) * 100 : 0;
  }

  dexProtocols.sort((a, b) => b.volume24h - a.volume24h);

  const orderlyRank = dexProtocols.findIndex((p) => p.isOrderly) + 1;

  return {
    dexProtocols,
    totalDexVolume,
    totalDexOI,
    orderlyRank,
    lastUpdated: Date.now()
  };
}
