import useSWR from 'swr';

import { useAppState } from '~/App';
import { types } from '~/types';

export type EventType = 'TRANSACTION' | 'PERPTRADE' | 'SETTLEMENT' | 'LIQUIDATION' | 'ADL';

export type EventsParams = {
  address: string;
  broker_id: string;
  event_type?: EventType;
};

export function useEvents(query: EventsParams | null) {
  const { queryServiceUrl } = useAppState();
  const searchParams = new URLSearchParams();
  if (query != null) {
    for (const [key, value] of Object.entries(query)) {
      if (value == null) continue;
      searchParams.set(key, value);
    }
  }
  const url = `${queryServiceUrl}/events?${searchParams.toString()}`;
  return useSWR<types.TradingEvent[]>(query != null ? url : null, (url: string) =>
    fetch(url)
      .then((r) => r.json())
      .then((val) => {
        if (!val.success) {
          const error = new Error('');
          console.log('val', val);
          error.message = val.message;
          throw error;
        }
        return val.data.events;
      })
  );
}
