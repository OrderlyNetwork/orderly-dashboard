import { json } from '@remix-run/node';
import { useLoaderData } from '@remix-run/react';

import { DashboardsView } from '~/components/analytics/views/DashboardsView';
import type { DashboardData } from '~/types/dashboard';
import { fetchDashboardData } from '~/utils/data-api';

export async function loader() {
  const data = await fetchDashboardData(90);
  return json<DashboardData>(data);
}

export default function DashboardsPage() {
  const dashboardData = useLoaderData<typeof loader>();

  return <DashboardsView data={dashboardData} />;
}
