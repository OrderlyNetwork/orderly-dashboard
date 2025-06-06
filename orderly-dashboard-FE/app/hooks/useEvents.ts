import { Dayjs } from 'dayjs';
import useSWRInfinite from 'swr/infinite';
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
};

export type ChainAddress = {
  address: string;
  chain_namespace: ChainNamespace;
};

export type ChainNamespace = 'evm' | 'sol';

export function useEvents(query: EventsParams | null) {
  const { queryServiceUrl } = useAppState();
  return useSWRInfinite<EventTableData[]>(
    (index) => {
      if (query == null) return null;

      const searchParams = new URLSearchParams();
      searchParams.set('address', query.address.address);
      searchParams.set('broker_id', query.broker_id);
      if (query.event_type != null) {
        searchParams.set('event_type', query.event_type);
      }

      if (query.from_time == null || query.to_time == null) {
        if (index > 0) {
          return null;
        }
        return match(query.address.chain_namespace)
          .with('evm', () => [`${queryServiceUrl}/events`])
          .with('sol', () => [`${queryServiceUrl}/events`, `${queryServiceUrl}/sol_events`])
          .exhaustive();
      }

      const minFromTime = query.from_time;
      let fromTime = query.to_time.subtract(2 * (index + 1), 'weeks');
      const toTime = query.to_time.subtract(2 * index, 'weeks');

      if (fromTime.valueOf() < minFromTime.valueOf()) {
        fromTime = minFromTime;
      }
      if (toTime.valueOf() < minFromTime.valueOf()) {
        return null;
      }

      searchParams.set('from_time', String(Math.trunc(fromTime.valueOf() / 1_000)));
      searchParams.set('to_time', String(Math.trunc(toTime.valueOf() / 1_000)));
      // we add `from` and `to` params here to invalidate SWR cache
      return match(query.address.chain_namespace)
        .with('evm', () => [
          `${queryServiceUrl}/events?${searchParams.toString()}&from=${query.from_time!.format('L')}&to=${query.to_time!.format('L')}`
        ])
        .with('sol', () => [
          `${queryServiceUrl}/events?${searchParams.toString()}&from=${query.from_time!.format('L')}&to=${query.to_time!.format('L')}`,
          `${queryServiceUrl}/sol_events?${searchParams.toString()}&from=${query.from_time!.format('L')}&to=${query.to_time!.format('L')}`
        ])
        .exhaustive();
    },
    (urls: string[]) =>
      Promise.all(
        urls.map((url) =>
          fetch(url)
            .then((r) => r.json())
            .then(handleFetchEvents)
        )
      ).then((data: EventTableData[][]) =>
        data.flat().sort((dataA, dataB) => dataA.block_timestamp - dataB.block_timestamp)
      ),
    {
      parallel: true,
      initialSize: 100,
      persistSize: true,
      revalidateOnFocus: false
    }
  );
}

function handleFetchEvents(val: {
  success: boolean;
  message: string;
  data: { events: types.TradingEvent[] };
}) {
  if (!val.success) {
    const error = new Error('');
    error.message = val.message;
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
  return flattenedEvents;
}
