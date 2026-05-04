import { json } from '@remix-run/node';
import { useLoaderData } from '@remix-run/react';

import { useDashboardLayout } from '~/components/DashboardLayout';
import { DashboardsView } from '~/components/analytics/views/DashboardsView';
import type { DuneData, TvlChainRow } from '~/types/dune';

const DUNE_KEY = process.env.DUNE_API_KEY;

async function duneJson(queryId: number) {
  try {
    const res = await fetch(`https://api.dune.com/api/v1/query/${queryId}/results?limit=1000`, {
      headers: { 'x-dune-api-key': DUNE_KEY }
    });
    if (!res.ok) return { result: { rows: [] } };
    return res.json();
  } catch {
    return { result: { rows: [] } };
  }
}

export async function loader() {
  const [volData, tvlData, feeData, acctData, mktData, bldFeeData, bldData] = await Promise.all([
    duneJson(3368961),
    duneJson(6383913),
    duneJson(3429965),
    duneJson(3795110),
    duneJson(4119181),
    duneJson(3612752),
    duneJson(4119185)
  ]);

  return json<DuneData>({
    volumeRows: volData.result?.rows ?? [],
    tvlChains: (tvlData.result?.rows ?? []).filter((r: TvlChainRow) => r.chain !== 'Total'),
    feeRows: feeData.result?.rows ?? [],
    accountRows: acctData.result?.rows ?? [],
    marketRows: mktData.result?.rows ?? [],
    builderFees: bldFeeData.result?.rows?.[0]?.broker_fee ?? 0,
    activeBuilders: bldData.result?.rows?.[0]?.builders ?? 0
  });
}

export default function DashboardsPage() {
  const duneData = useLoaderData<typeof loader>();
  const { role } = useDashboardLayout();

  return <DashboardsView role={role} data={duneData} />;
}
