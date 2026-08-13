import { keccak256 } from '@ethersproject/keccak256';
import useSWR from 'swr';

import { useAppState } from '~/App';
import { fetchEvmGet, fetchQueryGet } from '~/services/orderly';

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
  const { data: tokens } = useSWR<Token[]>(evmApiUrl ? ['tokens', evmApiUrl] : null, () =>
    fetchEvmGet<{ rows: Token[] }>(evmApiUrl, '/v1/public/token').then((v) => v.rows)
  );
  return tokens;
};

export const useAllTokens = () => {
  const { queryServiceUrl } = useAppState();
  const { data: allTokens } = useSWR<AllToken[]>(
    queryServiceUrl ? ['allTokens', queryServiceUrl] : null,
    () => fetchQueryGet<{ rows: AllToken[] }>(queryServiceUrl, '/tokens').then((v) => v.rows)
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
