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

export type EventsParams = {
  address: ChainAddress;
  broker_id: string;
  event_type?: EventType;
  from_time?: Dayjs | null;
  to_time?: Dayjs | null;
  offset?: number;
};

export type ChainAddress = {
  address: string;
  chain_namespace: ChainNamespace;
};

export type ChainNamespace = 'evm' | 'sol';

export function useEvents(query: EventsParams | null) {
  const { queryServiceUrl } = useAppState();

  return useSWR<{
    events: EventTableData[];
    nextOffset: number | null;
  }>(
    () => {
      if (query == null) return null;

      // Return a unique key for the query
      return [
        'events',
        query.address.address,
        query.broker_id,
        query.event_type,
        query.from_time?.valueOf(),
        query.to_time?.valueOf(),
        query.offset
      ];
    },
    async () => {
      if (query == null) return { events: [], nextOffset: null };

      // Check if we need to split the time range
      if (query.from_time && query.to_time) {
        const fromTime = query.from_time.valueOf() / 1000;
        const toTime = query.to_time.valueOf() / 1000;
        const diffInDays = (toTime - fromTime) / (60 * 60 * 24);

        // If time range is greater than 30 days, split into multiple requests
        if (diffInDays > 30) {
          const allEvents: EventTableData[] = [];
          let nextOffsetValue: number | null = null;

          // Calculate how many chunks we need
          const chunkCount = Math.ceil(diffInDays / 30);
          const chunkSizeMs = 30 * 24 * 60 * 60 * 1000; // 30 days in milliseconds

          // Make requests for each chunk
          for (let i = 0; i < chunkCount; i++) {
            const chunkFromTime = query.from_time.valueOf() + i * chunkSizeMs;
            const chunkToTime = Math.min(
              query.from_time.valueOf() + (i + 1) * chunkSizeMs,
              query.to_time.valueOf()
            );

            // Create a modified query for this chunk - convert Date to Dayjs
            const chunkQuery = {
              ...query,
              from_time: dayjs(chunkFromTime),
              to_time: dayjs(chunkToTime)
            };

            // Make the request for this chunk
            const result = await fetchEvents(chunkQuery, queryServiceUrl);

            // Add events to our collection
            allEvents.push(...result.events);

            // Keep track of nextOffset if any chunk has one
            if (result.nextOffset !== null) {
              nextOffsetValue = result.nextOffset;
            }
          }

          // Sort all events by timestamp and log_index
          const sortedEvents = allEvents.sort((a, b) => {
            if (a.block_timestamp === b.block_timestamp) {
              return a.log_index - b.log_index;
            }
            return a.block_timestamp - b.block_timestamp;
          });

          return {
            events: sortedEvents,
            nextOffset: nextOffsetValue
          };
        }
      }

      // For normal requests (within 31 days), use the standard approach
      return fetchEvents(query, queryServiceUrl);
    },
    {
      revalidateOnFocus: false
    }
  );
}

// Helper function to fetch events for a specific query
async function fetchEvents(
  query: EventsParams,
  queryServiceUrl: string
): Promise<{ events: EventTableData[]; nextOffset: number | null }> {
  const searchParams = new URLSearchParams();
  searchParams.set('address', query.address.address);
  searchParams.set('broker_id', query.broker_id);

  if (query.event_type != null) {
    searchParams.set('event_type', query.event_type);
  }

  if (query.offset != null) {
    searchParams.set('offset', query.offset.toString());
  }

  // Handle time range - simplify this since we're only dealing with Dayjs objects now
  if (query.from_time != null) {
    searchParams.set('from_time', String(Math.trunc(query.from_time.valueOf() / 1_000)));
  }

  if (query.to_time != null) {
    searchParams.set('to_time', String(Math.trunc(query.to_time.valueOf() / 1_000)));
  }

  const url = `${queryServiceUrl}/events_v2?${searchParams.toString()}`;

  // Make a single request regardless of chain namespace
  const response = await fetch(url)
    .then((r) => r.json())
    .then(handleFetchEventsV2);

  // Return the processed response directly
  return response;
}

function handleFetchEventsV2(val: {
  success: boolean;
  message?: string;
  data: {
    events: types.TradingEvent[];
    next_offset: number | null;
  };
}): { events: EventTableData[]; nextOffset: number | null } {
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
    nextOffset: val.data.next_offset
  };
}
