import { keccak256 } from '@ethersproject/keccak256';
import useSWR from 'swr';

import { useAppState } from '~/App';

export type PerpSymbol = {
  symbol: string;
  base_tick: number;
  quote_tick: number;
};

export type AllSymbol = {
  symbol: string;
  symbol_hash: string;
};

const encoder = new TextEncoder();

export const useSymbols = () => {
  const { evmApiUrl } = useAppState();
  const { data: symbols } = useSWR<PerpSymbol[]>(`${evmApiUrl}/v1/public/info`, (url: string) =>
    fetch(url)
      .then((r) => r.json())
      .then((val) => {
        if (!val.success) {
          const error = new Error('');
          error.message = val.message;
          throw error;
        }
        return val.data.rows as PerpSymbol[];
      })
  );
  return symbols;
};

export const useAllSymbols = () => {
  const { queryServiceUrl } = useAppState();
  const { data: allSymbols } = useSWR<AllSymbol[]>(`${queryServiceUrl}/symbols`, (url: string) =>
    fetch(url)
      .then((r) => r.json())
      .then((val) => {
        if (!val.success) {
          const error = new Error('');
          error.message = val.err_msg || 'Failed to fetch symbols';
          throw error;
        }
        return val.data.rows as AllSymbol[];
      })
  );
  return allSymbols;
};

export function getSymbolName(
  name: string,
  symbols: PerpSymbol[] | undefined,
  allSymbols?: AllSymbol[] | undefined
) {
  const fromActive = symbols?.find(
    ({ symbol }) => keccak256(encoder.encode(symbol)) === name
  )?.symbol;
  if (fromActive) return fromActive;
  return allSymbols?.find(({ symbol_hash }) => symbol_hash === name)?.symbol ?? '';
}

export function getSymbolBaseTick(name: string, symbols: PerpSymbol[] | undefined) {
  return (
    symbols?.find(({ symbol }) => keccak256(encoder.encode(symbol)) === name)?.base_tick ?? 0.01
  );
}

export function getSymbolQuoteTick(name: string | undefined, symbols: PerpSymbol[] | undefined) {
  if (!name) return undefined;
  return symbols?.find(({ symbol }) => keccak256(encoder.encode(symbol)) === name)?.quote_tick;
}

export function getMaxFractionDigits(baseTick: number): number {
  if (baseTick >= 1) return 0;

  const baseTickStr = baseTick.toString();
  const decimalIndex = baseTickStr.indexOf('.');
  if (decimalIndex === -1) return 0;

  return baseTickStr.length - decimalIndex - 1;
}

export type ParsedPerpSymbol = {
  prefix: string;
  base: string;
  quote: string;
  broker: string | null;
};

/**
 * Parse a perp symbol into its parts. Format: `PERP_{BASE}_{QUOTE}[_{BROKER}]`.
 * For non-PERP strings or malformed input, returns the raw string as `base`
 * with empty quote and null broker (so callers fall back gracefully).
 */
export function parsePerpSymbol(symbol: string | null | undefined): ParsedPerpSymbol {
  if (!symbol) return { prefix: '', base: '', quote: '', broker: null };
  const parts = symbol.split('_');
  if (parts.length < 3 || parts[0] !== 'PERP') {
    return { prefix: '', base: symbol, quote: '', broker: null };
  }
  return {
    prefix: 'PERP',
    base: parts[1],
    quote: parts[2],
    broker: parts.length >= 4 ? parts.slice(3).join('_') : null
  };
}

/** Extract the BASE token (e.g. `BTC` for both `PERP_BTC_USDC` and `PERP_BTC_USDC_mythos`). */
export function getBaseToken(symbol: string | null | undefined): string {
  return parsePerpSymbol(symbol).base;
}

/** Extract the quote token (e.g. `USDC`). Empty string for malformed symbols. */
export function getQuoteToken(symbol: string | null | undefined): string {
  return parsePerpSymbol(symbol).quote;
}

/**
 * Extract the broker suffix (e.g. `mythos`) for permissionless listings.
 * Returns `null` for canonical markets.
 */
export function getBroker(symbol: string | null | undefined): string | null {
  return parsePerpSymbol(symbol).broker;
}

/**
 * URL slug for a market: `BASE` for canonical markets, `BASE_broker` for
 * permissionless variants. Used in `/markets/...` paths.
 */
export function getShortSlug(symbol: string | null | undefined): string {
  const { base, broker } = parsePerpSymbol(symbol);
  if (!base) return '';
  return broker ? `${base}_${broker}` : base;
}
