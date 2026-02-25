import { FixedNumber } from '@tarnadas/fixed-number';
import dayjs, { Dayjs } from 'dayjs';
import { useState, useCallback } from 'react';
import { P, match } from 'ts-pattern';

import type { EventTableData, TradingEventCursor, EventType } from './useEvents';
import { getSymbolName, type PerpSymbol } from './useSymbols';
import { getTokenName, type Token } from './useTokens';

import { useAppState } from '~/App';
import { types } from '~/types';

export type BulkFetchParams = {
  account_id: string;
  from_time?: Dayjs | null;
  to_time?: Dayjs | null;
};

type EventsV2RequestBody = {
  account_id?: string;
  event_type?: EventType;
  from_time?: number;
  to_time?: number;
  trading_event_next_cursor?: TradingEventCursor;
};

export type BulkFetchState = {
  isFetching: boolean;
  progress: number;
  totalEvents: number;
  error: string | null;
};

export type BulkFetchResult = {
  trades: AggregatedTrade[];
  liquidations: AggregatedLiquidation[];
};

export type AggregatedTrade = {
  event_type: 'TRADE';
  timestamp: string;
  symbol: string;
  side: 'BUY' | 'SELL';
  qty: string;
  price: string;
  notional: string;
  fee: string;
  fee_asset: string;
  transaction_id: string;
};

export type AggregatedLiquidation = {
  event_type: 'LIQUIDATION';
  timestamp: string;
  symbol: string;
  position_qty: string;
  mark_price: string;
  notional: string;
  fee: string;
  transaction_id: string;
};

export function useBulkEvents() {
  const { queryServiceUrl } = useAppState();
  const [state, setState] = useState<BulkFetchState>({
    isFetching: false,
    progress: 0,
    totalEvents: 0,
    error: null
  });

  const fetchAllEvents = useCallback(
    async (
      params: BulkFetchParams,
      symbols?: PerpSymbol[],
      tokens?: Token[]
    ): Promise<BulkFetchResult> => {
      setState({
        isFetching: true,
        progress: 0,
        totalEvents: 0,
        error: null
      });

      try {
        const allTrades: EventTableData[] = [];
        const allLiquidations: EventTableData[] = [];
        let totalChunks = 0;
        let completedChunks = 0;

        // Calculate date chunks (31 days max each)
        const dateChunks = calculateDateChunks(params.from_time, params.to_time);
        totalChunks = dateChunks.length * 2; // *2 for trades + liquidations

        // Fetch trades for each date chunk
        for (const chunk of dateChunks) {
          let cursor: TradingEventCursor | null = null;
          let hasMore = true;

          while (hasMore) {
            const requestBody: EventsV2RequestBody = {
              account_id: params.account_id,
              event_type: 'PERPTRADE'
            };

            if (chunk.from_time != null) {
              requestBody.from_time = Math.trunc(chunk.from_time.valueOf() / 1_000);
            }

            if (chunk.to_time != null) {
              requestBody.to_time = Math.trunc(chunk.to_time.valueOf() / 1_000);
            }

            if (cursor != null) {
              requestBody.trading_event_next_cursor = cursor;
            }

            const response = await fetch(`${queryServiceUrl}/events_v2`, {
              method: 'POST',
              headers: { 'Content-Type': 'application/json' },
              body: JSON.stringify(requestBody)
            });

            const data = await response.json();

            if (!data.success) {
              throw new Error(data.message || 'Failed to fetch trades');
            }

            const flattened = flattenEvents(data.data.events);
            allTrades.push(...flattened);

            cursor = data.data.trading_event_next_cursor;
            hasMore = cursor != null;
          }

          completedChunks++;
          setState((prev) => ({
            ...prev,
            progress: Math.floor((completedChunks / totalChunks) * 100),
            totalEvents: allTrades.length + allLiquidations.length
          }));
        }

        // Fetch liquidations for each date chunk
        for (const chunk of dateChunks) {
          let cursor: TradingEventCursor | null = null;
          let hasMore = true;

          while (hasMore) {
            const requestBody: EventsV2RequestBody = {
              account_id: params.account_id,
              event_type: 'LIQUIDATION'
            };

            if (chunk.from_time != null) {
              requestBody.from_time = Math.trunc(chunk.from_time.valueOf() / 1_000);
            }

            if (chunk.to_time != null) {
              requestBody.to_time = Math.trunc(chunk.to_time.valueOf() / 1_000);
            }

            if (cursor != null) {
              requestBody.trading_event_next_cursor = cursor;
            }

            const response = await fetch(`${queryServiceUrl}/events_v2`, {
              method: 'POST',
              headers: { 'Content-Type': 'application/json' },
              body: JSON.stringify(requestBody)
            });

            const data = await response.json();

            if (!data.success) {
              throw new Error(data.message || 'Failed to fetch liquidations');
            }

            const flattened = flattenEvents(data.data.events);
            allLiquidations.push(...flattened);

            cursor = data.data.trading_event_next_cursor;
            hasMore = cursor != null;
          }

          completedChunks++;
          setState((prev) => ({
            ...prev,
            progress: Math.floor((completedChunks / totalChunks) * 100),
            totalEvents: allTrades.length + allLiquidations.length
          }));
        }

        // Aggregate trades
        const aggregatedTrades = aggregateTradesForTax(allTrades, symbols, tokens);
        const aggregatedLiquidations = aggregateLiquidationsForTax(allLiquidations, symbols);

        setState({
          isFetching: false,
          progress: 100,
          totalEvents: aggregatedTrades.length + aggregatedLiquidations.length,
          error: null
        });

        return {
          trades: aggregatedTrades,
          liquidations: aggregatedLiquidations
        };
      } catch (error) {
        setState({
          isFetching: false,
          progress: 0,
          totalEvents: 0,
          error: error instanceof Error ? error.message : 'Unknown error occurred'
        });
        throw error;
      }
    },
    [queryServiceUrl]
  );

  return {
    ...state,
    fetchAllEvents
  };
}

function calculateDateChunks(
  from_time?: Dayjs | null,
  to_time?: Dayjs | null
): { from_time: Dayjs | null; to_time: Dayjs | null }[] {
  if (from_time == null || to_time == null) {
    return [{ from_time: from_time ?? null, to_time: to_time ?? null }];
  }

  const chunks: { from_time: Dayjs; to_time: Dayjs }[] = [];

  // Work in Unix milliseconds to avoid DST issues
  const startMs = from_time.valueOf();
  const endMs = to_time.valueOf();
  const MAX_CHUNK_MS = 30 * 24 * 3600 * 1000; // 30 days to be safe (< 31 days requirement)

  let currentStartMs = startMs;

  while (currentStartMs < endMs) {
    // Calculate end of this chunk (max 30 days from start)
    const currentEndMs = Math.min(currentStartMs + MAX_CHUNK_MS, endMs);

    chunks.push({
      from_time: dayjs(currentStartMs),
      to_time: dayjs(currentEndMs)
    });

    // Move to next chunk (add 1ms to avoid overlap)
    currentStartMs = currentEndMs + 1;
  }

  return chunks;
}

function flattenEvents(events: types.TradingEvent[]): EventTableData[] {
  const flattened: EventTableData[] = [];

  for (const event of events) {
    match(event.data)
      .with({ ProcessedTrades: P.select() }, (data) => {
        for (const trade of data.trades) {
          flattened.push({ ...event, type: 'trade', trade });
        }
      })
      .with({ LiquidationResult: P.select() }, (data) => {
        for (const liquidation of data.liquidation_transfers) {
          flattened.push({
            ...event,
            type: 'liquidation',
            liquidation
          } as EventTableData);
        }
      })
      .with({ LiquidationResultV2: P.select() }, (data) => {
        for (const liquidationv2 of data.liquidation_transfers) {
          flattened.push({
            ...event,
            type: 'liquidationv2',
            liquidationv2
          } as EventTableData);
        }
      })
      .otherwise(() => {
        // Ignore other event types
      });
  }

  return flattened;
}

function aggregateTradesForTax(
  events: EventTableData[],
  symbols?: PerpSymbol[],
  tokens?: Token[]
): AggregatedTrade[] {
  const tradeGroups = new Map<string, EventTableData[]>();

  // Group trades by aggregation key
  for (const event of events) {
    if (event.type !== 'trade') continue;
    const key = `${event.transaction_id}_${event.block_number}_${event.trade.symbol_hash}_${event.trade.side}`;
    if (!tradeGroups.has(key)) {
      tradeGroups.set(key, []);
    }
    tradeGroups.get(key)!.push(event);
  }

  const aggregated: AggregatedTrade[] = [];

  for (const group of tradeGroups.values()) {
    const first = group[0];
    if (first.type !== 'trade') continue;

    const symbolName = getSymbolName(first.trade.symbol_hash, symbols);
    const feeAssetName = getTokenName(first.trade.fee_asset_hash, tokens);

    if (group.length === 1) {
      const qty = new FixedNumber(first.trade.trade_qty, 8);
      const price = new FixedNumber(first.trade.executed_price, 8);
      const notional = new FixedNumber(first.trade.notional, 6);
      const fee = new FixedNumber(first.trade.fee, 6);

      aggregated.push({
        event_type: 'TRADE',
        timestamp: new Date(first.block_timestamp * 1000).toISOString(),
        symbol: symbolName || first.trade.symbol_hash,
        side: first.trade.side,
        qty: qty.format({ maximumFractionDigits: 2 }),
        price: price.format({ maximumFractionDigits: 2 }),
        notional: notional.format({ maximumFractionDigits: 2 }),
        fee: fee.format({ maximumFractionDigits: 2 }),
        fee_asset: feeAssetName || first.trade.fee_asset_hash,
        transaction_id: first.transaction_id
      });
    } else {
      // Aggregate multiple trades
      let totalQty = new FixedNumber('0', 8);
      let totalNotional = new FixedNumber('0', 6);
      let totalFee = new FixedNumber('0', 6);
      let weightedPrice = new FixedNumber('0', 8);

      for (const event of group) {
        if (event.type !== 'trade') continue;
        const qty = new FixedNumber(event.trade.trade_qty, 8);
        const price = new FixedNumber(event.trade.executed_price, 8);
        const notional = new FixedNumber(event.trade.notional, 6);
        const fee = new FixedNumber(event.trade.fee, 6);

        totalQty = totalQty.add(qty);
        totalNotional = totalNotional.add(notional);
        totalFee = totalFee.add(fee);
        weightedPrice = weightedPrice.add(price.mul(qty));
      }

      // Calculate weighted average price
      const avgPrice =
        totalQty.valueOf() > 0n
          ? weightedPrice.div(totalQty)
          : new FixedNumber(first.trade.executed_price, 8);

      aggregated.push({
        event_type: 'TRADE',
        timestamp: new Date(first.block_timestamp * 1000).toISOString(),
        symbol: symbolName || first.trade.symbol_hash,
        side: first.trade.side,
        qty: totalQty.format({ maximumFractionDigits: 2 }),
        price: avgPrice.format({ maximumFractionDigits: 2 }),
        notional: totalNotional.format({ maximumFractionDigits: 2 }),
        fee: totalFee.format({ maximumFractionDigits: 2 }),
        fee_asset: feeAssetName || first.trade.fee_asset_hash,
        transaction_id: first.transaction_id
      });
    }
  }

  // Sort by timestamp
  return aggregated.sort((a, b) => a.timestamp.localeCompare(b.timestamp));
}

function aggregateLiquidationsForTax(
  events: EventTableData[],
  symbols?: PerpSymbol[]
): AggregatedLiquidation[] {
  const liquidations: AggregatedLiquidation[] = [];

  for (const event of events) {
    if (event.type === 'liquidation') {
      const symbolName = getSymbolName(event.liquidation.symbol_hash, symbols);

      const qty = new FixedNumber(event.liquidation.position_qty_transfer, 8);
      const price = new FixedNumber(event.liquidation.mark_price, 8);
      const notional = new FixedNumber(event.liquidation.cost_position_transfer, 8);
      const fee = new FixedNumber(event.liquidation.insurance_fee, 6);

      liquidations.push({
        event_type: 'LIQUIDATION',
        timestamp: new Date(event.block_timestamp * 1000).toISOString(),
        symbol: symbolName || event.liquidation.symbol_hash,
        position_qty: qty.format({ maximumFractionDigits: 2 }),
        mark_price: price.format({ maximumFractionDigits: 2 }),
        notional: notional.format({ maximumFractionDigits: 2 }),
        fee: fee.format({ maximumFractionDigits: 2 }),
        transaction_id: event.transaction_id
      });
    } else if (event.type === 'liquidationv2') {
      const symbolName = getSymbolName(event.liquidationv2.symbol_hash, symbols);

      const qty = new FixedNumber(event.liquidationv2.position_qty_transfer, 8);
      const price = new FixedNumber(event.liquidationv2.mark_price, 8);
      const notional = new FixedNumber(event.liquidationv2.cost_position_transfer, 8);
      const fee = new FixedNumber(event.liquidationv2.fee, 6);

      liquidations.push({
        event_type: 'LIQUIDATION',
        timestamp: new Date(event.block_timestamp * 1000).toISOString(),
        symbol: symbolName || event.liquidationv2.symbol_hash,
        position_qty: qty.format({ maximumFractionDigits: 2 }),
        mark_price: price.format({ maximumFractionDigits: 2 }),
        notional: notional.format({ maximumFractionDigits: 2 }),
        fee: fee.format({ maximumFractionDigits: 2 }),
        transaction_id: event.transaction_id
      });
    }
  }

  // Sort by timestamp
  return liquidations.sort((a, b) => a.timestamp.localeCompare(b.timestamp));
}

export function generateCombinedCSV(
  trades: AggregatedTrade[],
  liquidations: AggregatedLiquidation[]
): string {
  const headers = [
    'event_type',
    'timestamp',
    'symbol',
    'side',
    'qty',
    'price',
    'notional',
    'fee',
    'fee_asset',
    'transaction_id'
  ];

  // Convert trades to rows
  const tradeRows = trades.map((trade) => [
    trade.event_type,
    trade.timestamp,
    trade.symbol,
    trade.side,
    trade.qty,
    trade.price,
    trade.notional,
    trade.fee,
    trade.fee_asset,
    trade.transaction_id
  ]);

  // Convert liquidations to rows with same column structure
  const liquidationRows = liquidations.map((liq) => [
    liq.event_type,
    liq.timestamp,
    liq.symbol,
    '', // side (empty for liquidations)
    liq.position_qty,
    liq.mark_price,
    liq.notional,
    liq.fee,
    '', // fee_asset (empty for liquidations)
    liq.transaction_id
  ]);

  // Combine and sort by timestamp
  const allRows = [...tradeRows, ...liquidationRows].sort((a, b) =>
    String(a[1]).localeCompare(String(b[1]))
  );

  return [headers, ...allRows].map((row) => row.map(escapeCSV).join(',')).join('\n');
}

function escapeCSV(value: string): string {
  if (value.includes(',') || value.includes('"') || value.includes('\n')) {
    return `"${value.replace(/"/g, '""')}"`;
  }
  return value;
}
