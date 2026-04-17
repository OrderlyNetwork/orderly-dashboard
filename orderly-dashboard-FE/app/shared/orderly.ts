/**
 * Orderly – single source of truth for shared constants & utilities.
 * Both the desktop DesktopHomePage and the mobile (MobileHomePage)
 * import from here so data is never duplicated.
 */

import type { HTMLAttributeAnchorTarget } from 'react';

// ── Public URLs ──────────────────────────────────────────────────────────────
export const AI_URL = 'https://orderly.network/skill.md';
export const NPX_CMD = 'npx @orderly.network/mcp-server init';
export const ORDER_BUY_URL = 'https://app.orderly.network/staking';

// ── DEX card links (mobile carousel) ────────────────────────────────────────
export const DEX_URLS: Record<string, string> = {
  Aden: 'https://aden.finance/',
  'WOOFi Pro': 'https://pro.woofi.com',
  Raydium: 'https://raydium.io/swap',
  Quickswap: 'https://quickswap.exchange/',
  'What.Exchange': 'https://www.what.exchange/',
  VOOI: 'https://vooi.io/',
  Kodiak: 'https://perps.kodiak.finance/',
  PERPTools: 'https://app.perptools.ai/',
  Zetarium: 'https://www.zdex.world/'
};

// ── Social links (ordered to match the icon row in both frames) ──────────────
export const SOCIAL_LINKS: string[] = [
  'https://discord.com/invite/OrderlyNetwork',
  'https://t.me/Orderly_Discussions',
  'https://www.youtube.com/@Orderly.Network',
  'https://www.linkedin.com/company/orderly-network',
  'https://x.com/OrderlyNetwork'
];

// ── Header nav dropdown items (desktop & mobile accordion) ────────────────────
export type HeaderNavItem = {
  label: string;
  href: string;
  external?: boolean;
  /** e.g. `"_blank"` for internal routes that should open in a new tab */
  target?: HTMLAttributeAnchorTarget;
};

export const HEADER_NAV: Record<string, HeaderNavItem[]> = {
  Builders: [
    { label: 'Orderly One', href: 'https://dex.orderly.network/', external: true },
    { label: 'My DEX', href: 'https://dex.orderly.network/dex', external: true },
    { label: 'Orderly SDKs', href: 'https://orderly.network/docs/sdks/overview', external: true },
    { label: 'GitHub', href: 'https://github.com/OrderlyNetwork', external: true }
  ],
  Ecosystem: [
    { label: 'Partners', href: '/partners', external: false, target: '_blank' },
    { label: 'Listings', href: 'https://orderly.network/listing/', external: true },
    { label: 'Case Studies', href: 'https://dex.orderly.network/case-studies', external: true },
    { label: 'Governance', href: 'https://snapshot.box/#/s:orderlygov.eth', external: true },
    { label: 'Staking', href: ORDER_BUY_URL, external: true },
    { label: 'Blog', href: '/blog', external: false }
  ],
  Traders: [
    { label: 'Live DEXs', href: 'https://dex.orderly.network/board/', external: true },
    { label: 'Explorer', href: 'https://explorer.orderly.network/', external: true },
    { label: 'Vaults', href: 'http://app.orderly.network/vaults', external: true },
    {
      label: 'API',
      href: 'https://orderly.network/docs/build-on-omnichain/evm-api/introduction',
      external: true
    }
  ]
};

// ── Header CTA ─────────────────────────────────────────────────────────────────
export const HEADER_CTA = {
  label: 'Launch Now',
  href: 'https://dex.orderly.network/dex/',
  external: true
};

// ── Docs link (shared between desktop & mobile) ────────────────────────────────
export const DOCS_LINK = {
  label: 'Docs',
  href: 'https://orderly.network/docs/introduction/getting-started/what-is-orderly',
  external: true
};

// ── Campaigns link (shared between desktop & mobile) ───────────────────────────
export const CAMPAIGNS_LINK = {
  label: 'Campaigns',
  href: 'https://app.orderly.network/campaigns/?utm_source=orderly_website&utm_medium=navbar',
  external: true
};

// ── Campaign dropdown items (desktop header) ───────────────────────────────────
export type CampaignItem = {
  titleVariant: string;
  subtitle?: string;
  href: string;
  status: 'Ongoing' | 'Ended';
  backgroundImageSrc: string;
  backgroundImageClassName: string;
  borderClassName: string;
};

export const CAMPAIGN_ITEMS: CampaignItem[] = [
  {
    titleVariant: 'perps',
    subtitle: 'Join to earn USDC reward',
    href: 'https://app.orderly.network/campaigns/?utm_source=orderly_website&utm_medium=navbar',
    status: 'Ongoing',
    backgroundImageSrc: '/campaigns/header/campaign-bg-2.png',
    backgroundImageClassName:
      'absolute h-[160%] left-0 max-w-none top-[-30%] w-full object-cover object-top',
    borderClassName:
      'absolute border border-[#48bdff] border-solid inset-0 pointer-events-none rounded-[8px]'
  },
  {
    titleVariant: 'ucc',
    href: 'https://app.orderly.network/campaigns/ultimate-crypto-championship',
    status: 'Ended',
    backgroundImageSrc: '/campaigns/header/campaign-bg-1.png',
    backgroundImageClassName:
      'absolute h-[494.34%] left-[0.24%] max-w-none top-[-144.53%] w-full object-cover object-top',
    borderClassName:
      'absolute border border-[rgba(255,255,255,0.06)] border-solid inset-0 pointer-events-none rounded-[8px]'
  }
];

// ── Footer nav sections (mobile accordion + desktop footer columns) ───────────
export type FooterNavLink = {
  label: string;
  href: string;
  external: boolean;
  target?: HTMLAttributeAnchorTarget;
};

export const FOOTER_NAV: Record<string, FooterNavLink[]> = {
  Builders: [
    {
      label: 'Orderly One',
      href: 'https://dex.orderly.network/',
      external: true
    },
    {
      label: 'My DEX',
      href: 'https://dex.orderly.network/dex',
      external: true
    },
    {
      label: 'Orderly SDKs',
      href: 'https://orderly.network/docs/sdks/overview',
      external: true
    },
    {
      label: 'Docs',
      href: 'https://orderly.network/docs/introduction/getting-started/what-is-orderly',
      external: true
    },
    {
      label: 'Github',
      href: 'https://github.com/OrderlyNetwork',
      external: true
    }
  ],
  Ecosystem: [
    {
      label: 'Partners',
      href: '/partners',
      external: false,
      target: '_blank'
    },
    {
      label: 'Listings',
      href: 'https://orderly.network/listing/',
      external: true
    },
    {
      label: 'Case Studies',
      href: 'https://dex.orderly.network/case-studies',
      external: true
    },
    {
      label: 'Governance',
      href: 'https://snapshot.box/#/s:orderlygov.eth',
      external: true
    },
    { label: 'Staking', href: ORDER_BUY_URL, external: true },
    { label: 'Blog', href: '/blog', external: false }
  ],
  Traders: [
    {
      label: 'Live DEXs',
      href: 'https://dex.orderly.network/board/',
      external: true
    },
    {
      label: 'Dashboard',
      href: 'https://dashboard.orderly.network',
      external: true
    },
    {
      label: 'Explorer',
      href: 'https://explorer.orderly.network/',
      external: true
    },
    {
      label: 'Vaults',
      href: 'http://app.orderly.network/vaults',
      external: true
    },
    {
      label: 'API',
      href: 'https://orderly.network/docs/build-on-omnichain/evm-api/introduction',
      external: true
    },
    {
      label: 'Campaigns',
      href: 'https://app.orderly.network/campaigns/?utm_source=orderly_website&utm_medium=navbar',
      external: true
    }
  ],
  About: [
    { label: 'Team', href: 'https://orderly.network/team/', external: true },
    { label: 'FAQ', href: '/faq', external: false },
    {
      label: 'Dune Dashboard',
      href: 'https://dune.com/orderly_network/orderly-dashboard',
      external: true
    },
    { label: 'Careers', href: 'https://job-boards.greenhouse.io/orderly', external: true },
    {
      label: 'Brand Kit',
      href: 'https://live.standards.site/orderly-brandguidelines',
      external: true
    }
  ]
};

// ── Utilities ────────────────────────────────────────────────────────────────

/** Copy to clipboard with execCommand fallback (works in restricted iframes) */
export async function copyToClipboard(text: string): Promise<boolean> {
  if (navigator.clipboard?.writeText) {
    try {
      await navigator.clipboard.writeText(text);
      return true;
    } catch {
      /* fall through */
    }
  }
  try {
    const el = document.createElement('textarea');
    el.value = text;
    el.setAttribute('readonly', '');
    el.style.cssText = 'position:absolute;left:-9999px;top:-9999px;opacity:0;pointer-events:none';
    document.body.appendChild(el);
    el.select();
    const ok = document.execCommand('copy');
    document.body.removeChild(el);
    return ok;
  } catch {
    return false;
  }
}

export function openUrl(url: string): void {
  window.open(url, '_blank', 'noopener,noreferrer');
}
