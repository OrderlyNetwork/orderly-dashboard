import { Card } from '@radix-ui/themes';
import { useNavigate, useSearchParams } from '@remix-run/react';
import { FC } from 'react';

import { Spinner } from '~/components';
import { useSearchAddress } from '~/hooks';

export const Search: FC = () => {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const address = searchParams.get('q');
  const { addressData, loading } = useSearchAddress(address);

  if (address == null) {
    return '';
  }
  if (addressData == null) {
    return <Spinner size="2.5rem" />;
  }

  return (
    <div className="flex flex-wrap gap-8 max-w-[40rem] px-4 flex-justify-center flex-items-center">
      <h2>{address}</h2>

      {addressData.length === 0 && !loading && <>Account not found on any broker</>}

      {addressData.map((data) => (
        <Card
          className="cursor-pointer flex flex-col gap-4 p-4"
          onClick={() => {
            navigate({
              pathname: `/address/${data.address}`,
              search: `?broker_id=${data.broker_id}`
            });
          }}
          key={data.broker_id}
        >
          <div className="flex flex-col [&>*:first-child]:font-bold">
            <div>Broker ID:</div>
            <div>{data.broker_id}</div>
          </div>
          <div className="flex flex-col [&>*:first-child]:font-bold">
            <div>Account ID:</div>
            <div>
              {data.account_id.substring(0, 7)}...{data.account_id.substr(-7)}
            </div>
          </div>
          <div className="flex flex-col [&>*:first-child]:font-bold">
            <div>Address:</div>
            <div>
              {data.address.substring(0, 7)}...{data.address.substr(-7)}
            </div>
          </div>
        </Card>
      ))}
      {loading && <Spinner size="2.5rem" />}
    </div>
  );
};
export default Search;
