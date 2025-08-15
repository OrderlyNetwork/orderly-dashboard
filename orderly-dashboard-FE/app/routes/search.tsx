import { Card } from '@radix-ui/themes';
import { Link, useSearchParams } from '@remix-run/react';
import { FC } from 'react';
import { match } from 'ts-pattern';

import { Spinner } from '~/components';
import { ChainNamespace, useSearchAddress } from '~/hooks';
import { base64UrlSafeDecode } from '~/util';

const formatNumber = (value?: number): string => {
  if (value == null) return '-';
  if (value >= 1_000_000) {
    return `$${(value / 1_000_000).toFixed(1)}M`;
  }
  if (value >= 1_000) {
    return `$${(value / 1_000).toFixed(1)}K`;
  }
  return `$${value.toFixed(2)}`;
};

const formatPnL = (value?: number): { text: string; color: string } => {
  if (value == null) return { text: '-', color: 'text-gray-400' };
  const formatted = formatNumber(Math.abs(value));
  if (value > 0) {
    return { text: `+${formatted}`, color: 'text-green-400' };
  } else if (value < 0) {
    return { text: `-${formatted}`, color: 'text-red-400' };
  }
  return { text: formatted, color: 'text-gray-400' };
};

export const Search: FC = () => {
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
    <div className="flex flex-col gap-4 sm:gap-8 max-w-full lg:max-w-[60rem] px-2 sm:px-4 flex-justify-center flex-items-center">
      <h2 className="text-lg sm:text-xl break-all text-center">{address}</h2>
      <p className="text-sm text-gray-400 text-center">Stats shown are for the last 90 days</p>

      {addressData.length === 0 && !loading && (
        <div className="text-center text-sm sm:text-base">Account not found on any broker</div>
      )}

      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 w-full">
        {addressData.map((data) => {
          const searchParams = new URLSearchParams();
          searchParams.set('broker_id', data.broker_id);
          if (data.user_id) {
            searchParams.set('user_id', data.user_id.toString());
          }

          const pnlDisplay = formatPnL(data.realized_pnl);

          return (
            <Link
              key={`${data.broker_id}-${data.account_id}`}
              to={`/address/${data.address}?${searchParams.toString()}`}
              className="no-underline text-white hover:text-white"
            >
              <Card className="cursor-pointer flex flex-col gap-3 sm:gap-4 p-3 sm:p-4 hover:bg-gray-800 transition-colors">
                <div className="flex flex-col [&>*:first-child]:font-bold text-sm sm:text-base">
                  <div>Broker ID:</div>
                  <div className="break-all">
                    {data.broker_id}
                    {(() => {
                      const accountsForThisBroker = addressData.filter(
                        (account) => account.broker_id === data.broker_id
                      );
                      return accountsForThisBroker.length > 1 && data.user_type ? (
                        <span className="text-gray-400 ml-2">
                          (
                          {data.user_type === 'MAIN'
                            ? 'main'
                            : data.user_type === 'SUB'
                              ? 'sub'
                              : data.user_type.toLowerCase()}
                          )
                        </span>
                      ) : null;
                    })()}
                  </div>
                </div>
                <div className="flex flex-col [&>*:first-child]:font-bold text-sm sm:text-base">
                  <div>Account ID:</div>
                  <div className="break-all">
                    {data.account_id.substring(0, 7)}...{data.account_id.substr(-7)}
                  </div>
                </div>

                {/* Volume and PnL Stats */}
                <div className="flex flex-col gap-2 pt-2 border-t border-gray-700">
                  <div className="flex justify-between items-center">
                    <span className="text-xs text-gray-400">Volume:</span>
                    <span className="text-sm font-medium">{formatNumber(data.perp_volume)}</span>
                  </div>
                  <div className="flex justify-between items-center">
                    <span className="text-xs text-gray-400">PnL:</span>
                    <span className={`text-sm font-medium ${pnlDisplay.color}`}>
                      {pnlDisplay.text}
                    </span>
                  </div>
                </div>
              </Card>
            </Link>
          );
        })}
      </div>
      {loading && <Spinner size="2.5rem" />}
    </div>
  );
};
export default Search;
