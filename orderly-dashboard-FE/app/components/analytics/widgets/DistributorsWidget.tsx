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

const FS: React.CSSProperties = { fontFeatureSettings: "'ss02' 1, 'ss03' 1, 'ss05' 1, 'ss06' 1" };

export const DistributorsWidget: FC = () => {
  const { data: stats, isLoading: sLoad, error: sErr } = useDistributorStats();

  const distributors = Array.isArray(stats)
    ? [...stats]
        .filter((d) => (d['Number of Invitees'] ?? 0) > 0)
        .sort((a, b) => (b['30D Revenue Share'] ?? 0) - (a['30D Revenue Share'] ?? 0))
    : [];

  return (
    <div
      className="overflow-x-auto max-h-[300px] overflow-y-auto chips-scrollbar"
      style={{ scrollbarWidth: 'thin', scrollbarColor: 'rgba(156,117,255,0.25) #130E1D' }}
    >
      {sLoad ? (
        <div className="p-5">
          <Skeleton height={120} />
        </div>
      ) : sErr || distributors.length === 0 ? (
        <div className="p-6">
          <Empty msg={sErr ? 'Failed to load' : 'No data'} />
        </div>
      ) : (
        <table className="w-full border-collapse text-[15px]" style={{ fontFamily: "'Atyp BL Text', 'Atyp BL', sans-serif", fontWeight: 400, fontVariantNumeric: 'normal', fontFeatureSettings: "'ss02' 1, 'ss03' 1, 'ss05' 1, 'ss06' 1" }}>
          <thead>
            <tr>
              {DISTRIBUTOR_HEADERS.map((h, idx) => (
                <th
                  key={h}
                  style={
                    idx === 0
                      ? { ...TH_STICKY, ...FS }
                      : {
                          ...TH_STICKY,
                          ...FS,
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
                <td style={{ ...TD, ...FS }}>
                  <span
                    className="py-[2px] px-2 rounded-md text-xs"
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
                <td style={{ ...TD, ...FS, color: '#fff' }}>{fmtNum(d['Number of Invitees'])}</td>
                <td style={{ ...TD, ...FS, color: '#00dea3' }}>
                  {fmtNum(d['Number of Graduated Invitees'])}
                </td>
                <td style={{ ...TD, ...FS, color: '#fff' }}>{fmtUsd(d['30D Invitee Volume'])}</td>
                <td style={{ ...TD, ...FS, color: '#fff' }}>
                  {fmtUsd(d['30D Revenue Share'])}
                </td>
                <td style={{ ...TD, ...FS, color: '#D9AB52' }}>
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
