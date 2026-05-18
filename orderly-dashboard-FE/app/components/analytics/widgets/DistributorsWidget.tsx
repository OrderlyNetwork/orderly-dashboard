import { FC } from 'react';

import { fmtNum, fmtUsd } from '../shared/formatters';
import { Empty, Skeleton, TD, TH_STICKY } from '../shared/primitives';

import { useDistributorStats } from '~/hooks/useOrderlyMetrics';

const DISTRIBUTOR_HEADERS = [
  'Fee Tier',
  'Invitees',
  'Graduated',
  '30D Volume',
  '30D Revenue',
  'Total Revenue'
];

export const DistributorsWidget: FC = () => {
  const { data: stats, isLoading: sLoad, error: sErr } = useDistributorStats();

  const distributors = Array.isArray(stats)
    ? [...stats]
        .filter((d) => (d['Number of Invitees'] ?? 0) > 0)
        .sort((a, b) => (b['30D Revenue Share'] ?? 0) - (a['30D Revenue Share'] ?? 0))
    : [];

  return (
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
                          left: undefined,
                          zIndex: 2
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
  );
};
