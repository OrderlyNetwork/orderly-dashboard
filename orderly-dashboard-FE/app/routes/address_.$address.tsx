import { CopyIcon } from '@radix-ui/react-icons';
import { Button, IconButton } from '@radix-ui/themes';
import { LoaderFunctionArgs } from '@remix-run/node';
import { json, useLoaderData, useSearchParams, useNavigate } from '@remix-run/react';
import dayjs from 'dayjs';
import { FC, useMemo, useState, useEffect } from 'react';
import useSWR from 'swr';
import { match } from 'ts-pattern';

import { useRenderColumns } from './address';

import { useAppState } from '~/App';
import { Spinner, Positions, EventsTable, BrokerSelectionModal } from '~/components';
import { ChainAddress, EventsParams, EventType } from '~/hooks';
import { base64UrlSafeEncode, base64UrlSafeDecode } from '~/util';

export function loader({ params }: LoaderFunctionArgs) {
  return json({ address: params.address ?? '' });
}

export const Address: FC = () => {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();

  const initialTab = searchParams.get('tab') as 'events' | 'positions' | null;
  const [activeTab, setActiveTab] = useState<'events' | 'positions'>(
    initialTab === 'events' || initialTab === 'positions' ? initialTab : 'events'
  );

  useEffect(() => {
    const currentTab = searchParams.get('tab') as 'events' | 'positions' | null;
    if (currentTab === 'events' || currentTab === 'positions') {
      setActiveTab(currentTab);
    } else {
      setActiveTab('events');
    }
  }, [searchParams]);

  const [eventType, setEventType] = useState<EventType | 'ALL'>('ALL');

  const [dateRange, setDateRange] = useState<[string | null, string | null]>([
    dayjs(new Date()).subtract(30, 'days').format('YYYY-MM-DD'),
    dayjs(new Date()).format('YYYY-MM-DD')
  ]);
  const [validDateRange, setValidDateRange] = useState<[string | null, string | null]>([
    dayjs(new Date()).subtract(30, 'days').format('YYYY-MM-DD'),
    dayjs(new Date()).format('YYYY-MM-DD')
  ]);

  const [aggregateTrades, setAggregateTrades] = useState<boolean>(true);

  const [showBrokerModal, setShowBrokerModal] = useState(false);

  const { address: rawAddress } = useLoaderData<typeof loader>();

  const address: ChainAddress = useMemo(() => {
    try {
      const decodedAddress = base64UrlSafeDecode(rawAddress);
      if (decodedAddress.match(/^[0-9a-zA-Z]{43,44}$/)) {
        return {
          address: decodedAddress,
          chain_namespace: 'sol'
        };
      }
    } catch {
      // If decoding fails, continue with raw address matching
    }

    if (rawAddress.match(/^0x[0-9a-fA-F]{40}$/)) {
      return {
        address: rawAddress,
        chain_namespace: 'evm'
      };
    }

    if (rawAddress.match(/^[0-9a-zA-Z]{43,44}$/)) {
      return {
        address: rawAddress,
        chain_namespace: 'sol'
      };
    }

    if (rawAddress.match(/^0x[0-9a-fA-F]{64}$/)) {
      return {
        address: rawAddress,
        chain_namespace: 'evm'
      };
    }

    throw new Error(`Could not match address ${rawAddress}`);
  }, [rawAddress]);

  const broker_id = searchParams.get('broker_id');
  const user_id = searchParams.get('user_id');

  const handleTabChange = (newTab: 'events' | 'positions') => {
    const newSearchParams = new URLSearchParams(searchParams);
    if (newTab === 'events') {
      newSearchParams.delete('tab');
    } else {
      newSearchParams.set('tab', newTab);
    }

    navigate({
      pathname: window.location.pathname,
      search: `?${newSearchParams.toString()}`
    });

    setActiveTab(newTab);
  };

  useEffect(() => {
    if (dateRange[0] && dateRange[1]) {
      setValidDateRange([dateRange[0], dateRange[1]]);
    }
  }, [dateRange]);

  const { evmApiUrl } = useAppState();

  const { data: accountIdData } = useSWR<{
    address: string;
    broker_id: string;
  }>(
    rawAddress.match(/^0x[0-9a-fA-F]{64}$/)
      ? `${evmApiUrl}/v1/public/account?account_id=${rawAddress}`
      : undefined,
    (url: string) =>
      fetch(url)
        .then((r) => r.json())
        .then((val) => {
          if (!val.success) {
            throw new Error(val.message || 'Failed to fetch account data');
          }
          return val.data;
        })
  );

  const { data: accountIdSubaccounts } = useSWR<{
    rows: Array<{
      user_id: number;
      account_id: string;
      broker_id: string;
      chain_type: string;
      user_type: string;
    }>;
  }>(
    rawAddress.match(/^0x[0-9a-fA-F]{64}$/) && accountIdData
      ? `${evmApiUrl}/v1/get_all_accounts?address=${accountIdData.address}&broker_id=${accountIdData.broker_id}`
      : undefined,
    (url: string) =>
      fetch(url)
        .then((r) => r.json())
        .then((val) => {
          if (!val.success) {
            throw new Error(val.message || 'Failed to fetch subaccounts');
          }
          return val.data;
        })
  );

  useEffect(() => {
    if (rawAddress.match(/^0x[0-9a-fA-F]{64}$/) && accountIdData && accountIdSubaccounts?.rows) {
      const matchingSubaccount = accountIdSubaccounts.rows.find(
        (account) => account.account_id === rawAddress
      );

      const searchParams = new URLSearchParams();
      searchParams.set('broker_id', accountIdData.broker_id);
      if (matchingSubaccount) {
        searchParams.set('user_id', matchingSubaccount.user_id.toString());
      }
      navigate({
        pathname: (() => {
          const isSol = accountIdData.address.match(/^[0-9a-zA-Z]{43,44}$/);
          if (isSol) {
            return `/address/${base64UrlSafeEncode(accountIdData.address)}`;
          } else {
            return `/address/${accountIdData.address}`;
          }
        })(),
        search: `?${searchParams.toString()}`
      });
    }
  }, [accountIdData, accountIdSubaccounts, rawAddress, navigate]);

  const { data: accountsData } = useSWR<{
    rows: Array<{
      user_id: number;
      account_id: string;
      broker_id: string;
      chain_type: string;
      user_type: string;
    }>;
  }>(
    broker_id != null
      ? `${evmApiUrl}/v1/get_all_accounts?address=${address.address}&broker_id=${broker_id}&chain_type=${address.chain_namespace.toUpperCase()}`
      : undefined,
    (url: string) =>
      fetch(url)
        .then((r) => r.json())
        .then((val) => {
          return val.data;
        })
  );

  const selectedAccount = useMemo(() => {
    if (!accountsData?.rows || accountsData.rows.length === 0) return null;

    if (user_id) {
      const targetUserId = parseInt(user_id);
      return (
        accountsData.rows.find((account) => account.user_id === targetUserId) ||
        accountsData.rows[0]
      );
    }

    return accountsData.rows[0];
  }, [accountsData, user_id]);

  const accountId = selectedAccount?.account_id;

  const AddressPositions = () => {
    if (!accountId) {
      return (
        <div className="flex justify-center py-12">
          <Spinner size="2.5rem" />
        </div>
      );
    }

    return (
      <div className="p-4 sm:p-6 max-w-full">
        <h2 className="text-2xl font-bold text-white mb-2 mx-2 md:mx-4">Positions History</h2>
        <p className="text-gray-300 mb-6 mx-2 md:mx-4">
          Historical positions for this account. Currently only supports sorting by holding value.
          Date information not yet available.
        </p>
        <Positions
          accountId={accountId}
          hideFilters={true}
          hideTitle={true}
          hideQuickActions={true}
        />
      </div>
    );
  };

  const eventsParams = useMemo(
    () =>
      broker_id != null && selectedAccount != null
        ? ({
            account_id: selectedAccount.account_id,
            event_type: match(eventType)
              .with('ALL', () => undefined)
              .with('LIQUIDATIONV2', () => 'LIQUIDATION')
              .with('ADLV2', () => 'ADL')
              .otherwise((value) => value) as EventType,
            from_time: validDateRange[0] ? dayjs(validDateRange[0]) : null,
            to_time: validDateRange[1] ? dayjs(validDateRange[1]).endOf('day') : null
          } satisfies EventsParams)
        : null,
    [broker_id, selectedAccount, eventType, validDateRange]
  );

  const {
    columns,
    events,
    error,
    isLoading,
    isLoadingMore,
    loadMore,
    hasMore,
    tradesCount,
    rawEventsCount
  } = useRenderColumns(eventsParams, eventType, setEventType, aggregateTrades);

  if (error) {
    return error.message ?? '';
  }

  if (accountsData?.rows && accountsData.rows.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center gap-4 p-8 text-center">
        <div className="text-xl font-semibold text-gray-300">Account Not Found</div>
        <div className="text-gray-500 max-w-md">
          No accounts found for address{' '}
          <code className="bg-gray-800 px-2 py-1 rounded text-sm">{address.address}</code>
          {broker_id && (
            <>
              {' '}
              with broker ID{' '}
              <code className="bg-gray-800 px-2 py-1 rounded text-sm">{broker_id}</code>
            </>
          )}
        </div>
        <Button onClick={() => navigate(`/search?q=${address.address}`)} className="mt-4">
          Search All Accounts for This Address
        </Button>
      </div>
    );
  }

  if (rawAddress.match(/^0x[0-9a-fA-F]{64}$/)) {
    return <Spinner size="2.5rem" />;
  }

  return (
    <div className="space-y-8 animate-fade-in flex flex-col items-center">
      {/* Header Section */}
      <div className="card p-4 sm:p-6 space-y-4 sm:space-y-6 max-w-2xl">
        <div className="space-y-3 sm:space-y-4">
          <h1 className="text-2xl sm:text-3xl font-bold bg-gradient-to-r from-white to-gray-300 bg-clip-text text-transparent">
            Address Details
          </h1>
          <div className="flex items-center gap-3 p-3 bg-bg-secondary rounded-lg border border-border-primary">
            <div className="flex-1 min-w-0">
              <div className="text-sm text-gray-400 mb-1">Address</div>
              <div className="font-mono text-white break-all">{address.address}</div>
            </div>
            <IconButton
              size="2"
              variant="soft"
              onClick={async () => {
                navigator.clipboard.writeText(address.address);
              }}
              className="flex-shrink-0"
            >
              <CopyIcon height="16" />
            </IconButton>
          </div>
        </div>

        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3 sm:gap-4">
          {accountId != null && (
            <div className="p-3 sm:p-4 bg-bg-secondary rounded-lg border border-border-primary">
              <div className="text-sm text-gray-400 mb-2">Account ID</div>
              <div className="flex items-center gap-2">
                <div className="font-mono text-white text-sm break-all">
                  {accountId.substring(0, 7)}...{accountId.substr(-7)}
                </div>
                <IconButton
                  size="1"
                  variant="soft"
                  onClick={async () => {
                    if (accountId == null) return;
                    navigator.clipboard.writeText(accountId);
                  }}
                >
                  <CopyIcon height="12" />
                </IconButton>
              </div>
            </div>
          )}

          <div className="p-3 sm:p-4 bg-bg-secondary rounded-lg border border-border-primary">
            <div className="text-sm text-gray-400 mb-2">Chain Namespace</div>
            <div className="text-white font-medium">
              {match(address.chain_namespace)
                .with('evm', () => 'EVM')
                .with('sol', () => 'Solana')
                .exhaustive()}
            </div>
          </div>

          {broker_id && (
            <div className="p-3 sm:p-4 bg-bg-secondary rounded-lg border border-border-primary">
              <div className="text-sm text-gray-400 mb-2">Current Broker</div>
              <div className="flex items-center justify-between">
                <div className="text-white font-medium">{broker_id}</div>
                <Button
                  size="1"
                  variant="soft"
                  onClick={() => setShowBrokerModal(true)}
                  className="text-xs"
                >
                  Change
                </Button>
              </div>
            </div>
          )}

          {accountsData?.rows && accountsData.rows.length > 1 && (
            <div className="p-3 sm:p-4 bg-bg-secondary rounded-lg border border-border-primary">
              <div className="text-sm text-gray-400 mb-2">Account Type</div>
              <select
                value={selectedAccount?.user_id || ''}
                onChange={(e) => {
                  const selectedUserId = e.target.value;
                  if (selectedUserId) {
                    const searchParams = new URLSearchParams();
                    searchParams.set('broker_id', broker_id!);
                    searchParams.set('user_id', selectedUserId);
                    navigate({
                      pathname: (() => {
                        const isSol = address.address.match(/^[0-9a-zA-Z]{43,44}$/);
                        if (isSol) {
                          return `/address/${base64UrlSafeEncode(address.address)}`;
                        } else {
                          return `/address/${address.address}`;
                        }
                      })(),
                      search: `?${searchParams.toString()}`
                    });
                  }
                }}
                className="w-full bg-bg-primary text-white border border-border-primary rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-primary focus:border-transparent transition-all duration-200"
              >
                {accountsData.rows.map((account) => (
                  <option key={account.user_id} value={account.user_id}>
                    {account.user_type === 'MAIN'
                      ? 'Main'
                      : account.user_type === 'SUB'
                        ? 'Sub'
                        : account.user_type}
                    {account.user_type === 'SUB' ? ` (ID: ${account.user_id})` : ''}
                  </option>
                ))}
              </select>
            </div>
          )}
        </div>

        {/* Change Broker Button - only show if no broker is selected */}
        {!broker_id && (
          <div className="flex justify-center">
            <Button onClick={() => setShowBrokerModal(true)} className="btn btn-primary">
              Select Broker ID
            </Button>
          </div>
        )}
      </div>

      {/* Tab Navigation */}
      <div className="flex justify-center mb-8">
        <div className="flex gap-2">
          <button
            onClick={() => handleTabChange('events')}
            className={`btn ${activeTab === 'events' ? 'btn-primary' : 'btn-secondary'}`}
          >
            Events
          </button>
          <button
            onClick={() => handleTabChange('positions')}
            className={`btn ${activeTab === 'positions' ? 'btn-primary' : 'btn-secondary'}`}
          >
            Positions
          </button>
        </div>
      </div>

      {/* Tab Content */}
      {activeTab === 'events' ? (
        <EventsTable
          events={events}
          columns={columns}
          isLoading={isLoading}
          isLoadingMore={isLoadingMore}
          hasMore={hasMore}
          tradesCount={tradesCount}
          loadMore={loadMore}
          eventType={eventType}
          setEventType={setEventType}
          dateRange={dateRange}
          setDateRange={setDateRange}
          aggregateTrades={aggregateTrades}
          setAggregateTrades={setAggregateTrades}
          rawEventsCount={rawEventsCount}
        />
      ) : (
        <AddressPositions />
      )}

      {/* Broker Selection Modal */}
      <BrokerSelectionModal
        open={showBrokerModal}
        onOpenChange={setShowBrokerModal}
        address={address.address}
      />
    </div>
  );
};
export default Address;
