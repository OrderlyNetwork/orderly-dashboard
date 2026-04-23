import { redirect } from '@remix-run/node';
import type { LoaderFunctionArgs } from '@remix-run/node';

export async function loader(_args: LoaderFunctionArgs) {
  return redirect('/analytics');
}

export default function Dashboard() {
  return null;
}
