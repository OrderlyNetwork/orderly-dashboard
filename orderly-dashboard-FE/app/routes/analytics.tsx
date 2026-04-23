import { useEffect, useState } from 'react';
import { json } from '@remix-run/node';
import { useLoaderData, useRevalidator } from '@remix-run/react';
import type { LoaderFunctionArgs } from '@remix-run/node';

import { SearchModal } from '~/components/analytics/SearchModal';
import { Sidebar, type NavId, type Role } from '~/components/analytics/Sidebar';
import { Topbar } from '~/components/analytics/Topbar';
import { DashboardsView } from '~/components/analytics/views/DashboardsView';
import { QueriesView } from '~/components/analytics/views/QueriesView';
import { ApiCatalogView } from '~/components/analytics/views/ApiCatalogView';
import { StarredView } from '~/components/analytics/views/StarredView';
import { LeaderboardView } from '~/components/analytics/views/LeaderboardView';
import { ExplorerView } from '~/components/analytics/views/ExplorerView';
import { useStarred } from '~/hooks/useStarred';
import { useRefreshTimer } from '~/hooks/useRefreshTimer';

// ──────────────────────────────────────────────────────────────────────────────
// Types
// ──────────────────────────────────────────────────────────────────────────────

export type VolumeRow = {
  volume_date: string;
  volume: number;
  cumulative_volume: number;
  rolling_total_volume: number;
};

export type TvlChainRow = {
  chain: string;
  tvl: number;
};

export type FeeRow = {
  volume_date: string;
  cum_rev: number;
  daily_rev: number;
  rolling_rev: number;
  rolling_total_rev: number;
};

export type AccountRow = {
  date: string;
  cumulative_accounts: number;
  new_accounts: number;
};

export type MarketRow = {
  trade_week: string;
  markets: number;
};

export type DuneData = {
  volumeRows: VolumeRow[];
  tvlChains: TvlChainRow[];
  feeRows: FeeRow[];
  accountRows: AccountRow[];
  marketRows: MarketRow[];
  builderFees: number;
  activeBuilders: number;
};

// ──────────────────────────────────────────────────────────────────────────────
// Loader (server-side Dune fetch)
// ──────────────────────────────────────────────────────────────────────────────

const DUNE_KEY = process.env.DUNE_API_KEY;

async function duneJson(queryId: number) {
  try {
    const res = await fetch(
      `https://api.dune.com/api/v1/query/${queryId}/results?limit=1000`,
      { headers: { 'x-dune-api-key': DUNE_KEY } }
    );
    if (!res.ok) return { result: { rows: [] } };
    return res.json();
  } catch {
    return { result: { rows: [] } };
  }
}

export async function loader(_args: LoaderFunctionArgs) {
  const [volData, tvlData, feeData, acctData, mktData, bldFeeData, bldData] =
    await Promise.all([
      duneJson(3368961),  // volume
      duneJson(6383913),  // tvl by chain
      duneJson(3429965),  // net fees
      duneJson(3795110),  // total accounts
      duneJson(4119181),  // markets
      duneJson(3612752),  // builder fees
      duneJson(4119185),  // active builders
    ]);

  return json<DuneData>({
    volumeRows: volData.result?.rows ?? [],
    tvlChains: (tvlData.result?.rows ?? []).filter(
      (r: TvlChainRow) => r.chain !== 'Total'
    ),
    feeRows: feeData.result?.rows ?? [],
    accountRows: acctData.result?.rows ?? [],
    marketRows: mktData.result?.rows ?? [],
    builderFees: bldFeeData.result?.rows?.[0]?.broker_fee ?? 0,
    activeBuilders: bldData.result?.rows?.[0]?.builders ?? 0,
  });
}

// ──────────────────────────────────────────────────────────────────────────────
// CSV export helpers
// ──────────────────────────────────────────────────────────────────────────────

function toCsv(rows: object[]): string {
  if (rows.length === 0) return '';
  const headers = Object.keys(rows[0]);
  const lines = [
    headers.join(','),
    ...rows.map((r) =>
      headers
        .map((h) => {
          const v = String((r as Record<string, unknown>)[h] ?? '');
          return v.includes(',') ? `"${v}"` : v;
        })
        .join(',')
    )
  ];
  return lines.join('\n');
}

function downloadCsv(filename: string, content: string) {
  const blob = new Blob([content], { type: 'text/csv;charset=utf-8;' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = filename;
  a.click();
  URL.revokeObjectURL(url);
}

function exportForView(nav: NavId, data: DuneData) {
  switch (nav) {
    case 'dashboards':
      downloadCsv(
        'orderly-volume.csv',
        toCsv(
          data.volumeRows.map((r) => ({
            date: r.volume_date,
            daily_volume_usd: r.volume,
            cumulative_volume_usd: r.cumulative_volume,
            rolling_30d_volume_usd: r.rolling_total_volume
          }))
        )
      );
      break;
    case 'queries':
      downloadCsv('queries-export.csv', 'name,category,runCount\n');
      break;
    case 'api-catalog':
      downloadCsv('api-catalog.csv', 'method,path,category,auth\n');
      break;
    case 'starred':
      downloadCsv('starred-export.csv', 'id,type,title\n');
      break;
    case 'leaderboard':
      downloadCsv('leaderboard-export.csv', 'rank,address,perp_volume,realized_pnl\n');
      break;
    case 'explorer':
      downloadCsv('explorer-export.csv', 'address,chain\n');
      break;
  }
}

// ──────────────────────────────────────────────────────────────────────────────

const SIDEBAR_WIDTH = 224;

export default function Analytics() {
  const duneData = useLoaderData<typeof loader>();
  const { revalidate } = useRevalidator();

  const [activeNav, setActiveNav] = useState<NavId>('dashboards');
  const [role, setRole] = useState<Role>('trader');
  const [searchOpen, setSearchOpen] = useState(false);

  const { secondsAgo, refresh } = useRefreshTimer(() => revalidate());
  const { starred, isStarred, toggleStar, removeStar } = useStarred();

  // ⌘K global shortcut
  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
        e.preventDefault();
        setSearchOpen((v) => !v);
      }
    };
    window.addEventListener('keydown', handler);
    return () => window.removeEventListener('keydown', handler);
  }, []);

  const handleSearchNavigate = (id: string) => {
    const navIds: NavId[] = ['dashboards', 'queries', 'api-catalog', 'starred', 'leaderboard', 'explorer'];
    if (navIds.includes(id as NavId)) {
      setActiveNav(id as NavId);
    } else {
      setActiveNav('dashboards');
    }
  };

  return (
    <>
      {/* Full-viewport overlay */}
      <div
        style={{
          position: 'fixed',
          inset: 0,
          zIndex: 100,
          background: '#0A0010',
          display: 'flex',
          fontFamily:
            '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif'
        }}
      >
        {/* Sidebar */}
        <Sidebar
          activeNav={activeNav}
          onNavChange={setActiveNav}
          role={role}
          onRoleChange={setRole}
        />

        {/* Main content area */}
        <div
          style={{
            marginLeft: SIDEBAR_WIDTH,
            flex: 1,
            display: 'flex',
            flexDirection: 'column',
            minWidth: 0,
            overflow: 'hidden'
          }}
        >
          {/* Topbar */}
          <Topbar
            activeNav={activeNav}
            secondsAgo={secondsAgo}
            onRefresh={refresh}
            onExportCsv={() => exportForView(activeNav, duneData)}
            onSearchOpen={() => setSearchOpen(true)}
          />

          {/* Scrollable content */}
          <div
            style={{
              marginTop: 64,
              flex: 1,
              overflowY: 'auto',
              padding: '28px 32px',
              minHeight: 0
            }}
          >
            {activeNav === 'dashboards' && (
              <DashboardsView
                role={role}
                isStarred={isStarred}
                toggleStar={toggleStar}
                data={duneData}
              />
            )}
            {activeNav === 'queries' && (
              <QueriesView isStarred={isStarred} toggleStar={toggleStar} />
            )}
            {activeNav === 'api-catalog' && <ApiCatalogView />}
            {activeNav === 'starred' && (
              <StarredView
                starred={starred}
                onRemove={removeStar}
                onNavigate={(type) =>
                  setActiveNav(type === 'dashboard' ? 'dashboards' : 'queries')
                }
              />
            )}
            {activeNav === 'leaderboard' && <LeaderboardView />}
            {activeNav === 'explorer' && <ExplorerView />}
          </div>
        </div>
      </div>

      {/* ⌘K Search modal */}
      <SearchModal
        open={searchOpen}
        onClose={() => setSearchOpen(false)}
        onNavigate={handleSearchNavigate}
      />
    </>
  );
}
