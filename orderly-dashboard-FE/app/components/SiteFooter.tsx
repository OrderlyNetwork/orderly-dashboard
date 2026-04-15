import { useState, useEffect } from "react";
import { createPortal } from "react-dom";
import { motion, AnimatePresence } from "motion/react";
import { FOOTER_NAV, ORDER_BUY_URL, SOCIAL_LINKS, type FooterNavLink } from "~/shared/orderly";

// ─── Inlined SVG paths ────────────────────────────────────────────────────────
const SVG = {
  brandmark1:
    "M23.9432 0L24.0573 8.38177e-09C30.0744 0.0139813 35.5713 2.2397 39.7762 5.90632C40.0742 6.16613 39.8855 6.64295 39.4899 6.64295L8.51054 6.64294C8.11501 6.64294 7.92631 6.16613 8.22427 5.90631C12.4292 2.23969 17.9261 0.0139804 23.9432 0Z",
  brandmark2:
    "M14.6653 30.4959C14.4936 30.2513 14.2171 30.0977 13.9181 30.0977L1.50488 30.0977C1.14527 30.0977 0.883086 30.4382 0.985128 30.7826C3.9241 40.7021 13.1162 47.9395 24.0007 47.9395C34.8853 47.9395 44.0774 40.7021 47.0163 30.7826C47.1184 30.4382 46.8562 30.0977 46.4966 30.0977L34.0836 30.0977C33.7845 30.0977 33.5081 30.2513 33.3364 30.4959C31.2756 33.4309 27.8626 35.3497 24.0008 35.3497C20.1391 35.3497 16.7261 33.4309 14.6653 30.4959Z",
  brandmark3:
    "M33.0371 17.0372C33.2565 17.3222 33.5907 17.5009 33.9506 17.5009L46.3915 17.5009C46.7544 17.5009 47.0172 17.1549 46.9089 16.8089C45.9688 13.8049 44.4527 11.055 42.4899 8.68812C42.3265 8.49111 42.0825 8.37989 41.8263 8.37989L6.1717 8.37988C5.91557 8.37988 5.67153 8.49111 5.50814 8.68812C3.54528 11.055 2.02921 13.8049 1.08907 16.8089C0.980778 17.1549 1.24359 17.5009 1.60656 17.5009L14.0475 17.5009C14.4074 17.5009 14.7416 17.3222 14.961 17.0372C17.0441 14.3316 20.3177 12.5879 23.999 12.5879C27.6804 12.5879 30.954 14.3316 33.0371 17.0372Z",
  brandmark4:
    "M35.0087 28.3636C34.7737 28.3636 34.6141 28.1244 34.6952 27.9042C35.1468 26.6789 35.3933 25.3546 35.3933 23.9728C35.3933 22.4664 35.1003 21.0284 34.5681 19.7125C34.4784 19.4906 34.6382 19.2426 34.8779 19.2426L47.0812 19.2426C47.3425 19.2426 47.5682 19.4266 47.6146 19.6835C47.8664 21.0753 47.9978 22.5089 47.9978 23.9732C47.9978 25.3157 47.8873 26.6325 47.6749 27.9149C47.6318 28.1754 47.4044 28.3636 47.1401 28.3636L35.0087 28.3636ZM13.3026 27.9042C13.3838 28.1244 13.2241 28.3636 12.9891 28.3636L0.857763 28.3636C0.593443 28.3636 0.366068 28.1754 0.322925 27.9149C0.110505 26.6325 0 25.3157 0 23.9732C0 22.5089 0.131454 21.0753 0.383218 19.6835C0.42968 19.4266 0.655361 19.2426 0.916684 19.2426L13.12 19.2426C13.3596 19.2426 13.5195 19.4906 13.4297 19.7125C12.8975 21.0284 12.6045 22.4664 12.6045 23.9728C12.6045 25.3546 12.8511 26.6789 13.3026 27.9042Z",
  discord:
    "M13.5447 1.01094C12.5249 0.543021 11.4313 0.198277 10.2879 0.000828273C10.2671 -0.00298243 10.2463 0.00654073 10.2356 0.0255875C10.0949 0.275731 9.93915 0.602063 9.83006 0.858559C8.60027 0.674446 7.37679 0.674446 6.17221 0.858559C6.0631 0.596362 5.90166 0.275731 5.76038 0.0255875C5.74966 0.00717622 5.72886 -0.00234694 5.70803 0.000828273C4.56527 0.197646 3.47171 0.54239 2.45129 1.01094C2.44246 1.01475 2.43488 1.02111 2.42986 1.02935C0.355594 4.12826 -0.212633 7.151 0.0661201 10.1363C0.0673814 10.1509 0.0755799 10.1648 0.086932 10.1737C1.45547 11.1787 2.78114 11.7889 4.08219 12.1933C4.10301 12.1997 4.12507 12.192 4.13832 12.1749C4.44608 11.7546 4.72043 11.3114 4.95565 10.8454C4.96953 10.8181 4.95628 10.7857 4.92791 10.775C4.49275 10.6099 4.0784 10.4086 3.67982 10.1801C3.64829 10.1617 3.64577 10.1166 3.67477 10.095C3.75865 10.0321 3.84255 9.96673 3.92264 9.9007C3.93713 9.88864 3.95732 9.8861 3.97435 9.89372C6.59286 11.0892 9.4277 11.0892 12.0153 9.89372C12.0323 9.88547 12.0525 9.88801 12.0677 9.90007C12.1478 9.9661 12.2316 10.0321 12.3161 10.095C12.3451 10.1166 12.3433 10.1617 12.3117 10.1801C11.9131 10.4131 11.4988 10.6099 11.063 10.7743C11.0346 10.7851 11.022 10.8181 11.0359 10.8454C11.2762 11.3108 11.5505 11.754 11.8526 12.1743C11.8652 12.192 11.8879 12.1997 11.9087 12.1933C13.2161 11.7889 14.5417 11.1787 15.9103 10.1737C15.9223 10.1648 15.9298 10.1515 15.9311 10.1369C16.2647 6.6856 15.3723 3.68765 13.5655 1.02998C13.5611 1.02111 13.5535 1.01475 13.5447 1.01094ZM5.34668 8.31855C4.55833 8.31855 3.90875 7.59478 3.90875 6.70593C3.90875 5.81707 4.54574 5.09331 5.34668 5.09331C6.15392 5.09331 6.79721 5.82343 6.78459 6.70593C6.78459 7.59478 6.14761 8.31855 5.34668 8.31855ZM10.6632 8.31855C9.87484 8.31855 9.22526 7.59478 9.22526 6.70593C9.22526 5.81707 9.86222 5.09331 10.6632 5.09331C11.4704 5.09331 12.1137 5.82343 12.1011 6.70593C12.1011 7.59478 11.4704 8.31855 10.6632 8.31855Z",
  telegram:
    "M8 0C12.4184 0 16 3.58157 16 8C16 12.4184 12.4184 16 8 16C3.58157 16 0 12.4184 0 8C0 3.58157 3.58157 0 8 0ZM10.7597 11.2609C10.9068 10.8094 11.5962 6.30991 11.6814 5.4233C11.7071 5.15478 11.6223 4.97635 11.456 4.8967C11.255 4.8 10.9572 4.84835 10.6118 4.97287C10.1381 5.14365 4.08174 7.71513 3.73183 7.864C3.4 8.00487 3.08626 8.15861 3.08626 8.38122C3.08626 8.53774 3.17913 8.62574 3.43513 8.71722C3.70157 8.81217 4.37252 9.01565 4.7687 9.12487C5.15026 9.23026 5.5847 9.13878 5.82817 8.98748C6.08626 8.82713 9.0647 6.83409 9.27861 6.65948C9.49217 6.48487 9.66261 6.70852 9.488 6.88348C9.31339 7.05809 7.26887 9.04244 6.9993 9.31722C6.672 9.65078 6.90435 9.99652 7.12383 10.135C7.37461 10.2929 9.17809 11.5026 9.44974 11.6967C9.72139 11.8908 9.99687 11.9788 10.249 11.9788C10.5012 11.9788 10.6341 11.6466 10.7597 11.2609Z",
  twitter:
    "M11.5509 0H13.7998L8.88648 5.61566L14.6667 13.2573H10.1408L6.59602 8.62269L2.53997 13.2573H0.289621L5.54496 7.25073L0 0H4.64074L7.84493 4.23622L11.5509 0ZM10.7615 11.9112H12.0077L3.9636 1.27542H2.62631L10.7615 11.9112Z",
  linkedin:
    "M14.8156 0H1.18125C0.528125 0 0 0.515625 0 1.15313V14.8438C0 15.4813 0.528125 16 1.18125 16H14.8156C15.4688 16 16 15.4813 16 14.8469V1.15313C16 0.515625 15.4688 0 14.8156 0ZM4.74687 13.6344H2.37188V5.99687H4.74687V13.6344ZM3.55938 4.95625C2.79688 4.95625 2.18125 4.34062 2.18125 3.58125C2.18125 2.82188 2.79688 2.20625 3.55938 2.20625C4.31875 2.20625 4.93437 2.82188 4.93437 3.58125C4.93437 4.3375 4.31875 4.95625 3.55938 4.95625ZM13.6344 13.6344H11.2625V9.92188C11.2625 9.0375 11.2469 7.89687 10.0281 7.89687C8.79375 7.89687 8.60625 8.8625 8.60625 9.85938V13.6344H6.2375V5.99687H8.5125V7.04063H8.54375C8.85937 6.44063 9.63438 5.80625 10.7875 5.80625C13.1906 5.80625 13.6344 7.3875 13.6344 9.44375V13.6344V13.6344Z",
  buyArrow: "M8.58333 1.25L1.25 8.58333M1.25 8.58333L8.58333 15.9167M1.25 8.58333H15.9167",
} as const;

const fontRegularBuy = {
  fontFamily: "'atyp-bl-variable', 'Atyp BL', sans-serif",
  fontWeight: 400,
};
const fontMediumBuy = {
  fontFamily: "'atyp-bl-variable', 'Atyp BL', sans-serif",
  fontWeight: 500,
};

// ─── Exchange logos ───────────────────────────────────────────────────────────
function ExternalArrow() {
  return (
    <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
      <path d="M3 13L13 3M7 3h6v6" stroke="#9c75ff" strokeWidth="1.6" strokeLinecap="round" strokeLinejoin="round" />
    </svg>
  );
}

const BUY_EXCHANGES = [
  { name: "Uniswap",     color: "#FF007A", url: "https://app.uniswap.org/swap?outputCurrency=ORDER" },
  { name: "PancakeSwap", color: "#1FC7D4", url: "https://pancakeswap.finance/swap?outputCurrency=ORDER" },
  { name: "Camelot",     color: "#F5A81C", url: "https://app.camelot.exchange/" },
  { name: "Raydium",     color: "#6C44FC", url: "https://raydium.io/swap/" },
  { name: "WOOFi",       color: "#5D58F6", url: "https://fi.woo.org/" },
];

// ─── Buy ORDER modal ──────────────────────────────────────────────────────────
export function BuyOrderModal({ onClose }: { onClose: () => void }) {
  return createPortal(
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
      className="fixed inset-0 z-[9999] flex items-center justify-center p-[16px]"
      style={{ backgroundColor: "rgba(0,0,0,0.75)" }}
      onClick={onClose}
    >
      <motion.div
        initial={{ opacity: 0, scale: 0.94, y: 12 }}
        animate={{ opacity: 1, scale: 1, y: 0 }}
        exit={{ opacity: 0, scale: 0.94, y: 12 }}
        transition={{ duration: 0.2, ease: [0.22, 0.61, 0.36, 1] }}
        className="relative bg-[#14151a] rounded-[28px] w-full max-w-[460px] overflow-hidden shadow-2xl"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex items-start justify-between px-[28px] pt-[28px] pb-[20px]">
          <div>
            <h2 className="text-[24px] text-white tracking-[0.24px]" style={{ fontFamily: "'atyp-bl-variable', 'Atyp BL', sans-serif", fontWeight: 700 }}>
              Buy ORDER
            </h2>
            <p className="text-[13px] text-[#9c9fae] mt-[4px] tracking-[0.13px]" style={fontRegularBuy}>
              Select an exchange to purchase ORDER tokens.
            </p>
          </div>
          <button
            onClick={onClose}
            className="shrink-0 ml-[16px] -mt-[2px] -mr-[4px] text-[#9c9fae] hover:text-white hover:bg-white/[0.08] transition-all bg-transparent border-0 cursor-pointer rounded-full p-[8px]"
            aria-label="Close"
          >
            <svg width="16" height="16" viewBox="0 0 20 20" fill="none">
              <path d="M15 5L5 15M5 5l10 10" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" />
            </svg>
          </button>
        </div>

        <div className="mx-[28px] h-px" style={{ background: "rgba(255,255,255,0.07)" }} />

        <div className="px-[20px] py-[16px] flex flex-col gap-[6px]">
          {BUY_EXCHANGES.map((ex) => (
            <a
              key={ex.name}
              href={ex.url}
              target="_blank"
              rel="noopener noreferrer"
              className="flex items-center gap-[14px] no-underline rounded-[16px] px-[12px] py-[12px] transition-all duration-200 group"
              style={{ background: "transparent" }}
              onMouseEnter={(e) => { (e.currentTarget as HTMLAnchorElement).style.background = "rgba(156,117,255,0.1)"; }}
              onMouseLeave={(e) => { (e.currentTarget as HTMLAnchorElement).style.background = "transparent"; }}
            >
              <div
                className="relative shrink-0 size-[44px] rounded-full flex items-center justify-center text-white font-bold text-[13px]"
                style={{ background: ex.color }}
              >
                {ex.name[0]}
              </div>
              <span className="text-[15px] text-white tracking-[0.01em] flex-1" style={fontMediumBuy}>
                {ex.name}
              </span>
              <span className="shrink-0 opacity-30 group-hover:opacity-100 group-hover:translate-x-[3px] transition-all duration-200">
                <ExternalArrow />
              </span>
            </a>
          ))}
        </div>
        <div className="pb-[12px]" />
      </motion.div>
    </motion.div>,
    document.body,
  );
}

// ─── ORDER price + "Buy ORDER" button ─────────────────────────────────────────
const ORDER_PRICE_CACHE_KEY = "order_price";
const ORDER_PRICE_MAX_AGE = 60_000;

function getCachedPrice(): string | null {
  try {
    const raw = localStorage.getItem(ORDER_PRICE_CACHE_KEY);
    if (!raw) return null;
    const { value, ts } = JSON.parse(raw);
    if (Date.now() - ts < ORDER_PRICE_MAX_AGE) return value;
  } catch {}
  return null;
}

function setCachedPrice(value: string) {
  localStorage.setItem(ORDER_PRICE_CACHE_KEY, JSON.stringify({ value, ts: Date.now() }));
}

function formatUsd(usd: number): string {
  return "$" + usd.toLocaleString("en-US", { minimumFractionDigits: 2, maximumFractionDigits: 3 });
}

function BuyOrderButton({ onClick }: { onClick?: () => void }) {
  return (
    <div
      className="bg-white col-1 content-stretch flex gap-[6px] items-center ml-0 mt-0 p-[12px] relative rounded-[63px] row-1 cursor-pointer hover:opacity-80 transition-opacity"
      style={{ willChange: "opacity" }}
      onClick={onClick}
    >
      <div
        className="flex flex-col justify-center leading-[0] not-italic relative shrink-0 text-[14px] text-black tracking-[0.14px] whitespace-nowrap" style={{ "fontFamily": "'Atyp BL Text', 'Atyp BL', sans-serif", "fontWeight": 600 }}
      >
        <p className="leading-[0.753]">Buy ORDER</p>
      </div>
      <div className="flex items-center justify-center relative shrink-0">
        <div className="flex-none rotate-180">
          <div className="relative size-[14.667px]">
            <div className="absolute inset-[-8.52%]">
              <svg className="block size-full" fill="none" preserveAspectRatio="none" viewBox="0 0 17.1667 17.1667">
                <path d={SVG.buyArrow} stroke="var(--stroke-0, black)" strokeLinecap="round" strokeLinejoin="round" strokeWidth="2.5" />
              </svg>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

function OrderPriceDisplay({ onBuyOrder }: { onBuyOrder?: () => void }) {
  const [price, setPrice] = useState<string | null>(null);

  useEffect(() => {
    const cached = getCachedPrice();
    if (cached) { setPrice(cached); return; }
    const fetchPrice = async () => {
      try {
        const res = await fetch("https://api.coingecko.com/api/v3/simple/price?ids=orderly-network&vs_currencies=usd");
        const data = await res.json();
        const usd = data?.["orderly-network"]?.usd;
        if (usd !== undefined) {
          const formatted = formatUsd(usd);
          setPrice(formatted);
          setCachedPrice(formatted);
        }
      } catch {
        setPrice(getCachedPrice());
      }
    };
    fetchPrice();
  }, []);

  return (
    <div className="flex items-center gap-[16px] relative shrink-0">
      {price && (
        <div
          className="flex flex-col h-[38px] justify-center not-italic relative text-[24px] text-white tracking-[0.24px]" style={{ "fontFamily": "'Atyp BL Text', 'Atyp BL', sans-serif", "fontWeight": 600, transform: "translateZ(0)" }}
        >
          <p className="leading-[0.753] whitespace-pre-wrap">{price}</p>
        </div>
      )}
      <BuyOrderButton onClick={onBuyOrder} />
    </div>
  );
}

// ─── Brandmark ────────────────────────────────────────────────────────────────
function Brandmark() {
  return (
    <div className="col-1 h-[47.94px] ml-0 mt-0 relative row-1 w-[47.998px]">
      <svg className="absolute block size-full" fill="none" preserveAspectRatio="none" viewBox="0 0 47.9978 47.9396">
        <g id="Brandmark">
          <path clipRule="evenodd" d={SVG.brandmark1} fill="var(--fill-0, white)" fillRule="evenodd" />
          <path clipRule="evenodd" d={SVG.brandmark2} fill="var(--fill-0, white)" fillRule="evenodd" />
          <path clipRule="evenodd" d={SVG.brandmark3} fill="var(--fill-0, white)" fillRule="evenodd" />
          <path clipRule="evenodd" d={SVG.brandmark4} fill="var(--fill-0, white)" fillRule="evenodd" />
        </g>
      </svg>
    </div>
  );
}

// ─── Footer left panel ────────────────────────────────────────────────────────
function FooterLeftPanel() {
  const [buyOrderOpen, setBuyOrderOpen] = useState(false);
  return (
    <>
      <div className="bg-[rgba(156,117,255,0.1)] content-stretch flex gap-[9px] items-center leading-[0] mr-[-9px] p-[12px] relative rounded-[63px] shrink-0">
        <div className="grid-cols-[max-content] grid-rows-[max-content] inline-grid place-items-start relative shrink-0">
          <Brandmark />
        </div>
        <OrderPriceDisplay onBuyOrder={() => setBuyOrderOpen(true)} />
      </div>
      <AnimatePresence>
        {buyOrderOpen && <BuyOrderModal onClose={() => setBuyOrderOpen(false)} />}
      </AnimatePresence>
    </>
  );
}

// ─── Footer nav columns ───────────────────────────────────────────────────────
function FooterLinkItem({ label, href, external, target }: FooterNavLink) {
  const opensNewTab = external || target === "_blank";
  return (
    <a
      href={href}
      target={opensNewTab ? "_blank" : "_self"}
      rel={opensNewTab ? "noopener noreferrer" : undefined}
      className="no-underline hover:opacity-80 transition-opacity"
    >
      <div className="content-stretch flex items-start relative shrink-0">
        <p
          className="capitalize leading-[0.753] not-italic relative shrink-0 text-[#9c75ff] text-[14px] tracking-[0.14px]" style={{ "fontFamily": "'Atyp BL Text', 'Atyp BL', sans-serif", "fontWeight": 500 }}
        >
          {label}
        </p>
      </div>
    </a>
  );
}

function FooterColumn({ title, links }: { title: string; links: FooterNavLink[] }) {
  return (
    <div className="content-stretch flex flex-col gap-[24px] items-start relative shrink-0">
      <p
        className="capitalize leading-[0.753] not-italic relative shrink-0 text-[14px] text-white tracking-[0.14px]" style={{ "fontFamily": "'Atyp BL Text', 'Atyp BL', sans-serif", "fontWeight": 700 }}
      >
        {title}
      </p>
      <div className="content-stretch flex flex-col gap-[24px] items-start relative shrink-0">
        {links.map((link) => (
          <FooterLinkItem key={link.label} {...link} />
        ))}
      </div>
    </div>
  );
}

// ─── Status pill + social icons ───────────────────────────────────────────────
function AllSystemsOperational() {
  return (
    <div className="bg-[rgba(156,117,255,0.1)] content-stretch flex gap-[6px] items-center p-[12px] relative rounded-[63px] shrink-0">
      <div className="relative shrink-0 size-[6px]">
        <svg className="absolute block size-full" fill="none" preserveAspectRatio="none" viewBox="0 0 6 6">
          <circle cx="3" cy="3" fill="var(--fill-0, #24AD8F)" r="3" />
        </svg>
      </div>
      <p
        className="leading-[0.753] not-italic relative shrink-0 text-[14px] text-white tracking-[0.14px]" style={{ "fontFamily": "'Atyp BL Text', 'Atyp BL', sans-serif", "fontWeight": 600 }}
      >
        All systems operational.
      </p>
    </div>
  );
}

function SocialIcons() {
  return (
    <div className="content-stretch flex gap-[16px] items-center justify-end relative shrink-0 w-full">
      <a href="https://discord.com/invite/OrderlyNetwork" target="_blank" rel="noopener noreferrer"
        className="overflow-clip relative shrink-0 size-[16px] block hover:opacity-80 transition-opacity">
        <div className="absolute inset-[11%_0_12.78%_0]">
          <svg className="absolute block size-full" fill="none" preserveAspectRatio="none" viewBox="0 0 16 12.1955">
            <path d={SVG.discord} fill="var(--fill-0, white)" />
          </svg>
        </div>
      </a>
      <a href="https://t.me/Orderly_Discussions" target="_blank" rel="noopener noreferrer"
        className="relative shrink-0 size-[16px] block hover:opacity-80 transition-opacity">
        <svg className="absolute block size-full" fill="none" preserveAspectRatio="none" viewBox="0 0 16 16">
          <path d={SVG.telegram} fill="var(--fill-0, white)" />
        </svg>
      </a>
      <a href="https://x.com/OrderlyNetwork" target="_blank" rel="noopener noreferrer"
        className="overflow-clip relative shrink-0 size-[16px] block hover:opacity-80 transition-opacity">
        <div className="absolute inset-[7.93%_4.17%_9.21%_4.17%]">
          <svg className="absolute block size-full" fill="none" preserveAspectRatio="none" viewBox="0 0 14.6667 13.2573">
            <path d={SVG.twitter} fill="var(--fill-0, white)" />
          </svg>
        </div>
      </a>
      <a href="https://www.youtube.com/@Orderly.Network" target="_blank" rel="noopener noreferrer"
        className="relative shrink-0 size-[24px] block hover:opacity-80 transition-opacity">
        <svg viewBox="0 0 24 24" fill="white" className="block size-full">
          <path d="M23.5 6.2a3.02 3.02 0 0 0-2.1-2.1C19.5 3.6 12 3.6 12 3.6s-7.5 0-9.4.5A3.02 3.02 0 0 0 .5 6.2C0 8.1 0 12 0 12s0 3.9.5 5.8a3.02 3.02 0 0 0 2.1 2.1c1.9.5 9.4.5 9.4.5s7.5 0 9.4-.5a3.02 3.02 0 0 0 2.1-2.1c.5-1.9.5-5.8.5-5.8s0-3.9-.5-5.8zM9.6 15.6V8.4l6.3 3.6-6.3 3.6z"/>
        </svg>
      </a>
      <a href="https://www.linkedin.com/company/orderly-network" target="_blank" rel="noopener noreferrer"
        className="overflow-clip relative shrink-0 size-[16px] block hover:opacity-80 transition-opacity">
        <svg className="absolute block size-full" fill="none" preserveAspectRatio="none" viewBox="0 0 16 16">
          <path d={SVG.linkedin} fill="var(--fill-0, white)" />
        </svg>
      </a>
    </div>
  );
}

// ─── FooterSocialRow ─────────────────────────────────────────────────────────
function FooterSocialRow({ gap = 16 }: { gap?: number }) {
  const ARIA = ["Discord", "Telegram", "YouTube", "LinkedIn", "X (Twitter)"];
  const ICONS = [
    <svg key="discord" width="16" height="16" viewBox="0 0 24 24" fill="white"><path d="M20.317 4.37a19.791 19.791 0 0 0-4.885-1.515.074.074 0 0 0-.079.037c-.21.375-.444.864-.608 1.25a18.27 18.27 0 0 0-5.487 0 12.64 12.64 0 0 0-.617-1.25.077.077 0 0 0-.079-.037A19.736 19.736 0 0 0 3.677 4.37a.07.07 0 0 0-.032.027C.533 9.046-.32 13.58.099 18.057.1 18.081.118 18.1.138 18.11a19.9 19.9 0 0 0 5.993 3.03.078.078 0 0 0 .084-.028c.462-.63.874-1.295 1.226-1.994a.076.076 0 0 0-.041-.106 13.107 13.107 0 0 1-1.872-.892.077.077 0 0 1-.008-.128 10.2 10.2 0 0 0 .372-.292.074.074 0 0 1 .077-.01c3.928 1.793 8.18 1.793 12.062 0a.074.074 0 0 1 .078.01c.12.098.246.198.373.292a.077.077 0 0 1-.006.127 12.299 12.299 0 0 1-1.873.892.077.077 0 0 0-.041.107c.36.698.772 1.362 1.225 1.993a.076.076 0 0 0 .084.028 19.839 19.839 0 0 0 6.002-3.03.077.077 0 0 0 .032-.054c.5-5.177-.838-9.674-3.549-13.66a.061.061 0 0 0-.031-.03z" /></svg>,
    <svg key="telegram" width="16" height="16" viewBox="0 0 24 24" fill="white"><path d="M11.944 0A12 12 0 0 0 0 12a12 12 0 0 0 12 12 12 12 0 0 0 12-12A12 12 0 0 0 12 0a12 12 0 0 0-.056 0zm4.962 7.224c.1-.002.321.023.465.14a.506.506 0 0 1 .171.325c.016.093.036.306.02.472-.18 1.898-.962 6.502-1.36 8.627-.168.9-.499 1.201-.82 1.23-.696.065-1.225-.46-1.9-.902-1.056-.693-1.653-1.124-2.678-1.8-1.185-.78-.417-1.21.258-1.91.177-.184 3.247-2.977 3.307-3.23.007-.032.014-.15-.056-.212s-.174-.041-.249-.024c-.106.024-1.793 1.14-5.061 3.345-.48.33-.913.49-1.302.48-.428-.008-1.252-.241-1.865-.44-.752-.245-1.349-.374-1.297-.789.027-.216.325-.437.893-.663 3.498-1.524 5.83-2.529 6.998-3.014 3.332-1.386 4.025-1.627 4.476-1.635z" /></svg>,
    <svg key="youtube" width="20" height="20" viewBox="0 0 24 24" fill="white"><path d="M23.495 6.205a3.007 3.007 0 0 0-2.088-2.088c-1.87-.501-9.396-.501-9.396-.501s-7.507-.01-9.396.501A3.007 3.007 0 0 0 .527 6.205a31.247 31.247 0 0 0-.522 5.805 31.247 31.247 0 0 0 .522 5.783 3.007 3.007 0 0 0 2.088 2.088c1.868.502 9.396.502 9.396.502s7.506 0 9.396-.502a3.007 3.007 0 0 0 2.088-2.088 31.247 31.247 0 0 0 .5-5.783 31.247 31.247 0 0 0-.5-5.805zM9.609 15.601V8.408l6.264 3.602z" /></svg>,
    <svg key="linkedin" width="16" height="16" viewBox="0 0 24 24" fill="white"><path d="M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433a2.062 2.062 0 0 1-2.063-2.065 2.064 2.064 0 1 1 2.063 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z" /></svg>,
    <svg key="twitter" width="16" height="16" viewBox="0 0 24 24" fill="white"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-4.714-6.231-5.401 6.231H2.744l7.73-8.835L1.254 2.25H8.08l4.213 5.567L18.244 2.25zm-1.161 17.52h1.833L7.084 4.126H5.117z" /></svg>,
  ];
  return (
    <div style={{ display: "flex", alignItems: "center", gap }}>
      {SOCIAL_LINKS.map((href, i) => (
        <a key={i} href={href} target="_blank" rel="noopener noreferrer"
          className="hover:opacity-70 transition-opacity flex items-center justify-center"
          aria-label={ARIA[i]}>
          {ICONS[i]}
        </a>
      ))}
    </div>
  );
}

// ─── Shared copyright row ─────────────────────────────────────────────────────
function FooterCopyrightRow({ px = 28 }: { px?: number }) {
  const base: React.CSSProperties = {
    fontSize: 12,
    color: "white",
    letterSpacing: "0.12px",
    textDecoration: "none",
    margin: 0,
  };
  return (
    <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between", padding: `16px ${px}px`, flexWrap: "wrap", gap: 8 }}>
      <p className="" style={{ ...base, "fontFamily": "\'Atyp BL Text\', \'Atyp BL\', sans-serif", "fontWeight": 600 }}>© 2026 Orderly Network</p>
      <div style={{ display: "flex", gap: 20 }}>
        <a href="https://orderly.network/docs/introduction/terms-of-service" className="" style={{ ...base, opacity: 0.8, "fontFamily": "\'Atyp BL Text\', \'Atyp BL\', sans-serif", "fontWeight": 400 }} target="_blank" rel="noopener noreferrer" >Terms of Service</a>
        <a href="https://orderly.network/docs/introduction/privacy-policy"   className="" style={{ ...base, opacity: 0.8, "fontFamily": "\'Atyp BL Text\', \'Atyp BL\', sans-serif", "fontWeight": 400 }} target="_blank" rel="noopener noreferrer" >Privacy Policy</a>
      </div>
    </div>
  );
}

// ─── TabletFooter ─────────────────────────────────────────────────────────────
/**
 * Tablet footer (600–1023px).
 * Purple card with 2×2 nav grid + status/social bar + copyright row.
 */
export function TabletFooter() {
  const NAV_SECTIONS = [
    { heading: "Builders",  links: FOOTER_NAV.Builders },
    { heading: "Ecosystem", links: FOOTER_NAV.Ecosystem },
    { heading: "Traders",   links: FOOTER_NAV.Traders },
    { heading: "About",     links: FOOTER_NAV.About },
  ] as const;

  const fontHeading: React.CSSProperties = {
    fontSize: 13,
    color: "white",
    letterSpacing: "0.13px",
    margin: 0,
  };
  const fontLink: React.CSSProperties = {
    fontSize: 13,
    color: "#9c75ff",
    letterSpacing: "0.13px",
    textDecoration: "none",
    display: "block",
  };

  return (
    <div style={{ width: "100%", paddingTop: 60 }}>
      {/* Purple card */}
      <div style={{ background: "#6700ce", borderRadius: 24, overflow: "hidden", margin: "0 32px" }}>
        {/* ORDER logo + price + Buy ORDER pill */}
        <div style={{ padding: "28px 28px 0", display: "inline-block" }}>
          <FooterLeftPanel />
        </div>

        {/* Nav columns — 2×2 grid */}
        <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: "28px", padding: 28 }}>
          {NAV_SECTIONS.map((section) => (
            <div key={section.heading} style={{ display: "flex", flexDirection: "column", gap: 14 }}>
              <p className="" style={{ ...fontHeading, "fontFamily": "\'Atyp BL Text\', \'Atyp BL\', sans-serif", "fontWeight": 700 }}>{section.heading}</p>
              <div style={{ display: "flex", flexDirection: "column", gap: 12 }}>
                {section.links.map((link) => (
                  <a
                    key={link.label}
                    href={link.href}
                    target={link.external || link.target === "_blank" ? "_blank" : undefined}
                    rel={link.external || link.target === "_blank" ? "noopener noreferrer" : undefined}
                    className="hover:opacity-80 transition-opacity" style={{ ...fontLink, "fontFamily": "\'Atyp BL Text\', \'Atyp BL\', sans-serif", "fontWeight": 500 }}
                  >
                    {link.label}
                  </a>
                ))}
              </div>
            </div>
          ))}
        </div>

        {/* Divider */}
        <div style={{ height: 1, margin: "0 28px", background: "rgba(255,255,255,0.15)" }} />

        {/* Bottom bar: status pill + social icons */}
        <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between", padding: "20px 28px", flexWrap: "wrap", gap: 12 }}>
          <div style={{ display: "flex", alignItems: "center", gap: 6, background: "rgba(156,117,255,0.2)", borderRadius: 63, padding: "8px 12px" }}>
            <div style={{ width: 6, height: 6, borderRadius: "50%", background: "#24AD8F", flexShrink: 0 }} />
            <p className="" style={{ "fontFamily": "'Atyp BL Text', 'Atyp BL', sans-serif", "fontWeight": 600, fontSize: 12, color: "white", letterSpacing: "0.12px", margin: 0 }}>
              All systems operational.
            </p>
          </div>
          <FooterSocialRow />
        </div>
      </div>

      {/* Copyright row */}
      <FooterCopyrightRow px={32} />
    </div>
  );
}

// ─── MobileFooter ─────────────────────────────────────────────────────────────
function FooterSheet({ sectionKey, onClose }: { sectionKey: string; onClose: () => void }) {
  const items = FOOTER_NAV[sectionKey as keyof typeof FOOTER_NAV] ?? [];
  return (
    <div
      className="fixed inset-0 z-[9999] flex items-end"
      style={{ backgroundColor: "rgba(0,0,0,0.6)" }}
      onClick={onClose}
    >
      <motion.div
        initial={{ y: "100%" }}
        animate={{ y: 0 }}
        exit={{ y: "100%" }}
        transition={{ duration: 0.25, ease: [0.22, 0.61, 0.36, 1] }}
        className="w-full rounded-t-[24px] p-[32px] flex flex-col gap-[20px]"
        style={{ background: "#0d0d12" }}
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex items-center justify-between">
          <p className="" style={{ "fontFamily": "'Atyp BL Text', 'Atyp BL', sans-serif", "fontWeight": 700, color: "white", fontSize: 16, letterSpacing: "0.16px", textTransform: "capitalize", margin: 0 }}>
            {sectionKey}
          </p>
          <button onClick={onClose} className="text-[#9c9fae] hover:text-white bg-transparent border-0 cursor-pointer p-0 transition-colors" aria-label="Close">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
              <path d="M6 6l12 12M18 6L6 18" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" />
            </svg>
          </button>
        </div>
        <nav style={{ display: "flex", flexDirection: "column", gap: 16 }}>
          {items.map((item) => (
            <a
              key={item.label}
              href={item.href}
              target={item.external || item.target === "_blank" ? "_blank" : undefined}
              rel={item.external || item.target === "_blank" ? "noopener noreferrer" : undefined}
              onClick={onClose}
              className="hover:opacity-80 transition-opacity" style={{ color: "rgba(255,255,255,0.8)", fontSize: 15, textDecoration: "none", "fontFamily": "\'Atyp BL Text\', \'Atyp BL\', sans-serif", "fontWeight": 500 }}
            >
              {item.label}
            </a>
          ))}
        </nav>
      </motion.div>
    </div>
  );
}

/**
 * Mobile footer (< 600px).
 * Purple card with tappable nav section rows that open a slide-up sheet overlay.
 */
export function MobileFooter() {
  const [footerSheet, setFooterSheet] = useState<string | null>(null);
  const NAV_SECTIONS = Object.keys(FOOTER_NAV);

  return (
    <>
      <div style={{ width: "100%", paddingTop: 60 }}>
        {/* Purple card */}
        <div style={{ background: "#6700ce", borderRadius: 24, overflow: "hidden", margin: "0 16px" }}>
          {/* ORDER logo + price + Buy ORDER pill */}
          <div style={{ padding: "20px 20px 0", display: "inline-block" }}>
            <FooterLeftPanel />
          </div>

          {/* Nav section rows — tap to open slide-up sheet */}
          <div style={{ padding: "4px 20px" }}>
            {NAV_SECTIONS.map((section, i) => (
              <button
                key={section}
                onClick={() => setFooterSheet(section)}
                style={{
                  display: "flex",
                  alignItems: "center",
                  justifyContent: "space-between",
                  width: "100%",
                  padding: "16px 0",
                  background: "transparent",
                  border: "none",
                  borderBottom: i < NAV_SECTIONS.length - 1 ? "1px solid rgba(255,255,255,0.1)" : "none",
                  cursor: "pointer",
                  textAlign: "left",
                }}
              >
                <span className="" style={{ "fontFamily": "'Atyp BL Text', 'Atyp BL', sans-serif", "fontWeight": 600, color: "white", fontSize: 15, letterSpacing: "0.15px" }}>
                  {section}
                </span>
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none">
                  <path d="M9 18l6-6-6-6" stroke="rgba(255,255,255,0.5)" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" />
                </svg>
              </button>
            ))}
          </div>

          {/* Divider */}
          <div style={{ height: 1, margin: "0 20px", background: "rgba(255,255,255,0.15)" }} />

          {/* Bottom bar: status pill + social icons */}
          <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between", padding: "16px 20px", flexWrap: "wrap", gap: 12 }}>
            <div style={{ display: "flex", alignItems: "center", gap: 6, background: "rgba(156,117,255,0.2)", borderRadius: 63, padding: "8px 12px" }}>
              <div style={{ width: 6, height: 6, borderRadius: "50%", background: "#24AD8F", flexShrink: 0 }} />
              <p className="" style={{ "fontFamily": "'Atyp BL Text', 'Atyp BL', sans-serif", "fontWeight": 600, fontSize: 12, color: "white", letterSpacing: "0.12px", margin: 0 }}>
                All systems operational.
              </p>
            </div>
            <FooterSocialRow gap={12} />
          </div>
        </div>

        {/* Copyright row */}
        <FooterCopyrightRow px={16} />
      </div>

      {/* Footer nav sheet overlay — rendered via portal */}
      <AnimatePresence>
        {footerSheet && createPortal(
          <FooterSheet sectionKey={footerSheet} onClose={() => setFooterSheet(null)} />,
          document.body
        )}
      </AnimatePresence>
    </>
  );
}

// ─── SiteFooter ───────────────────────────────────────────────────────────────
export function SiteFooter() {
  return (
    <footer style={{ background: "transparent", paddingTop: 60 }}>
      <div style={{ width: "100%", paddingBottom: 24 }}>
        <div style={{ background: "#6700ce", borderRadius: 30, overflow: "hidden" }}>
          {/* Top row */}
          <div style={{ display: "flex", alignItems: "flex-start", padding: "60px 40px 40px", gap: 40 }}>
            <FooterLeftPanel />
            <div className="content-stretch flex flex-1 flex-col items-end min-h-px min-w-px relative">
              <div className="content-stretch flex gap-[48px] items-start justify-end relative w-full">
                <FooterColumn title="Builders"  links={FOOTER_NAV.Builders} />
                <FooterColumn title="Ecosystem" links={FOOTER_NAV.Ecosystem} />
                <FooterColumn title="Traders"   links={FOOTER_NAV.Traders} />
                <FooterColumn title="About"     links={FOOTER_NAV.About} />
              </div>
            </div>
          </div>

          {/* Bottom bar */}
          <div className="content-stretch flex justify-between items-center px-[60px] relative shrink-0 w-full">
            <AllSystemsOperational />
            <div className="flex-[1_0_0] min-h-px min-w-px relative">
              <div className="flex flex-col items-end size-full">
                <div className="content-stretch flex flex-col items-end p-[8px] relative w-full">
                  <SocialIcons />
                </div>
              </div>
            </div>
          </div>

          <div style={{ height: 24 }} />
        </div>

        {/* Copyright */}
        <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between", padding: "16px 0" }}>
          <p className="" style={{ "fontFamily": "'Atyp BL Text', 'Atyp BL', sans-serif", "fontWeight": 600, fontSize: 12, color: "white", letterSpacing: "0.12px", margin: 0 }}>
            © 2026 Orderly Network
          </p>
          <div style={{ display: "flex", gap: 24 }}>
            <a
              href="https://orderly.network/docs/introduction/terms-of-service"
              target="_blank"
              rel="noopener noreferrer"
              className="" style={{ "fontFamily": "'Atyp BL Text', 'Atyp BL', sans-serif", "fontWeight": 400, fontSize: 12, color: "white", letterSpacing: "0.12px", textDecoration: "none", opacity: 0.8 }}
            >
              Terms of Service
            </a>
            <a
              href="https://orderly.network/docs/introduction/privacy-policy"
              target="_blank"
              rel="noopener noreferrer"
              className="" style={{ "fontFamily": "'Atyp BL Text', 'Atyp BL', sans-serif", "fontWeight": 400, fontSize: 12, color: "white", letterSpacing: "0.12px", textDecoration: "none", opacity: 0.8 }}
            >
              Privacy Policy
            </a>
          </div>
        </div>
      </div>
    </footer>
  );
}
