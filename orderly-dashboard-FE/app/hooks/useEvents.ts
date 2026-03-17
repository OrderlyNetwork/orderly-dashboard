import { Dayjs } from 'dayjs';
import { useState, useEffect, useCallback } from 'react';
import useSWR from 'swr';
import { P, match } from 'ts-pattern';

import { useAppState } from '~/App';
import { types } from '~/types';

export type EventType =
  | 'TRANSACTION'
  | 'PERPTRADE'
  | 'SETTLEMENT'
  | 'LIQUIDATION'
  | 'ADL'
  | 'MARGINTRANSFER';

export type UIEventType =
  | 'ALL'
  | EventType
  | 'LIQUIDATIONV2'
  | 'LIQUIDATIONV3'
  | 'ADLV2'
  | 'ADLV3'
  | 'SETTLEMENTV3';

export function toBackendEventType(uiEventType: UIEventType): EventType | null {
  switch (uiEventType) {
    case 'ALL':
      return null;
    case 'LIQUIDATIONV2':
    case 'LIQUIDATIONV3':
      return 'LIQUIDATION';
    case 'ADLV2':
    case 'ADLV3':
      return 'ADL';
    case 'SETTLEMENTV3':
      return 'SETTLEMENT';
    default:
      return uiEventType;
  }
}

export function getEventVersion(data: types.TradingEventInnerData): string {
  if ('LiquidationResultV3' in data) return 'v3';
  if ('LiquidationResultV2' in data) return 'v2';
  if ('LiquidationResult' in data) return 'v1';
  if ('AdlResultV3' in data) return 'v3';
  if ('AdlResultV2' in data) return 'v2';
  if ('AdlResult' in data) return 'v1';
  if ('SettlementResultV3' in data) return 'v3';
  if ('SettlementResult' in data) return 'v1';
  return '';
}

export type EventTableData = types.TradingEvent &
  (
    | {
        type: 'transaction';
      }
    | {
        type: 'trade';
        trade: types.Trade;
      }
    | {
        type: 'settlement';
        settlement: types.SettlementExecution;
      }
    | {
        type: 'settlementv3';
        settlementv3: types.SettlementExecutionV3;
      }
    | {
        type: 'liquidation';
        liquidation: types.LiquidationTransfer;
      }
    | {
        type: 'liquidationv2';
        liquidationv2: types.LiquidationTransferV2;
      }
    | {
        type: 'liquidationv3';
        liquidationv3: types.LiquidationTransferV3;
      }
    | {
        type: 'adl';
      }
    | {
        type: 'adlv2';
      }
    | {
        type: 'adlv3';
      }
    | {
        type: 'margintransfer';
      }
  );

export type TradingEventCursor = {
  block_time: number;
  block_number: number;
  transaction_index: number;
  log_index: number;
};

export type EventsParams = {
  account_id: string;
  event_type?: EventType;
  from_time?: Dayjs | null;
  to_time?: Dayjs | null;
  trading_event_next_cursor?: TradingEventCursor | null;
};

export type ChainAddress = {
  address: string;
  chain_namespace: ChainNamespace;
};

export type ChainNamespace = 'evm' | 'sol';

type EventsV2RequestBody = {
  account_id?: string;
  event_type?: EventType;
  from_time?: number;
  to_time?: number;
  trading_event_next_cursor?: TradingEventCursor;
};

const MAX_TIME_RANGE_SECONDS = 31 * 24 * 3600;

export function useEvents(query: EventsParams | null) {
  const { queryServiceUrl } = useAppState();
  const [allEvents, setAllEvents] = useState<EventTableData[]>([]);
  const [nextCursor, setNextCursor] = useState<TradingEventCursor | null>(null);
  const [pageSizeLimit, setPageSizeLimit] = useState<number>(0);
  const [tradesCount, setTradesCount] = useState<number>(0);
  const [isLoadingMore, setIsLoadingMore] = useState<boolean>(false);

  const { data, error, isLoading } = useSWR<{
    events: EventTableData[];
    nextCursor: TradingEventCursor | null;
    pageSizeLimit: number;
    tradesCount: number;
  }>(
    () => {
      if (query == null) return null;

      return [
        'events_v2',
        query.account_id,
        query.event_type,
        query.from_time?.valueOf(),
        query.to_time?.valueOf(),
        query.trading_event_next_cursor
      ];
    },
    async () => {
      if (query == null) return { events: [], nextCursor: null, pageSizeLimit: 0, tradesCount: 0 };

      return fetchEvents(query, queryServiceUrl);
    },
    {
      revalidateOnFocus: false
    }
  );

  useEffect(() => {
    setAllEvents([]);
    setNextCursor(null);
    setPageSizeLimit(0);
    setTradesCount(0);
  }, [
    query?.account_id,
    query?.event_type,
    // eslint-disable-next-line react-hooks/exhaustive-deps
    query?.from_time?.valueOf(),
    // eslint-disable-next-line react-hooks/exhaustive-deps
    query?.to_time?.valueOf()
  ]);

  // Update accumulated data when new data arrives
  useEffect(() => {
    if (data) {
      if (query?.trading_event_next_cursor) {
        // Loading more data - append to existing events
        setAllEvents((prev) => [...prev, ...data.events]);
      } else {
        // New query - replace events
        setAllEvents(data.events);
      }
      setNextCursor(data.nextCursor);
      setPageSizeLimit(data.pageSizeLimit);
      setTradesCount(data.tradesCount);
    }
  }, [data, query?.trading_event_next_cursor]);

  const loadMore = useCallback(async () => {
    if (!nextCursor || isLoadingMore) return;

    setIsLoadingMore(true);
    try {
      const loadMoreQuery = {
        ...query!,
        trading_event_next_cursor: nextCursor
      };

      const result = await fetchEvents(loadMoreQuery, queryServiceUrl);

      setAllEvents((prev) => [...prev, ...result.events]);
      setNextCursor(result.nextCursor);
      setPageSizeLimit(Math.max(pageSizeLimit, result.pageSizeLimit));
      // Don't update tradesCount - it's the total count and should remain constant
    } catch (error) {
      console.error('Error loading more events:', error);
    } finally {
      setIsLoadingMore(false);
    }
  }, [nextCursor, isLoadingMore, query, queryServiceUrl, pageSizeLimit]);

  return {
    events: allEvents,
    nextCursor,
    pageSizeLimit,
    tradesCount,
    error,
    isLoading,
    isLoadingMore,
    loadMore,
    hasMore: nextCursor !== null
  };
}

async function fetchEvents(
  query: EventsParams,
  queryServiceUrl: string
): Promise<{
  events: EventTableData[];
  nextCursor: TradingEventCursor | null;
  pageSizeLimit: number;
  tradesCount: number;
}> {
  const requestBody: EventsV2RequestBody = {};

  requestBody.account_id = query.account_id;

  if (query.event_type != null) {
    requestBody.event_type = query.event_type;
  }

  if (query.from_time != null) {
    requestBody.from_time = Math.trunc(query.from_time.valueOf() / 1_000);
  }

  if (query.to_time != null) {
    requestBody.to_time = Math.trunc(query.to_time.valueOf() / 1_000);
  }

  if (
    requestBody.from_time != null &&
    requestBody.to_time != null &&
    requestBody.to_time - requestBody.from_time > MAX_TIME_RANGE_SECONDS
  ) {
    requestBody.from_time = requestBody.to_time - MAX_TIME_RANGE_SECONDS;
  }

  if (query.trading_event_next_cursor != null) {
    requestBody.trading_event_next_cursor = query.trading_event_next_cursor;
  }

  const response = await fetch(`${queryServiceUrl}/events_v2`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(requestBody)
  });

  const data = await response.json();
  return handleFetchEventsV2(data);
}

function handleFetchEventsV2(val: {
  success: boolean;
  data: {
    events: types.TradingEvent[];
    trading_event_next_cursor: TradingEventCursor | null;
    page_size_limit: number;
    trades_count: number;
  };
  message?: string;
}): {
  events: EventTableData[];
  nextCursor: TradingEventCursor | null;
  pageSizeLimit: number;
  tradesCount: number;
} {
  if (!val.success) {
    const error = new Error('');
    error.message = val.message || 'Failed to fetch events';
    throw error;
  }

  const events: types.TradingEvent[] = val.data.events;
  const flattenedEvents: EventTableData[] = [];

  events.forEach((event) => {
    match(event.data)
      .with(
        {
          Transaction: P.select()
        },
        () => {
          flattenedEvents.push({ ...event, type: 'transaction' });
        }
      )
      .with(
        {
          ProcessedTrades: P.select()
        },
        (data) => {
          for (const trade of data.trades) {
            flattenedEvents.push({ ...event, type: 'trade', trade });
          }
        }
      )
      .with(
        {
          LiquidationResult: P.select()
        },
        (data) => {
          for (const liquidation of data.liquidation_transfers) {
            flattenedEvents.push({ ...event, type: 'liquidation', liquidation });
          }
        }
      )
      .with(
        {
          LiquidationResultV2: P.select()
        },
        (data) => {
          for (const liquidationv2 of data.liquidation_transfers) {
            flattenedEvents.push({ ...event, type: 'liquidationv2', liquidationv2 });
          }
        }
      )
      .with(
        {
          SettlementResult: P.select()
        },
        (data) => {
          for (const settlement of data.settlement_executions) {
            flattenedEvents.push({ ...event, type: 'settlement', settlement });
          }
        }
      )
      .with(
        {
          AdlResult: P.select()
        },
        () => {
          flattenedEvents.push({ ...event, type: 'adl' });
        }
      )
      .with(
        {
          AdlResultV2: P.select()
        },
        () => {
          flattenedEvents.push({ ...event, type: 'adlv2' });
        }
      )
      .with(
        {
          SettlementResultV3: P.select()
        },
        (data) => {
          for (const settlementv3 of data.settlement_executions) {
            flattenedEvents.push({ ...event, type: 'settlementv3', settlementv3 });
          }
        }
      )
      .with(
        {
          LiquidationResultV3: P.select()
        },
        (data) => {
          for (const liquidationv3 of data.liquidation_transfers) {
            flattenedEvents.push({ ...event, type: 'liquidationv3', liquidationv3 });
          }
        }
      )
      .with(
        {
          AdlResultV3: P.select()
        },
        () => {
          flattenedEvents.push({ ...event, type: 'adlv3' });
        }
      )
      .with(
        {
          MarginTransferV3: P.select()
        },
        () => {
          flattenedEvents.push({ ...event, type: 'margintransfer' });
        }
      )
      .exhaustive();
  });

  return {
    events: flattenedEvents,
    nextCursor: val.data.trading_event_next_cursor,
    pageSizeLimit: val.data.page_size_limit,
    tradesCount: val.data.trades_count
  };
}
