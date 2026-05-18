import { keccak256 } from '@ethersproject/keccak256';
import useSWR from 'swr';

import { useAppState } from '~/App';

export type Token = {
  token: string;
};

export type AllToken = {
  token: string;
  token_hash: string;
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

export const useAllTokens = () => {
  const { queryServiceUrl } = useAppState();
  const { data: allTokens } = useSWR<AllToken[]>(`${queryServiceUrl}/tokens`, (url: string) =>
    fetch(url)
      .then((r) => r.json())
      .then((val) => {
        if (!val.success) {
          const error = new Error('');
          error.message = val.err_msg || 'Failed to fetch tokens';
          throw error;
        }
        return val.data.rows as AllToken[];
      })
  );
  return allTokens;
};

export function getTokenName(
  hash: string,
  tokens: Token[] | undefined,
  allTokens?: AllToken[] | undefined
) {
  const fromActive = tokens?.find(({ token }) => keccak256(encoder.encode(token)) === hash)?.token;
  if (fromActive) return fromActive;
  return allTokens?.find(({ token_hash }) => token_hash === hash)?.token ?? '';
}
