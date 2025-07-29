import { Select } from '@radix-ui/themes';
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
    if (broker != null || brokers == null) return;
    setBroker(brokers[0].broker_id);
  }, [broker, brokers]);

  useEffect(() => {
    const broker_id = searchParams.get('broker_id');
    if (broker_id == null) return;
    setBroker(broker_id);
  }, [searchParams]);

  useEffect(() => {
    if (broker == null) return;
    searchParams.set('broker_id', broker);
    setSearchParams(searchParams);
  }, [broker, searchParams, setSearchParams]);

  if (!brokers || isLoading) {
    return <Spinner size="1.2rem" inline />;
  }

  return (
    <div className="w-full sm:w-auto">
      <Select.Root
        defaultValue={brokers[0].broker_id}
        onValueChange={(value) => {
          setBroker(value);
        }}
        value={broker}
      >
        <Select.Trigger className="w-full sm:w-auto" />
        <Select.Content>
          {brokers.map(({ broker_id, broker_name }) => (
            <Select.Item key={broker_id} value={broker_id}>
              {broker_name}
            </Select.Item>
          ))}
        </Select.Content>
      </Select.Root>
    </div>
  );
};
