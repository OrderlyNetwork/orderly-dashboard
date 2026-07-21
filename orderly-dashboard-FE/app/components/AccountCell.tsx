import { Link } from '@remix-run/react';
import { FC } from 'react';
import useSWR from 'swr';

import { useAppState } from '~/App';
import { Broker } from '~/hooks/useBrokers';
import { base64UrlSafeEncode } from '~/util';

type AccountInfo = {
  address: string;
  broker_id?: string;
};

const formatAddress = (address: string) =>
  `${address.substring(0, 6)}...${address.substring(address.length - 4)}`;

const isSolanaAddress = (address: string) => /^[0-9a-zA-Z]{43,44}$/.test(address);

const addressLink = (address: string, brokerId?: string): string => {
  if (brokerId) return `/address/${address}?broker_id=${brokerId}`;
  if (isSolanaAddress(address)) {
    return `/search?q=${base64UrlSafeEncode(address)}&chain_namespace=sol`;
  }
  return `/search?q=${address}&chain_namespace=evm`;
};

export const AccountCell: FC<{
  accountId: string;
  brokers?: Broker[];
}> = ({ accountId, brokers }) => {
  const { evmApiUrl } = useAppState();

  const { data, isLoading } = useSWR<AccountInfo>(
    `${evmApiUrl}/v1/public/account?account_id=${accountId}`,
    async (url: string) => {
      const response = await fetch(url);
      const val = await response.json();
      if (!val.success) throw new Error(val.message);
      return val.data as AccountInfo;
    },
    {
      revalidateOnFocus: false,
      shouldRetryOnError: false,
      dedupingInterval: 60000
    }
  );

  if (isLoading) {
    return <span style={{ color: 'rgba(255,255,255,0.3)' }}>…</span>;
  }

  if (!data?.address) {
    return <span style={{ color: 'rgba(255,255,255,0.3)' }}>—</span>;
  }

  const brokerName = data.broker_id
    ? (brokers?.find((b) => b.broker_id === data.broker_id)?.broker_name ?? data.broker_id)
    : undefined;

  return (
    <div className="flex flex-col gap-0.5">
      <Link to={addressLink(data.address, data.broker_id)} className="font-address text-[13px]">
        {formatAddress(data.address)}
      </Link>
      {brokerName && (
        <span style={{ color: 'rgba(255,255,255,0.35)', fontSize: 11 }}>{brokerName}</span>
      )}
    </div>
  );
};
