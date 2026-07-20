import { FC, useMemo } from 'react';

import { fmtCompact } from '../shared/formatters';
import { Empty, Skeleton, StatCard, TD, TH } from '../shared/primitives';

import { useInsuranceFund } from '~/hooks/useInsuranceFund';
import { useMarketSummary } from '~/hooks/usePublicInfo';
import { formatPriceByTick } from '~/utils/format';

const prettySymbol = (s: string): string => s.replace('PERP_', '').replace('_USDC', '');

export const InsuranceFundWidget: FC = () => {
  const { data, isLoading, error } = useInsuranceFund();
  const { data: marketSummary } = useMarketSummary();

  const quoteTickFor = useMemo(() => {
    return (symbol: string): number | null => {
      const tick = marketSummary?.markets.find((m) => m.symbol === symbol)?.quote_tick;
      const parsed = tick != null ? parseFloat(tick) : NaN;
      return Number.isFinite(parsed) ? parsed : null;
    };
  }, [marketSummary]);

  if (isLoading) return <Skeleton height={120} />;
  if (error || !data) return <Empty msg={error ? 'Failed to load' : 'No data'} />;

  const pnlPositive = data.total_pnl_24_h >= 0;

  const positions = (data.rows ?? []).filter((r) => Math.abs(r.position_qty) > 0);

  return (
    <div className="flex flex-col gap-5">
      <div className="dash-grid-sm">
        <StatCard label="Fund Balance" value={fmtCompact(data.balance)} color="#6700CE" selected />
        <StatCard
          label="Free Collateral"
          value={fmtCompact(data.free_collateral)}
          color="#00dea3"
          selected
        />
        <StatCard
          label="24h PnL"
          value={fmtCompact(data.total_pnl_24_h)}
          color={pnlPositive ? '#00dea3' : '#FF6390'}
          selected
        />
        <StatCard label="Margin Ratio" value={`${data.margin_ratio}x`} color="#9C75FF" selected />
      </div>

      {positions.length > 0 ? (
        <div className="overflow-x-auto">
          <table style={{ width: '100%', borderCollapse: 'collapse' }}>
            <thead>
              <tr>
                <th style={TH}>Symbol</th>
                <th style={{ ...TH, textAlign: 'right' }}>Qty</th>
                <th style={{ ...TH, textAlign: 'right' }}>Mark Price</th>
                <th style={{ ...TH, textAlign: 'right' }}>Avg Entry</th>
                <th style={{ ...TH, textAlign: 'right' }}>PnL 24h</th>
                <th style={{ ...TH, textAlign: 'right' }}>Fee 24h</th>
              </tr>
            </thead>
            <tbody>
              {positions.map((p) => {
                const posPnlPositive = p.pnl_24_h >= 0;
                return (
                  <tr key={p.symbol}>
                    <td style={TD}>{prettySymbol(p.symbol)}</td>
                    <td style={{ ...TD, textAlign: 'right' }}>{p.position_qty}</td>
                    <td style={{ ...TD, textAlign: 'right' }}>
                      {formatPriceByTick(p.mark_price, quoteTickFor(p.symbol))}
                    </td>
                    <td style={{ ...TD, textAlign: 'right' }}>
                      {formatPriceByTick(p.average_open_price, quoteTickFor(p.symbol))}
                    </td>
                    <td
                      style={{
                        ...TD,
                        textAlign: 'right',
                        color: posPnlPositive ? '#00dea3' : '#FF6390'
                      }}
                    >
                      {fmtCompact(p.pnl_24_h)}
                    </td>
                    <td style={{ ...TD, textAlign: 'right' }}>{fmtCompact(p.fee_24_h)}</td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      ) : (
        <div className="text-[13px] text-[rgba(255,255,255,0.25)]">
          No open positions held by the fund.
        </div>
      )}
    </div>
  );
};
