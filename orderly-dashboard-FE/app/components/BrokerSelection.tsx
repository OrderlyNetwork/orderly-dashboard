import { useSearchParams } from '@remix-run/react';
import { FC, useEffect, useState } from 'react';

import { Spinner } from '.';

import { useBrokers } from '~/hooks';

export const BrokerSelection: FC = () => {
  const [searchParams, setSearchParams] = useSearchParams();
  const [broker, setBroker] = useState<string | undefined>(
    searchParams.get('broker_id') ?? undefined
  );
  const { data: brokers, isLoading } = useBrokers();

  useEffect(() => {
    const broker_id = searchParams.get('broker_id');
    if (broker_id == null) return;
    setBroker(broker_id);
  }, [searchParams]);

  useEffect(() => {
    if (broker == null) {
      searchParams.delete('broker_id');
    } else {
      searchParams.set('broker_id', broker);
    }
    setSearchParams(searchParams);
  }, [broker, searchParams, setSearchParams]);

  if (!brokers || isLoading) {
    return <Spinner size="1.2rem" inline />;
  }

  return (
    <div className="w-full sm:w-auto">
      <select
        value={broker || ''}
        onChange={(e) => {
          setBroker(e.target.value === '' ? undefined : e.target.value);
        }}
        className="w-full sm:w-64"
      >
        <option value="">All Brokers</option>
        {brokers.map(({ broker_id, broker_name }) => (
          <option key={broker_id} value={broker_id}>
            {broker_name}
          </option>
        ))}
      </select>
    </div>
  );
};
