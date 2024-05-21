import { keccak256 } from '@ethersproject/keccak256';
import useSWR from 'swr';

import { useAppState } from '~/App';

export type Token = {
  token: string;
};

const encoder = new TextEncoder();

export const useTokens = () => {
  const { evmApiUrl } = useAppState();
  const { data: tokens } = useSWR<Token[]>(`${evmApiUrl}/v1/public/token`, (url: string) =>
    fetch(url)
      .then((r) => r.json())
      .then((val) => {
        if (!val.success) {
          const error = new Error('');
          error.message = val.message;
          throw error;
        }
        return val.data.rows as Token[];
      })
  );
  return tokens;
};

export function getTokenName(hash: string, tokens: Token[] | undefined) {
  return tokens?.find(({ token }) => keccak256(encoder.encode(token)) === hash)?.token ?? '';
}
