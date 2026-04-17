import { useSearchParams, useNavigate } from '@remix-run/react';
import { useState, useEffect } from 'react';

import { SearchInput, Leaderboard, Positions } from '~/components';

export default function Index() {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();

  const initialTab = searchParams.get('tab') as 'trading' | 'positions' | null;
  const [activeTab, setActiveTab] = useState<'trading' | 'positions'>(
    initialTab === 'trading' || initialTab === 'positions' ? initialTab : 'trading'
  );

  useEffect(() => {
    const currentTab = searchParams.get('tab') as 'trading' | 'positions' | null;
    if (currentTab === 'trading' || currentTab === 'positions') {
      setActiveTab(currentTab);
    } else {
      setActiveTab('trading');
    }
  }, [searchParams]);

  const handleTabChange = (newTab: 'trading' | 'positions') => {
    const newSearchParams = new URLSearchParams(searchParams);
    if (newTab === 'trading') {
      newSearchParams.delete('tab');
    } else {
      newSearchParams.set('tab', newTab);
    }

    navigate({
      pathname: window.location.pathname,
      search: `?${newSearchParams.toString()}`
    });

    setActiveTab(newTab);
  };

  return (
    <div className="space-y-6 sm:space-y-12 animate-fade-in">
      {/* Hero Section */}
      <div className="text-center">
        <div className="space-y-3 sm:space-y-6">
          <h1
            style={{
              fontFamily: 'var(--font-display)',
              fontSize: 'clamp(2rem, 8vw, 60px)',
              fontWeight: 800,
              lineHeight: 1.05,
              color: '#fff',
              letterSpacing: '-0.01em',
              marginTop: 'clamp(10px, 4vw, 30px)'
            }}
          >
            Orderly Dashboard
          </h1>
          <p
            style={{
              fontFamily: 'var(--font-display)',
              fontSize: 'clamp(1rem, 2vw, 1.25rem)',
              fontWeight: 500,
              color: 'rgba(255,255,255,0.7)',
              maxWidth: '900px',
              margin: '0 auto',
              lineHeight: 1.5,
              letterSpacing: '-0.02em'
            }}
          >
            Advanced analytics and insights for decentralized trading on Orderly
          </p>
        </div>

        {/* Search card */}
        <div className="mx-auto px-4 sm:px-6" style={{ maxWidth: 1200 }}>
          <div
            style={{
              maxWidth: '820px',
              margin: '36px auto 0',
              background: '#6700CE',
              border: 'none',
              borderRadius: '16px',
              padding: 'clamp(16px, 4vw, 36px)',
              backdropFilter: 'blur(20px)'
            }}
          >
            <div className="space-y-3 sm:space-y-4 mb-4 sm:mb-6">
              <h3
                style={{
                  fontFamily: 'var(--font-display)',
                  fontSize: '1.25rem',
                  fontWeight: 700,
                  color: '#fff',
                  margin: 0
                }}
              >
                Explore Trading Data
              </h3>
              <p style={{ color: '#fff', lineHeight: 1.6, margin: 0 }}>
                Search for wallet addresses or account IDs to view detailed trading information
                including executed trades, deposits &amp; withdrawals, liquidations, and performance
                metrics.
              </p>
            </div>

            <div className="space-y-4">
              <div
                className="flex flex-wrap items-center justify-center gap-4 sm:gap-6 text-sm"
                style={{ color: '#fff' }}
              >
                <div className="flex items-center space-x-2">
                  <div
                    style={{
                      width: 8,
                      height: 8,
                      borderRadius: '50%',
                      background: 'var(--color-success)'
                    }}
                  ></div>
                  <span className="font-bold">EVM Addresses</span>
                </div>
                <div className="flex items-center space-x-2">
                  <div
                    style={{
                      width: 8,
                      height: 8,
                      borderRadius: '50%',
                      background: 'var(--color-purple-accent)'
                    }}
                  ></div>
                  <span className="font-bold">Solana Addresses</span>
                </div>
                <div className="flex items-center space-x-2">
                  <div
                    style={{
                      width: 8,
                      height: 8,
                      borderRadius: '50%',
                      background: 'var(--color-warning)'
                    }}
                  ></div>
                  <span className="font-bold">Account IDs</span>
                </div>
              </div>

              <div className="flex justify-center w-full">
                <SearchInput />
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Leaderboard Section */}
      <div className="mt-8 sm:mt-16 mx-auto px-4 sm:px-6" style={{ maxWidth: 1200 }}>
        {/* Tab Navigation */}
        <div className="flex justify-center mb-4 sm:mb-8">
          <div
            className="max-w-full overflow-x-auto"
            style={{
              display: 'flex',
              gap: '8px',
              background: 'rgba(20, 21, 26, 0.7)',
              border: '1px solid rgba(156, 117, 255, 0.15)',
              borderRadius: '16px',
              padding: '6px'
            }}
          >
            <button
              onClick={() => handleTabChange('trading')}
              className="btn whitespace-nowrap text-xs sm:text-sm"
              style={
                activeTab === 'trading'
                  ? { background: 'var(--color-purple)', color: '#fff', borderRadius: '12px' }
                  : {
                      background: 'transparent',
                      color: 'rgba(255,255,255,0.6)',
                      borderRadius: '12px'
                    }
              }
            >
              Trading<span className="hidden sm:inline"> Leaderboard</span>
            </button>
            <button
              onClick={() => handleTabChange('positions')}
              className="btn whitespace-nowrap text-xs sm:text-sm"
              style={
                activeTab === 'positions'
                  ? { background: 'var(--color-purple)', color: '#fff', borderRadius: '12px' }
                  : {
                      background: 'transparent',
                      color: 'rgba(255,255,255,0.6)',
                      borderRadius: '12px'
                    }
              }
            >
              Positions<span className="hidden sm:inline"> Leaderboard</span>
            </button>
          </div>
        </div>

        {/* Tab Content */}
        {activeTab === 'trading' ? <Leaderboard /> : <Positions />}
      </div>
    </div>
  );
}
