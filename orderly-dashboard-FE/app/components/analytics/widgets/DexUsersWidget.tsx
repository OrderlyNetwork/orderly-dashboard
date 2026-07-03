import { FC, useState } from 'react';

import { fmtNum, fmtPct } from '../shared/formatters';
import { Empty, TableSkeleton, TD, TH_STICKY, tdSticky } from '../shared/primitives';

import { useDexUsers } from '~/hooks/useOrderlyMetrics';

type SortKey =
  | 'name'
  | 'dau'
  | 'dau_dod_pct'
  | 'wau'
  | 'wau_wow_pct'
  | 'mau'
  | 'mau_mom_pct'
  | 'total_users'
  | 'new_users_30d';

const COLUMNS: { label: string; key: SortKey }[] = [
  { label: 'Broker', key: 'name' },
  { label: 'DAU', key: 'dau' },
  { label: 'DoD%', key: 'dau_dod_pct' },
  { label: 'WAU', key: 'wau' },
  { label: 'WoW%', key: 'wau_wow_pct' },
  { label: 'MAU', key: 'mau' },
  { label: 'MoM%', key: 'mau_mom_pct' },
  { label: 'Total', key: 'total_users' },
  { label: 'New 30d', key: 'new_users_30d' }
];

export const DexUsersWidget: FC<{
  search?: string;
  onSearchChange?: (s: string) => void;
}> = ({ search = '' }) => {
  const { data, isLoading, error } = useDexUsers();
  const [sortKey, setSortKey] = useState<SortKey>('total_users');
  const [sortDir, setSortDir] = useState<'asc' | 'desc'>('desc');

  const raw = data?.data ?? [];

  const brokers = raw
    .filter((b) => {
      if (!search) return true;
      const name = (b.broker_name ?? b.broker_id ?? '').toLowerCase();
      return name.includes(search.toLowerCase());
    })
    .slice()
    .sort((a, b) => {
      const av =
        sortKey === 'name'
          ? (a.broker_name ?? a.broker_id ?? '')
          : (((a as Record<string, unknown>)[sortKey] as number) ?? 0);
      const bv =
        sortKey === 'name'
          ? (b.broker_name ?? b.broker_id ?? '')
          : (((b as Record<string, unknown>)[sortKey] as number) ?? 0);
      if (av < bv) return sortDir === 'asc' ? -1 : 1;
      if (av > bv) return sortDir === 'asc' ? 1 : -1;
      return 0;
    });

  const handleSort = (key: SortKey) => {
    if (key === sortKey) setSortDir((d) => (d === 'asc' ? 'desc' : 'asc'));
    else {
      setSortKey(key);
      setSortDir('desc');
    }
  };

  return (
    <div>
      <div className="overflow-x-auto max-h-[320px] overflow-y-auto thin-scrollbar">
        {isLoading ? (
          <div className="p-5">
            <TableSkeleton height={160} />
          </div>
        ) : error ? (
          <div className="p-6">
            <Empty msg="Failed to load" />
          </div>
        ) : brokers.length === 0 ? (
          <div className="p-6">
            <Empty msg={raw.length === 0 ? 'No data' : 'No results'} />
          </div>
        ) : (
          <table className="w-full border-collapse text-base">
            <thead>
              <tr>
                {COLUMNS.map(({ label, key }, idx) => (
                  <th
                    key={key}
                    onClick={() => handleSort(key)}
                    style={{
                      ...(idx === 0 ? TH_STICKY : { ...TH_STICKY, left: undefined, zIndex: 2 }),
                      cursor: 'pointer',
                      userSelect: 'none',
                      color: sortKey === key ? '#9C75FF' : 'rgba(255,255,255,0.3)'
                    }}
                  >
                    {label}
                    {sortKey === key && (
                      <span style={{ marginLeft: 3, fontSize: 9 }}>
                        {sortDir === 'desc' ? '▼' : '▲'}
                      </span>
                    )}
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
                  <td style={{ ...TD, color: (b.dau_dod_pct ?? 0) >= 0 ? '#1DF6B5' : '#FF6390' }}>
                    {fmtPct(b.dau_dod_pct)}
                  </td>
                  <td style={{ ...TD, color: '#fff' }}>{fmtNum(b.wau)}</td>
                  <td style={{ ...TD, color: (b.wau_wow_pct ?? 0) >= 0 ? '#1DF6B5' : '#FF6390' }}>
                    {fmtPct(b.wau_wow_pct)}
                  </td>
                  <td style={{ ...TD, color: '#fff' }}>{fmtNum(b.mau)}</td>
                  <td style={{ ...TD, color: (b.mau_mom_pct ?? 0) >= 0 ? '#1DF6B5' : '#FF6390' }}>
                    {fmtPct(b.mau_mom_pct)}
                  </td>
                  <td style={{ ...TD, color: 'rgba(255,255,255,0.7)' }}>{fmtNum(b.total_users)}</td>
                  <td style={{ ...TD, color: '#C4A9FF' }}>{fmtNum(b.new_users_30d)}</td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>
    </div>
  );
};
