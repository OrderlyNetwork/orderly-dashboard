import { FC, useEffect, useMemo, useState } from 'react';

import { AccountCell } from '~/components/AccountCell';
import { fmtNum, fmtUsd } from '~/components/analytics/shared/formatters';
import { Empty, TableSkeleton, TH, TD } from '~/components/analytics/shared/primitives';
import { useBrokers, useTokens } from '~/hooks';
import { useDepositRank, useWithdrawRank, type FlowsDirection } from '~/hooks/useFlowsRank';
import { useMarketSummary, type MarketSummaryMarket } from '~/hooks/usePublicInfo';

const DAYS_OPTIONS = [
  { label: '1D', value: 1 },
  { label: '7D', value: 7 },
  { label: '30D', value: 30 }
] as const;

const SIZE = 10;
const DEFAULT_TOKEN = 'USDC';
const STABLES = new Set(['USDC', 'USDT', 'USDE', 'DAI', 'USD']);

const selectStyle: React.CSSProperties = {
  background: '#130E1D',
  border: '1px solid rgba(156,117,255,0.18)',
  color: '#ffffff',
  borderRadius: 6,
  padding: '4px 8px',
  fontSize: 12,
  outline: 'none',
  cursor: 'pointer'
};

const pillStyle = (active: boolean): React.CSSProperties => ({
  background: active ? '#6700CE' : 'transparent',
  color: active ? '#E9DEFF' : 'rgba(255,255,255,0.45)',
  fontWeight: active ? 600 : 400
});

function tokenToUsdPrice(token: string, markets: MarketSummaryMarket[]): number | null {
  const candidates = [`PERP_${token}_USDC`];
  if (token.startsWith('W') && token.length > 1) candidates.push(`PERP_${token.slice(1)}_USDC`);
  for (const sym of candidates) {
    const m = markets.find((x) => x.symbol === sym);
    const p = m ? parseFloat(m.mark_price) : NaN;
    if (isFinite(p) && p > 0) return p;
  }
  return null;
}

function formatAmount(amount: string, token: string, price: number | null): React.ReactNode {
  const n = parseFloat(amount);
  if (!isFinite(n)) return '—';
  if (STABLES.has(token)) return fmtUsd(n);
  const raw = `${fmtNum(n)} ${token}`;
  if (price != null) {
    return (
      <span>
        {raw} <span style={{ color: 'rgba(255,255,255,0.4)' }}>· {fmtUsd(n * price)}</span>
      </span>
    );
  }
  return raw;
}

const DirectionToggle: FC<{ value: FlowsDirection; onChange: (d: FlowsDirection) => void }> = ({
  value,
  onChange
}) => (
  <div className="flex gap-1 rounded-lg p-1" style={{ background: '#130E1D' }}>
    {(['deposit', 'withdraw'] as const).map((d) => (
      <button
        key={d}
        type="button"
        onClick={() => onChange(d)}
        className="px-3 py-1 rounded-md border-none cursor-pointer text-[12px] transition-all duration-150"
        style={pillStyle(value === d)}
      >
        {d === 'deposit' ? 'Deposits' : 'Withdrawals'}
      </button>
    ))}
  </div>
);

const DaysToggle: FC<{ value: number; onChange: (d: number) => void }> = ({ value, onChange }) => (
  <div className="flex gap-1 rounded-lg p-1" style={{ background: '#130E1D' }}>
    {DAYS_OPTIONS.map((o) => (
      <button
        key={o.value}
        type="button"
        onClick={() => onChange(o.value)}
        className="px-2.5 py-1 rounded-md border-none cursor-pointer text-[12px] transition-all duration-150"
        style={pillStyle(value === o.value)}
      >
        {o.label}
      </button>
    ))}
  </div>
);

export const TopFlowsWidget: FC<{
  initialToken?: string;
  initialDirection?: FlowsDirection;
  initialDays?: number;
}> = ({ initialToken = DEFAULT_TOKEN, initialDirection = 'deposit', initialDays = 7 }) => {
  const [direction, setDirection] = useState<FlowsDirection>(initialDirection);
  const [token, setToken] = useState<string>(initialToken);
  const [days, setDays] = useState<number>(initialDays);

  useEffect(() => {
    setToken(initialToken);
  }, [initialToken]);
  useEffect(() => {
    setDirection(initialDirection);
  }, [initialDirection]);
  useEffect(() => {
    setDays(initialDays);
  }, [initialDays]);

  const params = useMemo(() => ({ days, size: SIZE, token }), [days, token]);
  const deposit = useDepositRank(params);
  const withdraw = useWithdrawRank(params);
  const { data: brokers } = useBrokers();
  const tokens = useTokens();
  const { data: marketSummary } = useMarketSummary();

  const price = useMemo(
    () => (token && marketSummary?.markets ? tokenToUsdPrice(token, marketSummary.markets) : null),
    [token, marketSummary]
  );

  const { data, isLoading, error } = direction === 'deposit' ? deposit : withdraw;

  const tokenOptions = useMemo(() => {
    const list = (tokens ?? []).map((t) => t.token).filter((t) => t && t !== 'ALL');
    const unique = Array.from(new Set(list)).sort();
    if (!unique.includes(DEFAULT_TOKEN)) unique.unshift(DEFAULT_TOKEN);
    return unique;
  }, [tokens]);

  return (
    <div>
      <div className="flex items-center justify-between mb-3 gap-2 flex-wrap">
        <DirectionToggle value={direction} onChange={setDirection} />
        <div className="flex items-center gap-2">
          <select value={token} onChange={(e) => setToken(e.target.value)} style={selectStyle}>
            {tokenOptions.map((t) => (
              <option key={t} value={t}>
                {t}
              </option>
            ))}
          </select>
          <DaysToggle value={days} onChange={setDays} />
        </div>
      </div>

      {isLoading ? (
        <TableSkeleton rows={SIZE} height={420} />
      ) : error || !data || data.length === 0 ? (
        <Empty msg={error ? 'Failed to load' : 'No data'} />
      ) : (
        <div className="overflow-auto w-full" style={{ maxHeight: 440, scrollbarWidth: 'thin' }}>
          <table className="w-full border-collapse text-[13px]">
            <thead>
              <tr>
                <th style={TH}>#</th>
                <th style={TH}>Account</th>
                <th style={{ ...TH, textAlign: 'right' }}>Amount</th>
              </tr>
            </thead>
            <tbody>
              {data.map((row, i) => (
                <tr key={`${row.account_id}-${i}`}>
                  <td style={{ ...TD, color: 'rgba(255,255,255,0.4)' }}>{i + 1}</td>
                  <td style={TD}>
                    <AccountCell accountId={row.account_id} brokers={brokers} />
                  </td>
                  <td style={{ ...TD, textAlign: 'right', color: '#fff' }}>
                    {formatAmount(row.amount, token, price)}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
};
