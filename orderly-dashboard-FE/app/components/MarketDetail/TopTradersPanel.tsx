import { FC, useState } from 'react';

import { AddressLink } from '~/components/analytics/shared/AddressLink';
import { fmtUsd } from '~/components/analytics/shared/formatters';
import { TableSkeleton, Empty } from '~/components/analytics/shared/primitives';
import { WidgetShareButton } from '~/components/analytics/widgets/WidgetShareButton';
import { useIsEmbed } from '~/hooks/useIsEmbed';
import { useTopAddresses, type TopAddressesSortOption } from '~/hooks/usePublicInfo';
import { getBaseToken } from '~/hooks/useSymbols';

export type TopTradersPanelProps = {
  symbol: string;
};

const SORT_OPTIONS: { value: TopAddressesSortOption; label: string }[] = [
  { value: 'notional', label: 'Notional' },
  { value: 'pnl_7d', label: 'PnL 7d' },
  { value: 'pnl_30d', label: 'PnL 30d' },
  { value: 'volume_24h', label: 'Vol 24h' }
];

export const TopTradersPanel: FC<TopTradersPanelProps> = ({ symbol }) => {
  const isEmbed = useIsEmbed();
  const [sortBy, setSortBy] = useState<TopAddressesSortOption>('notional');

  const { data, isLoading } = useTopAddresses({
    symbol,
    sort_by: sortBy,
    limit: 30
  });

  const rows = (data?.rows ?? []).filter((row) => row.broker_id !== 'orderly').slice(0, 15);

  const thClass =
    'py-2 px-3 text-xs font-medium text-gray-500 uppercase tracking-wider sticky top-0';
  const thStyle = { background: 'rgba(20,15,35,.95)' };

  const title = `Top Traders${isEmbed ? ` — ${getBaseToken(symbol)}-PERP` : ''}`;

  return (
    <div
      className="rounded-2xl overflow-hidden flex flex-col"
      style={{
        background: 'rgba(20,15,35,.9)',
        border: '1px solid rgba(156,117,255,0.15)',
        maxHeight: 600
      }}
    >
      <div
        className="flex flex-col sm:flex-row sm:items-center sm:justify-between border-b px-5 py-4 gap-3"
        style={{ borderBottomColor: 'rgba(156,117,255,0.08)' }}
      >
        <div>
          <div
            className="text-lg font-semibold text-white"
            style={{
              fontFamily: "'Atyp BL Text', sans-serif",
              fontFeatureSettings: "'ss02' 1, 'ss03' 1, 'ss05' 1"
            }}
          >
            {title}
          </div>
          <div className="text-[13px] mt-0.5 text-[rgba(255,255,255,0.35)]">
            Leading traders for this symbol
          </div>
        </div>
        <div className="flex items-center gap-2">
          <div className="flex gap-1 rounded-lg p-1" style={{ background: '#130E1D' }}>
            {SORT_OPTIONS.map((opt) => (
              <button
                key={opt.value}
                onClick={() => setSortBy(opt.value)}
                className="px-2.5 py-1 rounded-md border-none cursor-pointer text-xs transition-all duration-150"
                style={{
                  background: sortBy === opt.value ? '#6700CE' : 'transparent',
                  color: sortBy === opt.value ? '#E9DEFF' : 'rgba(255,255,255,0.45)',
                  fontWeight: sortBy === opt.value ? 600 : 400
                }}
              >
                {opt.label}
              </button>
            ))}
          </div>
          <WidgetShareButton widgetId="market-top-traders" title={title} symbol={symbol} />
        </div>
      </div>

      <div className="overflow-auto flex-1" style={{ minHeight: 0 }}>
        {isLoading && rows.length === 0 ? (
          <div className="px-4 py-4">
            <TableSkeleton rows={8} />
          </div>
        ) : rows.length > 0 ? (
          <table className="w-full">
            <thead>
              <tr>
                <th className={`text-left ${thClass}`} style={thStyle}>
                  #
                </th>
                <th className={`text-left ${thClass}`} style={thStyle}>
                  Address
                </th>
                <th className={`text-right ${thClass}`} style={thStyle}>
                  Notional
                </th>
                <th className={`text-right ${thClass}`} style={thStyle}>
                  PnL 24h
                </th>
                <th className={`text-right ${thClass}`} style={thStyle}>
                  PnL 7d
                </th>
                <th className={`text-right ${thClass}`} style={thStyle}>
                  PnL 30d
                </th>
                <th className={`text-right ${thClass}`} style={thStyle}>
                  Vol 24h
                </th>
                <th className={`text-right ${thClass}`} style={thStyle}>
                  Win Rate
                </th>
              </tr>
            </thead>
            <tbody>
              {rows.map((row, i) => {
                const notional = parseFloat(row.total_notional || '0');
                const pnl24h = parseFloat(row.pnl_24h || '0');
                const pnl7d = parseFloat(row.pnl_7d || '0');
                const pnl30d = parseFloat(row.pnl_30d || '0');
                const vol24h = parseFloat(row.volume_24h || '0');
                const winRate = row.win_rate_24h;

                return (
                  <tr
                    key={`${row.address}-${row.broker_id}`}
                    className="border-b border-border-primary hover:bg-bg-tertiary transition-colors"
                  >
                    <td className="py-2.5 px-3 text-xs text-gray-500">{i + 1}</td>
                    <td className="py-2.5 px-3">
                      <AddressLink
                        address={row.address}
                        brokerId={row.broker_id}
                        className="text-xs font-mono text-[#D4B2FF] hover:text-white transition-colors no-underline"
                      >
                        {row.address.slice(0, 6)}...{row.address.slice(-4)}
                      </AddressLink>
                    </td>
                    <td className="py-2.5 px-3 text-right text-xs font-mono text-white">
                      {fmtUsd(notional)}
                    </td>
                    <td
                      className="py-2.5 px-3 text-right text-xs font-mono"
                      style={{ color: pnl24h >= 0 ? '#00dea3' : '#FF6390' }}
                    >
                      {fmtUsd(pnl24h)}
                    </td>
                    <td
                      className="py-2.5 px-3 text-right text-xs font-mono"
                      style={{ color: pnl7d >= 0 ? '#00dea3' : '#FF6390' }}
                    >
                      {fmtUsd(pnl7d)}
                    </td>
                    <td
                      className="py-2.5 px-3 text-right text-xs font-mono"
                      style={{ color: pnl30d >= 0 ? '#00dea3' : '#FF6390' }}
                    >
                      {fmtUsd(pnl30d)}
                    </td>
                    <td className="py-2.5 px-3 text-right text-xs font-mono text-gray-300">
                      {fmtUsd(vol24h)}
                    </td>
                    <td className="py-2.5 px-3 text-right text-xs font-mono text-gray-300">
                      {winRate != null ? `${(winRate * 100).toFixed(0)}%` : '—'}
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        ) : (
          <div className="flex items-center justify-center flex-1">
            <Empty msg="No trader data available" />
          </div>
        )}
      </div>
    </div>
  );
};
