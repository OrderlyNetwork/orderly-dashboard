import { SearchInput, Leaderboard } from '~/components';

export default function Index() {
  return (
    <div className="flex flex-col flex-items-center gap-4 sm:gap-6 text-center">
      <h1 className="text-2xl sm:text-3xl lg:text-4xl">Orderly Network Dashboard</h1>

      <div className="text-sm sm:text-base max-w-lg px-4">
        Welcome to the Orderly Network Dashboard!
        <br />
        Here you can query information about users like executed trades, deposits & withdrawals and
        liquidations.
        <br />
        <br />
        <span className="hidden sm:block">
          You can either search via wallet address or account ID:
        </span>
      </div>

      <div className="w-full max-w-md hidden sm:flex flex-col items-center mb-10">
        <SearchInput />
      </div>
      <Leaderboard />
    </div>
  );
}
