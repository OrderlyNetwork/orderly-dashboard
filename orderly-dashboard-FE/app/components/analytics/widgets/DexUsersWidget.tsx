import { FC } from 'react';

import { fmtNum, fmtPct } from '../shared/formatters';
import { Empty, Skeleton, TD, TH_STICKY, tdSticky } from '../shared/primitives';

import { useDexUsers } from '~/hooks/useOrderlyMetrics';

const HEADERS = ['Broker', 'DAU', 'DoD%', 'WAU', 'WoW%', 'MAU', 'MoM%', 'Total', 'New 30d'];

export const DexUsersWidget: FC = () => {
  const { data, isLoading, error } = useDexUsers();
  const brokers = (data?.data ?? []).sort((a, b) => (b.total_users ?? 0) - (a.total_users ?? 0));

  return (
    <div className="overflow-x-auto max-h-[340px] overflow-y-auto">
      {isLoading ? (
        <div className="p-5">
          <Skeleton height={160} />
        </div>
      ) : error ? (
        <div className="p-6">
          <Empty msg="Failed to load" />
        </div>
      ) : brokers.length === 0 ? (
        <div className="p-6">
          <Empty msg="No data" />
        </div>
      ) : (
        <table className="w-full border-collapse text-xs">
          <thead>
            <tr>
              {HEADERS.map((h, idx) => (
                <th
                  key={h}
                  style={
                    idx === 0
                      ? TH_STICKY
                      : {
                          ...TH_STICKY,
                          position: undefined,
                          left: undefined,
                          zIndex: undefined,
                          background: undefined
                        }
                  }
                >
                  {h}
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {brokers.map((b, i) => (
              <tr key={i}>
                <td style={{ ...tdSticky(i), color: '#9C75FF', fontWeight: 600 }}>
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
