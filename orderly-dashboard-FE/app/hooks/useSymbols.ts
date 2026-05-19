import { keccak256 } from '@ethersproject/keccak256';
import useSWR from 'swr';

import { useAppState } from '~/App';

export type PerpSymbol = {
  symbol: string;
  base_tick: number;
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

export function getMaxFractionDigits(baseTick: number): number {
  if (baseTick >= 1) return 0;

  const baseTickStr = baseTick.toString();
  const decimalIndex = baseTickStr.indexOf('.');
  if (decimalIndex === -1) return 0;

  return baseTickStr.length - decimalIndex - 1;
}
