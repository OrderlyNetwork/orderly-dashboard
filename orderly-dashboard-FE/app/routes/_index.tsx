import { useState } from 'react';

import { SearchInput, Leaderboard, Positions } from '~/components';

export default function Index() {
  const [activeTab, setActiveTab] = useState<'trading' | 'positions'>('trading');

  return (
    <div className="space-y-12 animate-fade-in">
      {/* Hero Section */}
      <div className="text-center space-y-8">
        <div className="space-y-6">
          <h1 className="text-4xl sm:text-5xl lg:text-6xl font-bold bg-gradient-to-r from-white via-gray-200 to-gray-400 bg-clip-text text-transparent">
            Orderly Network Dashboard
          </h1>
          <p className="text-xl sm:text-2xl text-gray-300 max-w-3xl mx-auto leading-relaxed">
            Advanced analytics and insights for decentralized trading on the Orderly Network
          </p>
        </div>

        <div className="card max-w-2xl mx-auto p-8 space-y-6">
          <div className="space-y-4">
            <h3 className="text-xl font-semibold text-white">Explore Trading Data</h3>
            <p className="text-gray-300 leading-relaxed">
              Search for wallet addresses or account IDs to view detailed trading information
              including executed trades, deposits & withdrawals, liquidations, and performance
              metrics.
            </p>
          </div>

          <div className="space-y-4">
            <div className="flex items-center justify-center space-x-4 text-sm text-gray-400">
              <div className="flex items-center space-x-2">
                <div className="w-2 h-2 bg-success rounded-full"></div>
                <span>EVM Addresses</span>
              </div>
              <div className="flex items-center space-x-2">
                <div className="w-2 h-2 bg-info rounded-full"></div>
                <span>Solana Addresses</span>
              </div>
              <div className="flex items-center space-x-2">
                <div className="w-2 h-2 bg-warning rounded-full"></div>
                <span>Account IDs</span>
              </div>
            </div>

            <div className="hidden sm:flex justify-center w-full">
              <SearchInput />
            </div>
          </div>
        </div>
      </div>

      {/* Leaderboard Section */}
      <div className="mt-16">
        {/* Tab Navigation */}
        <div className="flex justify-center mb-8">
          <div className="flex gap-2">
            <button
              onClick={() => setActiveTab('trading')}
              className={`btn ${activeTab === 'trading' ? 'btn-primary' : 'btn-secondary'}`}
            >
              Trading Leaderboard
            </button>
            <button
              onClick={() => setActiveTab('positions')}
              className={`btn ${activeTab === 'positions' ? 'btn-primary' : 'btn-secondary'}`}
            >
              Positions Leaderboard
            </button>
          </div>
        </div>

        {/* Tab Content */}
        {activeTab === 'trading' ? <Leaderboard /> : <Positions />}
      </div>
    </div>
  );
}
