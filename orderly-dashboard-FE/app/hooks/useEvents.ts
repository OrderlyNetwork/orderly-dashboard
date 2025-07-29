import dayjs, { Dayjs } from 'dayjs';
import useSWR from 'swr';
import { P, match } from 'ts-pattern';

import { useAppState } from '~/App';
import { types } from '~/types';

export type EventType =
  | 'TRANSACTION'
  | 'PERPTRADE'
  | 'SETTLEMENT'
  | 'LIQUIDATION'
  | 'LIQUIDATIONV2'
  | 'ADL'
  | 'ADLV2';

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
        type: 'liquidation';
        liquidation: types.LiquidationTransfer;
      }
    | {
        type: 'liquidationv2';
        liquidationv2: types.LiquidationTransferV2;
      }
    | {
        type: 'adl';
      }
    | {
        type: 'adlv2';
      }
  );

export type TradingEventCursor = {
  block_time: number;
  block_number: number;
  transaction_index: number;
  log_index: number;
};

export type EventsParams = {
  address: ChainAddress;
  broker_id: string;
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
  broker_id?: string;
  address?: string;
  event_type?: EventType;
  from_time?: number;
  to_time?: number;
  trading_event_next_cursor?: TradingEventCursor;
};

export function useEvents(query: EventsParams | null) {
  const { queryServiceUrl } = useAppState();

  return useSWR<{
    events: EventTableData[];
    nextCursor: TradingEventCursor | null;
    pageSizeLimit: number;
    tradesCount: number;
  }>(
    () => {
      if (query == null) return null;

      return [
        'events_v2',
        query.address.address,
        query.broker_id,
        query.event_type,
        query.from_time?.valueOf(),
        query.to_time?.valueOf(),
        query.trading_event_next_cursor
      ];
    },
    async () => {
      if (query == null) return { events: [], nextCursor: null, pageSizeLimit: 0, tradesCount: 0 };

      if (query.from_time && query.to_time) {
        const fromTime = query.from_time.valueOf() / 1000;
        const toTime = query.to_time.valueOf() / 1000;
        const diffInDays = (toTime - fromTime) / (60 * 60 * 24);

        if (diffInDays > 30) {
          const allEvents: EventTableData[] = [];
          let nextCursorValue: TradingEventCursor | null = null;
          let pageSizeLimit = 0;
          let tradesCount = 0;

          const chunkCount = Math.ceil(diffInDays / 30);
          const chunkSizeMs = 30 * 24 * 60 * 60 * 1000;

          for (let i = 0; i < chunkCount; i++) {
            const chunkFromTime = query.from_time.valueOf() + i * chunkSizeMs;
            const chunkToTime = Math.min(
              query.from_time.valueOf() + (i + 1) * chunkSizeMs,
              query.to_time.valueOf()
            );

            const chunkQuery = {
              ...query,
              from_time: dayjs(chunkFromTime),
              to_time: dayjs(chunkToTime)
            };

            const result = await fetchAllPages(chunkQuery, queryServiceUrl);

            allEvents.push(...result.events);

            if (result.nextCursor !== null) {
              nextCursorValue = result.nextCursor;
            }

            pageSizeLimit = Math.max(pageSizeLimit, result.pageSizeLimit);
            tradesCount += result.tradesCount;
          }

          const sortedEvents = allEvents.sort((a, b) => {
            if (a.block_timestamp === b.block_timestamp) {
              return a.log_index - b.log_index;
            }
            return a.block_timestamp - b.block_timestamp;
          });

          return {
            events: sortedEvents,
            nextCursor: nextCursorValue,
            pageSizeLimit,
            tradesCount
          };
        }
      }

      return fetchAllPages(query, queryServiceUrl);
    },
    {
      revalidateOnFocus: false
    }
  );
}

async function fetchAllPages(
  query: EventsParams,
  queryServiceUrl: string
): Promise<{
  events: EventTableData[];
  nextCursor: TradingEventCursor | null;
  pageSizeLimit: number;
  tradesCount: number;
}> {
  const allEvents: EventTableData[] = [];
  let currentCursor: TradingEventCursor | null = query.trading_event_next_cursor || null;
  let pageSizeLimit = 0;
  let tradesCount = 0;

  do {
    const pageQuery = {
      ...query,
      trading_event_next_cursor: currentCursor
    };

    const result = await fetchEvents(pageQuery, queryServiceUrl);

    allEvents.push(...result.events);
    currentCursor = result.nextCursor;
    pageSizeLimit = Math.max(pageSizeLimit, result.pageSizeLimit);
    tradesCount += result.tradesCount;

    if (result.events.length === 0) {
      break;
    }
  } while (currentCursor !== null);

  return {
    events: allEvents,
    nextCursor: currentCursor,
    pageSizeLimit,
    tradesCount
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

  requestBody.broker_id = query.broker_id;
  requestBody.address = query.address.address;

  if (query.event_type != null) {
    requestBody.event_type = query.event_type;
  }

  if (query.from_time != null) {
    requestBody.from_time = Math.trunc(query.from_time.valueOf() / 1_000);
  }

  if (query.to_time != null) {
    requestBody.to_time = Math.trunc(query.to_time.valueOf() / 1_000);
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
      .exhaustive();
  });

  return {
    events: flattenedEvents,
    nextCursor: val.data.trading_event_next_cursor,
    pageSizeLimit: val.data.page_size_limit,
    tradesCount: val.data.trades_count
  };
}
