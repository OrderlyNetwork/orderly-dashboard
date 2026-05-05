const DUNE_KEY = process.env.DUNE_API_KEY;

export async function duneJson(queryId: number) {
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
