// WebMCP reference/metadata tools: broker directory, symbol hash→name map, and
// token hash→name map. Used to render human-readable labels for IDs/hashes that
// other tools return. Read-only.

import { ro, type WebMcpCtx } from './tools';

import { fetchEvmGet, fetchQueryGet } from '~/services/orderly';

export function createMetaTools(ctx: WebMcpCtx): ModelContextTool[] {
  const { evmApiUrl, queryServiceUrl } = ctx;

  return [
    ro(
      'get_brokers',
      'Directory of all Orderly brokers (broker_id → human-readable broker_name), sorted by ' +
        'name. Use to label broker_id values returned by other tools.',
      { type: 'object', properties: {}, additionalProperties: false },
      () =>
        fetchEvmGet<{ rows: { broker_id: string; broker_name: string }[] }>(
          evmApiUrl,
          '/v1/public/broker/name'
        ).then((v) => v.rows.sort((a, b) => a.broker_name.localeCompare(b.broker_name)))
    ),
    ro(
      'get_all_symbols',
      'Every known perp symbol (including delisted) as a hash→name map: {symbol, symbol_hash}. ' +
        'Use to resolve keccak256 symbol hashes in event/position data to display names. Active ' +
        'symbols are also available via get_markets.',
      { type: 'object', properties: {}, additionalProperties: false },
      () =>
        fetchQueryGet<{ rows: { symbol: string; symbol_hash: string }[] }>(
          queryServiceUrl,
          '/symbols'
        ).then((v) => v.rows)
    ),
    ro(
      'get_all_tokens',
      'Every known settlement/collateral token as a hash→name map: {token, token_hash}. Use to ' +
        'resolve keccak256 token hashes in settlement/fee fields to display names.',
      { type: 'object', properties: {}, additionalProperties: false },
      () =>
        fetchQueryGet<{ rows: { token: string; token_hash: string }[] }>(
          queryServiceUrl,
          '/tokens'
        ).then((v) => v.rows)
    )
  ];
}
