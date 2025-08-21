import { Dialog } from '@radix-ui/themes';
import { Link } from '@remix-run/react';
import { FC } from 'react';

import { Spinner } from '.';

import { useSearchAddress } from '~/hooks';
import { base64UrlSafeEncode } from '~/util';

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

interface BrokerSelectionModalProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  address: string;
}

export const BrokerSelectionModal: FC<BrokerSelectionModalProps> = ({
  open,
  onOpenChange,
  address
}) => {
  const { addressData, loading } = useSearchAddress(address);

  return (
    <Dialog.Root open={open} onOpenChange={onOpenChange}>
      <Dialog.Content className="card max-w-4xl w-full sm:w-[95vw] max-h-[90vh] mx-0 sm:mx-auto flex flex-col">
        <Dialog.Title className="text-lg sm:text-xl font-semibold text-white mb-4 break-all flex-shrink-0">
          Select Broker for {address}
        </Dialog.Title>

        {loading ? (
          <div className="flex justify-center py-8 flex-1">
            <Spinner size="2rem" />
          </div>
        ) : addressData && addressData.length > 0 ? (
          <div className="flex flex-col flex-1 min-h-0">
            <p className="text-sm sm:text-base text-gray-300 mb-4 flex-shrink-0">
              Found {addressData.length} account(s) across different brokers
            </p>
            <div className="grid grid-cols-1 lg:grid-cols-2 gap-3 overflow-y-auto flex-1">
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
                    to={(() => {
                      const isSol = data.address.match(/^[0-9a-zA-Z]{43,44}$/);
                      if (isSol) {
                        return `/address/${base64UrlSafeEncode(data.address)}?${searchParams.toString()}`;
                      } else {
                        return `/address/${data.address}?${searchParams.toString()}`;
                      }
                    })()}
                    className="no-underline text-white hover:text-white group"
                    onClick={() => onOpenChange(false)}
                  >
                    <div className="card p-3 sm:p-4 hover:bg-bg-tertiary transition-colors cursor-pointer text-left w-full">
                      <div className="space-y-2 sm:space-y-3">
                        <div className="flex items-center justify-between">
                          <span className="text-xs sm:text-sm text-gray-400">Broker ID</span>
                          <span className="text-xs px-2 py-1 bg-primary rounded-full text-white">
                            {data.broker_id}
                          </span>
                        </div>
                        <div className="text-base sm:text-lg font-semibold text-white">
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
                        <div className="space-y-2">
                          <span className="text-xs sm:text-sm text-gray-400">Account ID</span>
                          <div className="font-mono text-xs sm:text-sm bg-bg-primary p-2 rounded border border-border-primary break-all">
                            {data.account_id.substring(0, 7)}...{data.account_id.substr(-7)}
                          </div>
                        </div>
                        <div className="space-y-2 pt-2 border-t border-border-primary">
                          <div className="flex justify-between items-center">
                            <span className="text-xs sm:text-sm text-gray-400">Trading Volume</span>
                            <span className="text-xs sm:text-sm font-semibold text-white">
                              {formatNumber(data.perp_volume || 0)}
                            </span>
                          </div>
                          <div className="flex justify-between items-center">
                            <span className="text-xs sm:text-sm text-gray-400">Realized PnL</span>
                            <span
                              className={`text-xs sm:text-sm font-semibold ${pnlDisplay.color}`}
                            >
                              {pnlDisplay.text}
                            </span>
                          </div>
                        </div>
                      </div>
                    </div>
                  </Link>
                );
              })}
            </div>
          </div>
        ) : (
          <div className="text-center py-8 flex-1">
            <div className="text-lg sm:text-xl text-gray-300 mb-2">No Results Found</div>
            <div className="text-sm sm:text-base text-gray-400">
              No accounts found for this address
            </div>
          </div>
        )}

        <div className="flex justify-end gap-2 mt-6 flex-shrink-0">
          <button onClick={() => onOpenChange(false)} className="btn btn-secondary">
            Close
          </button>
        </div>
      </Dialog.Content>
    </Dialog.Root>
  );
};
