import { SearchInput } from '~/components';

export default function Index() {
  return (
    <div className="flex flex-col flex-items-center gap-6">
      <h1>Orderly Network Dashboard</h1>

      <div>
        Welcome to the Orderly Network Dashboard!
        <br />
        Here you can query information about users like executed trades, deposits & withdrawals and
        liquidations.
        <br />
        <br />
        You can either search via wallet address or account ID:
      </div>

      <SearchInput />
    </div>
  );
}
