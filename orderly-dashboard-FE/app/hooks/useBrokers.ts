import useSWR from 'swr';

import { useAppState } from '~/App';
import { fetchEvmGet } from '~/services/orderly';

export type Broker = {
  broker_id: string;
  broker_name: string;
};

export function useBrokers() {
  const { evmApiUrl } = useAppState();
  return useSWR<Broker[]>(evmApiUrl ? ['brokers', evmApiUrl] : null, () =>
    fetchEvmGet<{ rows: Broker[] }>(evmApiUrl, '/v1/public/broker/name').then((v) =>
      v.rows.sort((a, b) => a.broker_name.localeCompare(b.broker_name))
    )
  );
}
