import { Card } from '@radix-ui/themes';
import { useNavigate, useSearchParams } from '@remix-run/react';
import { FC } from 'react';

import { Spinner } from '~/components';
import { useEvents } from '~/hooks';

export const Search: FC = () => {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  // TODO allow fuzzy search
  const address = searchParams.get('q');
  const broker_id = searchParams.get('broker_id');
  const {
    data: events,
    error,
    isLoading
  } = useEvents(
    address != null && broker_id != null
      ? {
          address,
          broker_id
        }
      : null
  );

  if (address == null) {
    return '';
  }
  if (error) {
    return error.message ?? '';
  }

  if (!events || isLoading) {
    return <Spinner size="2.5rem" />;
  }

  return (
    <div className="flex flex-col">
      <Card
        className="cursor-pointer"
        onClick={() => {
          navigate(`/address/${address}`);
        }}
      >
        {address.substring(0, 4)}...{address.substr(-4)}
      </Card>
    </div>
  );
};
export default Search;
