import { keccak256 } from '@ethersproject/keccak256';
import useSWR from 'swr';

import { useAppState } from '~/App';

export type PerpSymbol = {
  symbol: string;
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

export function getSymbolName(name: string, symbols: PerpSymbol[] | undefined) {
  return symbols?.find(({ symbol }) => keccak256(encoder.encode(symbol)) === name)?.symbol ?? '';
}
