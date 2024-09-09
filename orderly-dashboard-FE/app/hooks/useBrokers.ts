import useSWR from 'swr';

import { useAppState } from '~/App';

export type Broker = {
  broker_id: string;
  broker_name: string;
};

export function useBrokers() {
  const { evmApiUrl } = useAppState();
  return useSWR<Broker[]>(`${evmApiUrl}/v1/public/broker/name`, (url: string) =>
    fetch(url)
      .then((r) => r.json())
      .then((val) => {
        if (!val.success) {
          const error = new Error('');
          error.message = val.message;
          throw error;
        }
        return val.data.rows as Broker[];
      })
  );
}
