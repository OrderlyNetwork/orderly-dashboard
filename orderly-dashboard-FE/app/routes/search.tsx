import { Card } from '@radix-ui/themes';
import { useNavigate, useSearchParams } from '@remix-run/react';
import { FC } from 'react';
import { match } from 'ts-pattern';

import { Spinner } from '~/components';
import { ChainNamespace, useSearchAddress } from '~/hooks';
import { base64UrlSafeDecode } from '~/util';

export const Search: FC = () => {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const rawAddress = searchParams.get('q');
  const chainNamespace: ChainNamespace = match(searchParams.get('chain_namespace'))
    .with('evm', () => 'evm' as const)
    .with('sol', () => 'sol' as const)
    .otherwise(() => 'evm' as const);
  const address = match(chainNamespace)
    .with('evm', () => rawAddress)
    .with('sol', () => base64UrlSafeDecode(rawAddress ?? ''))
    .exhaustive();
  const { addressData, loading } = useSearchAddress(address);

  if (address == null) {
    return '';
  }
  if (addressData == null) {
    return <Spinner size="2.5rem" />;
  }

  return (
    <div className="flex flex-col gap-4 sm:gap-8 max-w-full lg:max-w-[40rem] px-2 sm:px-4 flex-justify-center flex-items-center">
      <h2 className="text-lg sm:text-xl break-all text-center">{address}</h2>

      {addressData.length === 0 && !loading && (
        <div className="text-center text-sm sm:text-base">Account not found on any broker</div>
      )}

      <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 w-full">
        {addressData.map((data) => (
          <Card
            className="cursor-pointer flex flex-col gap-3 sm:gap-4 p-3 sm:p-4 hover:bg-gray-800 transition-colors"
            onClick={() => {
              navigate({
                pathname: `/address/${data.address}`,
                search: `?broker_id=${data.broker_id}`
              });
            }}
            key={data.broker_id}
          >
            <div className="flex flex-col [&>*:first-child]:font-bold text-sm sm:text-base">
              <div>Broker ID:</div>
              <div className="break-all">{data.broker_id}</div>
            </div>
            <div className="flex flex-col [&>*:first-child]:font-bold text-sm sm:text-base">
              <div>Account ID:</div>
              <div className="break-all">
                {data.account_id.substring(0, 7)}...{data.account_id.substr(-7)}
              </div>
            </div>
            <div className="flex flex-col [&>*:first-child]:font-bold text-sm sm:text-base">
              <div>Address:</div>
              <div className="break-all">
                {data.address.substring(0, 7)}...{data.address.substr(-7)}
              </div>
            </div>
          </Card>
        ))}
      </div>
      {loading && <Spinner size="2.5rem" />}
    </div>
  );
};
export default Search;
