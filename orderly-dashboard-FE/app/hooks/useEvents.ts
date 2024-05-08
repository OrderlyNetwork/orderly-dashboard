import { Dayjs } from 'dayjs';
import useSWRInfinite from 'swr/infinite';

import { useAppState } from '~/App';
import { types } from '~/types';

export type EventType = 'TRANSACTION' | 'PERPTRADE' | 'SETTLEMENT' | 'LIQUIDATION' | 'ADL';

export type EventTableData =
  | { type: 'event'; event: types.TradingEvent }
  | { type: 'trade'; trade: types.Trade }
  | { type: 'settlement'; settlement: types.SettlementExecution }
  | { type: 'liquidation'; liquidation: types.LiquidationTransfer };

export type EventsParams = {
  address: string;
  broker_id: string;
  event_type?: EventType;
  from_time?: Dayjs | null;
  to_time?: Dayjs | null;
};

export function useEvents(query: EventsParams | null) {
  const { queryServiceUrl } = useAppState();
  return useSWRInfinite<EventTableData[]>(
    (index) => {
      if (query == null) return null;

      const searchParams = new URLSearchParams();
      searchParams.set('address', query.address);
      searchParams.set('broker_id', query.broker_id);
      if (query.event_type != null) {
        searchParams.set('event_type', query.event_type);
      }

      if (query.from_time == null || query.to_time == null) {
        if (index > 0) {
          return null;
        }
        return `${queryServiceUrl}/events?${searchParams.toString()}`;
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
      const url = `${queryServiceUrl}/events?${searchParams.toString()}&from=${query.from_time.format('L')}&to=${query.to_time.format('L')}`;
      return url;
    },
    (url: string) =>
      fetch(url)
        .then((r) => r.json())
        .then((val) => {
          if (!val.success) {
            const error = new Error('');
            console.log('val', val);
            error.message = val.message;
            throw error;
          }
          return (val.data.events as types.TradingEvent[]).map(
            (event) =>
              ({
                type: 'event',
                event
              }) as EventTableData
          );
        }),
    {
      parallel: true,
      initialSize: 100,
      persistSize: true
    }
  );
}
