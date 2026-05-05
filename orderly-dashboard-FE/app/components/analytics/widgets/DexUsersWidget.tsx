import { FC } from 'react';

import { fmtNum, fmtPct } from '../shared/formatters';
import { Empty, Skeleton, TD, TH } from '../shared/primitives';

import { useDexUsers } from '~/hooks/useOrderlyMetrics';

export const DexUsersWidget: FC = () => {
  const { data, isLoading, error } = useDexUsers();
  const brokers = (data?.data ?? []).sort((a, b) => (b.total_users ?? 0) - (a.total_users ?? 0));

  return (
    <div style={{ overflowX: 'auto', maxHeight: 340, overflowY: 'auto' }}>
      {isLoading ? (
        <div style={{ padding: 20 }}>
          <Skeleton height={160} />
        </div>
      ) : error ? (
        <div style={{ padding: 24 }}>
          <Empty msg="Failed to load" />
        </div>
      ) : brokers.length === 0 ? (
        <div style={{ padding: 24 }}>
          <Empty msg="No data" />
        </div>
      ) : (
        <table style={{ width: '100%', borderCollapse: 'collapse', fontSize: 12 }}>
          <thead>
            <tr>
              {['Broker', 'DAU', 'DoD%', 'WAU', 'WoW%', 'MAU', 'MoM%', 'Total', 'New 30d'].map(
                (h) => (
                  <th key={h} style={TH}>
                    {h}
                  </th>
                )
              )}
            </tr>
          </thead>
          <tbody>
            {brokers.map((b, i) => (
              <tr key={i}>
                <td style={{ ...TD, color: '#9C75FF', fontWeight: 600 }}>
                  {b.broker_name ?? b.broker_id ?? '—'}
                </td>
                <td style={{ ...TD, color: '#fff' }}>{fmtNum(b.dau)}</td>
                <td style={{ ...TD, color: (b.dau_dod_pct ?? 0) >= 0 ? '#34d399' : '#f87171' }}>
                  {fmtPct(b.dau_dod_pct)}
                </td>
                <td style={{ ...TD, color: '#fff' }}>{fmtNum(b.wau)}</td>
                <td style={{ ...TD, color: (b.wau_wow_pct ?? 0) >= 0 ? '#34d399' : '#f87171' }}>
                  {fmtPct(b.wau_wow_pct)}
                </td>
                <td style={{ ...TD, color: '#fff' }}>{fmtNum(b.mau)}</td>
                <td style={{ ...TD, color: (b.mau_mom_pct ?? 0) >= 0 ? '#34d399' : '#f87171' }}>
                  {fmtPct(b.mau_mom_pct)}
                </td>
                <td style={{ ...TD, color: 'rgba(255,255,255,0.7)' }}>{fmtNum(b.total_users)}</td>
                <td style={{ ...TD, color: '#60a5fa' }}>{fmtNum(b.new_users_30d)}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </div>
  );
};
