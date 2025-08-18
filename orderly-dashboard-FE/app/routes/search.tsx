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
    return { text: `+${formatted}`, color: 'text-success' };
  } else if (value < 0) {
    return { text: `-${formatted}`, color: 'text-error' };
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
    return (
      <div className="flex justify-center py-12">
        <Spinner size="2.5rem" />
      </div>
    );
  }

  return (
    <div className="space-y-8 animate-fade-in">
      {/* Header Section */}
      <div className="text-center space-y-4">
        <div className="card max-w-2xl mx-auto p-6">
          <h2 className="text-2xl font-bold text-white mb-2 break-all">{address}</h2>
          <div className="flex items-center justify-center gap-2 text-sm text-gray-300">
            <div
              className={`w-2 h-2 rounded-full ${
                chainNamespace === 'evm' ? 'bg-success' : 'bg-info'
              }`}
            ></div>
            <span className="uppercase">{chainNamespace} Address</span>
          </div>
          <p className="text-sm text-gray-400 mt-2">Stats shown are for the last 90 days</p>
        </div>
      </div>

      {/* Results Section */}
      {addressData.length === 0 && !loading && (
        <div className="card text-center py-12">
          <div className="text-xl text-gray-300 mb-2">No Results Found</div>
          <div className="text-gray-400">Account not found on any broker</div>
        </div>
      )}

      {addressData.length > 0 && (
        <div className="space-y-6">
          <div className="text-center">
            <h3 className="text-xl font-semibold text-white mb-2">Trading Accounts</h3>
            <p className="text-gray-300">
              Found {addressData.length} account(s) across different brokers
            </p>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
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
                  className="no-underline text-white hover:text-white group"
                >
                  <div className="card hover:scale-105 transition-all duration-300 cursor-pointer">
                    <div className="space-y-4">
                      {/* Broker Info */}
                      <div className="space-y-2">
                        <div className="flex items-center justify-between">
                          <span className="text-sm text-gray-400">Broker ID</span>
                          <span className="text-xs px-2 py-1 bg-primary rounded-full text-white">
                            {data.broker_id}
                          </span>
                        </div>
                        <div className="text-lg font-semibold text-white">
                          {data.broker_id}
                          {(() => {
                            const accountsForThisBroker = addressData.filter(
                              (account) => account.broker_id === data.broker_id
                            );
                            return accountsForThisBroker.length > 1 && data.user_type ? (
                              <span className="text-sm text-gray-400 ml-2 font-normal">
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

                      {/* Account ID */}
                      <div className="space-y-2">
                        <span className="text-sm text-gray-400">Account ID</span>
                        <div className="font-mono text-sm bg-bg-primary p-2 rounded border border-border-primary">
                          {data.account_id.substring(0, 7)}...{data.account_id.substr(-7)}
                        </div>
                      </div>

                      {/* Stats */}
                      <div className="space-y-3 pt-4 border-t border-border-primary">
                        <div className="flex justify-between items-center">
                          <span className="text-sm text-gray-400">Trading Volume</span>
                          <span className="text-lg font-semibold text-white">
                            {formatNumber(data.perp_volume)}
                          </span>
                        </div>
                        <div className="flex justify-between items-center">
                          <span className="text-sm text-gray-400">Realized PnL</span>
                          <span className={`text-lg font-semibold ${pnlDisplay.color}`}>
                            {pnlDisplay.text}
                          </span>
                        </div>
                      </div>

                      {/* View Details Button */}
                      <div className="pt-2">
                        <div className="btn btn-primary w-full text-center group-hover:scale-105 transition-transform duration-200">
                          View Details
                        </div>
                      </div>
                    </div>
                  </div>
                </Link>
              );
            })}
          </div>
        </div>
      )}

      {loading && (
        <div className="flex justify-center py-12">
          <Spinner size="2.5rem" />
        </div>
      )}
    </div>
  );
};
export default Search;
