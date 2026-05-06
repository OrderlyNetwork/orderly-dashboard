import { FC } from 'react';

import { fmtNum, fmtUsd } from '../shared/formatters';
import { Empty, Skeleton, TD, TH_STICKY, tdSticky } from '../shared/primitives';

import { useDistributorInvitees, useDistributorStats } from '~/hooks/useOrderlyMetrics';

const DISTRIBUTOR_HEADERS = [
  'Name',
  'Type',
  'Fee Tier',
  'Invitees',
  'Graduated',
  '30D Volume',
  '30D Revenue',
  'Total Revenue'
];

const INVITEE_HEADERS = [
  'Invitee DEX',
  'Status',
  'Orderly One',
  'DEX Link',
  '30D Volume',
  '30D Revenue',
  'Date Invited'
];

export const DistributorsWidget: FC = () => {
  const { data: stats, isLoading: sLoad, error: sErr } = useDistributorStats();
  const { data: invitees, isLoading: iLoad, error: iErr } = useDistributorInvitees();

  const distributors = Array.isArray(stats)
    ? [...stats].sort((a, b) => (b['30D Revenue Share'] ?? 0) - (a['30D Revenue Share'] ?? 0))
    : [];
  const invList = Array.isArray(invitees)
    ? [...invitees].sort((a, b) => (b['30D Invitee Volume'] ?? 0) - (a['30D Invitee Volume'] ?? 0))
    : [];

  return (
    <div className="flex flex-col gap-4">
      <div className="overflow-x-auto max-h-[300px] overflow-y-auto">
        {sLoad ? (
          <div className="p-5">
            <Skeleton height={120} />
          </div>
        ) : sErr || distributors.length === 0 ? (
          <div className="p-6">
            <Empty msg={sErr ? 'Failed to load' : 'No data'} />
          </div>
        ) : (
          <table className="w-full border-collapse text-xs">
            <thead>
              <tr>
                {DISTRIBUTOR_HEADERS.map((h, idx) => (
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
              {distributors.map((d, i) => (
                <tr key={i}>
                  <td style={{ ...tdSticky(i), color: '#9C75FF', fontWeight: 600 }}>
                    {d['Distributor Name'] ?? '—'}
                  </td>
                  <td style={{ ...TD, color: 'rgba(255,255,255,0.5)' }}>
                    {d['Distributor Type'] ?? '—'}
                  </td>
                  <td style={TD}>
                    <span
                      className="py-[2px] px-2 rounded-md text-[11px]"
                      style={{
                        background:
                          d['Fee Tier'] === 'PLATINUM'
                            ? 'rgba(156,117,255,0.2)'
                            : 'rgba(255,255,255,0.05)',
                        color: d['Fee Tier'] === 'PLATINUM' ? '#c4a8ff' : 'rgba(255,255,255,0.5)'
                      }}
                    >
                      {d['Fee Tier'] ?? '—'}
                    </span>
                  </td>
                  <td style={{ ...TD, color: '#fff' }}>{fmtNum(d['Number of Invitees'])}</td>
                  <td style={{ ...TD, color: '#34d399' }}>
                    {fmtNum(d['Number of Graduated Invitees'])}
                  </td>
                  <td style={{ ...TD, color: '#60a5fa' }}>{fmtUsd(d['30D Invitee Volume'])}</td>
                  <td style={{ ...TD, color: '#f59e0b', fontWeight: 600 }}>
                    {fmtUsd(d['30D Revenue Share'])}
                  </td>
                  <td style={{ ...TD, color: 'rgba(255,255,255,0.6)' }}>
                    {fmtUsd(d['Total Revenue Share'])}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>

      <div className="overflow-x-auto max-h-[300px] overflow-y-auto">
        {iLoad ? (
          <div className="p-5">
            <Skeleton height={120} />
          </div>
        ) : iErr || invList.length === 0 ? (
          <div className="p-6">
            <Empty msg={iErr ? 'Failed to load' : 'No data'} />
          </div>
        ) : (
          <table className="w-full border-collapse text-xs">
            <thead>
              <tr>
                {INVITEE_HEADERS.map((h, idx) => (
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
              {invList.map((inv, i) => (
                <tr key={i}>
                  <td style={{ ...tdSticky(i), color: '#9C75FF', fontWeight: 600 }}>
                    {inv['Invitee DEX'] ?? '—'}
                  </td>
                  <td style={TD}>
                    <span
                      className="py-[2px] px-2 rounded-md text-[11px]"
                      style={{
                        background:
                          inv['DEX status'] === 'Graduated'
                            ? 'rgba(52,211,153,0.15)'
                            : 'rgba(255,255,255,0.05)',
                        color:
                          inv['DEX status'] === 'Graduated' ? '#34d399' : 'rgba(255,255,255,0.5)'
                      }}
                    >
                      {inv['DEX status'] ?? '—'}
                    </span>
                  </td>
                  <td
                    style={{
                      ...TD,
                      color: inv['is_orderly_one'] === 'yes' ? '#34d399' : 'rgba(255,255,255,0.4)'
                    }}
                  >
                    {inv['is_orderly_one'] === 'yes' ? '✓' : '—'}
                  </td>
                  <td style={{ ...TD, color: '#60a5fa', fontSize: 11 }}>
                    {inv['DEX link'] ?? '—'}
                  </td>
                  <td style={{ ...TD, color: '#60a5fa', fontWeight: 600 }}>
                    {fmtUsd(inv['30D Invitee Volume'])}
                  </td>
                  <td style={{ ...TD, color: '#f59e0b' }}>{fmtUsd(inv['30D Revenue Share'])}</td>
                  <td style={{ ...TD, color: 'rgba(255,255,255,0.4)', fontSize: 11 }}>
                    {inv['Date Invited'] ? new Date(inv['Date Invited']).toLocaleDateString() : '—'}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>
    </div>
  );
};
