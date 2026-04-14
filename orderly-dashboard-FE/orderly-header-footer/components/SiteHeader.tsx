"use client";

/**
 * SiteHeader — standalone Orderly Network header component.
 *
 * Dependencies to copy to a new project:
 *   - src/app/shared/orderly.ts   (nav data)
 *   - npm install motion posthog-js
 *
 * Remove posthog.capture() calls if not using PostHog analytics.
 */

import posthog from "posthog-js";
import { useState, useRef, useEffect, useId } from "react";
import Link from "next/link";
import { AnimatePresence, motion } from "motion/react";
import {
  HEADER_NAV,
  HEADER_CTA,
  DOCS_LINK,
  CAMPAIGNS_LINK,
  CAMPAIGN_ITEMS,
  type HeaderNavItem,
} from "@/app/shared/orderly";

// ─── Inlined SVG paths ────────────────────────────────────────────────────────
const SVG = {
  // Orderly "O" icon — 4 pieces
  logoIcon1:
    "M19.3406 2.64415C19.4575 2.7389 19.3891 2.92382 19.2388 2.92382H4.50814C4.35779 2.92382 4.28944 2.7389 4.40629 2.64415C6.44594 0.990579 9.04408 0 11.8734 0C14.7027 0 17.301 0.990579 19.3406 2.64415Z",
  logoIcon2:
    "M7.25428 15.1146C7.16931 14.9933 7.03261 14.9172 6.88468 14.9172H0.673565C0.531294 14.9172 0.427556 15.0524 0.467075 15.1892C1.89912 20.1412 6.46335 23.7619 11.8724 23.7619C17.2815 23.7619 21.8457 20.1412 23.2777 15.1892C23.3173 15.0524 23.2136 14.9172 23.0713 14.9172H16.8603C16.7124 14.9172 16.5756 14.9933 16.4907 15.1146C15.4712 16.5696 13.7828 17.5208 11.8725 17.5208C9.96217 17.5208 8.27379 16.5696 7.25428 15.1146Z",
  logoIcon3:
    "M16.0489 8.0857C16.1349 8.18045 16.2554 8.23754 16.3833 8.23754H22.8806C23.0277 8.23754 23.1322 8.09406 23.0836 7.95508C22.6251 6.64132 21.9425 5.43265 21.0817 4.37519C21.001 4.27595 20.8792 4.21976 20.7513 4.21976H2.99429C2.86639 4.21976 2.74469 4.27595 2.66386 4.37519C1.80314 5.43265 1.12053 6.64132 0.662017 7.95508C0.613516 8.09406 0.717973 8.23754 0.865003 8.23754H7.36237C7.49017 8.23754 7.6108 8.18045 7.69666 8.0857C8.72794 6.94778 10.217 6.23319 11.8728 6.23319C13.5287 6.23319 15.0177 6.94778 16.0489 8.0857Z",
  logoIcon4:
    "M17.46 13.6234C17.3495 13.6234 17.2711 13.5155 17.3008 13.409C17.4363 12.9245 17.5087 12.4135 17.5087 11.8856C17.5087 11.1428 17.3653 10.4335 17.1047 9.7839C17.0605 9.67395 17.1396 9.55134 17.258 9.55134H23.3815C23.4591 9.55134 23.526 9.60617 23.5403 9.68249C23.6741 10.3965 23.744 11.133 23.744 11.8858C23.744 12.4278 23.7078 12.9612 23.6376 13.484C23.6268 13.5641 23.5582 13.6234 23.4774 13.6234H17.46ZM6.4432 13.409C6.47302 13.5155 6.39461 13.6234 6.28414 13.6234H0.266667C0.185921 13.6234 0.11721 13.5641 0.106522 13.484C0.0362851 12.9612 0 12.4278 0 11.8858C0 11.133 0.0699666 10.3965 0.203704 9.68249C0.218075 9.60617 0.284989 9.55134 0.362591 9.55134H6.48614C6.60451 9.55134 6.68355 9.67395 6.63945 9.7839C6.3788 10.4335 6.23536 11.1428 6.23536 11.8856C6.23536 12.4135 6.30776 12.9245 6.4432 13.409Z",
  // "Orderly" wordmark — 6 letterforms
  logoTextR:
    "M26.6387 7.34344H30.8517V11.3692L30.1651 10.589C30.5604 9.73601 31.1118 9.00784 31.8191 8.40449C32.5265 7.80115 33.3171 7.39546 34.1909 7.18741C35.0647 6.97936 35.9594 7.01056 36.8748 7.28103V11.2444C35.7721 10.8699 34.7527 10.7659 33.8164 10.9323C32.901 11.0987 32.1728 11.494 31.6319 12.1182C31.1118 12.7423 30.8517 13.5121 30.8517 14.4275V23.4465H26.6387V7.34344Z",
  logoTextO:
    "M37.0091 15.3949C37.0091 13.8554 37.342 12.4406 38.0078 11.1507C38.6943 9.86084 39.6306 8.8518 40.8164 8.12363C42.0023 7.39546 43.3131 7.03137 44.7486 7.03137C46.1842 7.03137 47.4533 7.39546 48.5559 8.12363C49.6586 8.8518 50.5012 9.85044 51.0837 11.1195C51.6871 12.3886 51.9888 13.8138 51.9888 15.3949C51.9888 16.9761 51.6871 18.4013 51.0837 19.6704C50.5012 20.9395 49.6586 21.9381 48.5559 22.6663C47.4533 23.3944 46.1842 23.7585 44.7486 23.7585C43.3131 23.7585 42.0023 23.3944 40.8164 22.6663C39.6306 21.9381 38.6943 20.9395 38.0078 19.6704C37.342 18.3805 37.0091 16.9553 37.0091 15.3949ZM41.3782 15.3949C41.3782 16.2271 41.5654 16.9969 41.9399 17.7043C42.3144 18.3909 42.8241 18.9318 43.4691 19.3271C44.114 19.7224 44.8318 19.92 45.6224 19.92C46.413 19.92 47.1308 19.7224 47.7757 19.3271C48.4207 18.9318 48.9304 18.3909 49.3049 17.7043C49.6794 16.9969 49.8666 16.2271 49.8666 15.3949C49.8666 14.5628 49.6794 13.8034 49.3049 13.1168C48.9304 12.4094 48.4207 11.8685 47.7757 11.494C47.1308 11.0987 46.413 10.9011 45.6224 10.9011C44.8318 10.9011 44.114 11.0987 43.4691 11.494C42.8241 11.8685 42.3144 12.4094 41.9399 13.1168C41.5654 13.8034 41.3782 14.5628 41.3782 15.3949ZM49.8666 0.352991H54.0797V23.4465H49.8666V0.352991Z",
  logoTextE:
    "M60.3444 13.6785H70.2684L69.3009 15.0205C69.2593 14.1883 69.0513 13.4393 68.6768 12.7735C68.3023 12.1078 67.7926 11.5876 67.1476 11.2132C66.5235 10.8387 65.8057 10.6514 64.9943 10.6514C64.1829 10.6514 63.4443 10.8595 62.7785 11.2756C62.1128 11.6709 61.5927 12.243 61.2182 12.992C60.8437 13.7202 60.6564 14.5211 60.6564 15.3949C60.6564 16.2896 60.8437 17.101 61.2182 17.8291C61.6135 18.5573 62.1648 19.1294 62.8722 19.5455C63.5795 19.9616 64.3805 20.1697 65.2752 20.1697C66.149 20.1697 66.9292 19.972 67.6157 19.5767C68.3231 19.1606 68.8744 18.5989 69.2697 17.8915L72.7338 19.4519C71.9848 20.7626 70.9549 21.8133 69.6442 22.6039C68.3335 23.3736 66.8355 23.7585 65.1503 23.7585C63.5483 23.7585 62.092 23.3944 60.7813 22.6663C59.4705 21.9381 58.4303 20.9395 57.6605 19.6704C56.9115 18.3805 56.537 16.9553 56.537 15.3949C56.537 13.8554 56.9011 12.4511 57.6293 11.182C58.3783 9.89205 59.3977 8.88301 60.6876 8.15484C61.9776 7.40586 63.4131 7.03137 64.9943 7.03137C66.7627 7.03137 68.3335 7.47868 69.7066 8.37329C71.0798 9.2679 72.0888 10.4746 72.7338 11.9933C73.3995 13.4913 73.5868 15.1245 73.2955 16.8929H60.3444V13.6785Z",
  logoTextR2:
    "M75.8967 7.34344H80.1097V11.3692L79.4232 10.589C79.8185 9.73601 80.3698 9.00784 81.0772 8.40449C81.7846 7.80115 82.5751 7.39546 83.449 7.18741C84.3228 6.97936 85.2174 7.01056 86.1328 7.28103V11.2444C85.0301 10.8699 84.0107 10.7659 83.0745 10.9323C82.159 11.0987 81.4309 11.494 80.8899 12.1182C80.3698 12.7423 80.1097 13.5121 80.1097 14.4275V23.4465H75.8967V7.34344Z",
  logoTextL: "M88.1103 0.352991H92.3234V23.4465H88.1103V0.352991Z",
  logoTextY:
    "M99.0069 23.6649C99.3814 24.4763 99.8703 25.09 100.474 25.5061C101.077 25.943 101.774 26.1615 102.565 26.1615C103.75 26.1615 104.739 25.7454 105.529 24.9132C106.341 24.1018 106.746 23.0512 106.746 21.7613V18.9526L107.402 19.8264C106.882 20.8874 106.122 21.73 105.124 22.3542C104.125 22.9575 102.981 23.2592 101.691 23.2592C100.359 23.2592 99.2046 22.9783 98.2267 22.4166C97.2697 21.8549 96.5311 21.0747 96.011 20.0761C95.4909 19.0774 95.2308 17.9227 95.2308 16.612V7.34344H99.4438V15.8631C99.4438 16.5912 99.5791 17.2258 99.8495 17.7667C100.12 18.2868 100.505 18.7029 101.004 19.015C101.524 19.3063 102.159 19.4519 102.908 19.4519C103.657 19.4519 104.323 19.3063 104.905 19.015C105.509 18.7029 105.966 18.2764 106.278 17.7355C106.59 17.1738 106.746 16.5496 106.746 15.8631V7.34344H110.959V21.7613C110.959 23.3216 110.585 24.726 109.836 25.9742C109.108 27.2433 108.099 28.2316 106.809 28.939C105.54 29.6463 104.125 30 102.565 30C100.942 30 99.4958 29.6047 98.2267 28.8141C96.9576 28.0235 95.9694 26.9729 95.262 25.6622L99.0069 23.6649Z",
  // Dropdown chevron
  chevron: "M0.75 3.72401e-08L0.75 5.5C0.75 5.5 4.59027 5.5 6.25 5.5",
} as const;

// ─── Logo ─────────────────────────────────────────────────────────────────────
export function HeaderLogo() {
  return (
    <Link
      href="/"
      className="h-[29.999px] relative shrink-0 w-[110.959px] block hover:opacity-80 transition-opacity"
      data-name="Header Logo"
    >
      <svg
        className="absolute block size-full"
        fill="none"
        preserveAspectRatio="none"
        viewBox="0 0 110.959 30"
      >
        <g id="Header Logo">
          <g id="Header Logo Icon">
            <path clipRule="evenodd" d={SVG.logoIcon1} fill="var(--fill-0, white)" fillRule="evenodd" />
            <path clipRule="evenodd" d={SVG.logoIcon2} fill="var(--fill-0, white)" fillRule="evenodd" />
            <path clipRule="evenodd" d={SVG.logoIcon3} fill="var(--fill-0, white)" fillRule="evenodd" />
            <path clipRule="evenodd" d={SVG.logoIcon4} fill="var(--fill-0, white)" fillRule="evenodd" />
          </g>
          <g id="Orderly">
            <path d={SVG.logoTextR}  fill="var(--fill-0, white)" />
            <path d={SVG.logoTextO}  fill="var(--fill-0, white)" />
            <path d={SVG.logoTextE}  fill="var(--fill-0, white)" />
            <path d={SVG.logoTextR2} fill="var(--fill-0, white)" />
            <path d={SVG.logoTextL}  fill="var(--fill-0, white)" />
            <path d={SVG.logoTextY}  fill="var(--fill-0, white)" />
          </g>
        </g>
      </svg>
    </Link>
  );
}

function HeaderLogoContainer() {
  return (
    <div
      className="content-stretch flex flex-col h-[50px] items-start justify-center relative shrink-0"
      data-name="Header Logo Container"
    >
      <HeaderLogo />
    </div>
  );
}

// ─── Chevron ─────────────────────────────────────────────────────────────────
function ChevronIcon({ open }: { open: boolean }) {
  return (
    <div
      className={`overflow-clip relative shrink-0 size-[12px] transition-transform duration-200 ${open ? "rotate-180" : ""}`}
      data-name="Header Menu Item Icon"
    >
      <div className="-translate-y-1/2 absolute flex h-[7.778px] items-center justify-center left-[17.68%] right-[17.5%] top-[calc(50%-1.63px)]">
        <div className="-rotate-45 flex-none size-[5.5px]">
          <div className="relative size-full">
            <div className="absolute inset-[0_0_-13.64%_-13.64%]">
              <svg className="block size-full" fill="none" preserveAspectRatio="none" viewBox="0 0 6.25 6.25">
                <path d={SVG.chevron} stroke="var(--stroke-0, white)" strokeWidth="1.5" />
              </svg>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

// ─── Dropdown helpers ─────────────────────────────────────────────────────────
function labelToKey(label: string) {
  return label.toLowerCase().replace(/\s+/g, "_");
}

function useMenuHover() {
  const [open, setOpen] = useState(false);
  const timer = useRef<ReturnType<typeof setTimeout> | null>(null);
  const handleEnter = () => {
    if (timer.current) clearTimeout(timer.current);
    setOpen(true);
  };
  const handleLeave = () => {
    timer.current = setTimeout(() => setOpen(false), 120);
  };
  return { open, handleEnter, handleLeave };
}

function DropdownPanel({
  items,
  onEnter,
  onLeave,
  groupKey,
}: {
  items: HeaderNavItem[];
  onEnter: () => void;
  onLeave: () => void;
  groupKey: string;
}) {
  return (
    <motion.div
      initial={{ opacity: 0, y: -6, scale: 0.97 }}
      animate={{ opacity: 1, y: 0, scale: 1 }}
      exit={{ opacity: 0, y: -6, scale: 0.97 }}
      transition={{ duration: 0.18, ease: [0.22, 0.61, 0.36, 1] }}
      className="absolute top-full left-0 z-50 pt-[4px]"
      onMouseEnter={onEnter}
      onMouseLeave={onLeave}
    >
      <div className="bg-[#3f0086] flex flex-col gap-[15px] items-start justify-center px-[15px] py-[20px] rounded-[8px] min-w-[160px]">
        {items.map((item) => {
          const opensNewTab = item.external === true || item.target === "_blank";
          const isInternal = item.href.startsWith("/");
          const useClientLink = isInternal && !opensNewTab;
          const linkClass =
            "content-stretch flex items-center relative shrink-0 w-full no-underline group";
          const inner = (
            <p
              className="font-['Atyp_BL:Medium',sans-serif] font-medium leading-[1.6] relative shrink-0 text-[14px] text-white group-hover:text-purple-300 transition-colors"
              style={{ letterSpacing: "0.042em" }}
            >
              {item.label}
            </p>
          );
          const trackClick = () =>
            posthog.capture("header_nav_clicked", {
              tab_name: `${groupKey}_${labelToKey(item.label)}`,
              source_page: "homepage",
              device_layout: "desktop",
              section: "header",
            });
          return useClientLink ? (
            <Link key={item.label} href={item.href} className={linkClass} onClick={trackClick}>
              {inner}
            </Link>
          ) : (
            <a
              key={item.label}
              href={item.href}
              target="_blank"
              rel="noopener noreferrer"
              className={linkClass}
              onClick={trackClick}
            >
              {inner}
            </a>
          );
        })}
      </div>
    </motion.div>
  );
}

// ─── Campaigns icon + dropdown ────────────────────────────────────────────────
export function CampaignIcon({ className = "" }: { className?: string }) {
  const gradientId = useId().replace(/:/g, "");
  return (
    <svg
      width="8"
      height="11"
      viewBox="0 0 8 11"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      className={className}
      style={{ transform: "scale(1.3)" }}
      aria-hidden
    >
      <path
        d="M0.000489424 6.785C-0.00634591 5.73247 0.369598 4.79952 1.04416 3.96991C1.42095 3.50697 1.85329 3.09279 2.25144 2.64902C2.571 2.29275 2.86363 1.91857 3.04264 1.47147C3.21224 1.04646 3.24727 0.609775 3.17293 0.162259C3.1708 0.148508 3.16695 0.135174 3.16567 0.121423C3.16268 0.0805887 3.14986 0.0372538 3.19814 0.00975272C3.24086 -0.0144149 3.27205 0.0114195 3.30494 0.0322536C3.55742 0.190176 3.7911 0.371433 4.00513 0.576441C4.59553 1.14105 4.96593 1.81441 5.07828 2.61985C5.13553 3.02987 5.0817 3.42863 4.99454 3.82781C4.94627 4.04866 4.90441 4.27283 4.95097 4.49826C5.03556 4.90702 5.44995 5.2287 5.87587 5.22453C6.31975 5.22037 6.70895 4.90577 6.79608 4.48201C6.81788 4.37533 6.81828 4.26825 6.81362 4.16075C6.81149 4.11074 6.79008 4.04991 6.85928 4.02657C6.92808 4.00365 6.95115 4.06616 6.98148 4.10491C7.32455 4.54409 7.55695 5.03536 7.70222 5.56705C8.00549 6.67873 7.92308 7.75047 7.30488 8.74887C6.63588 9.82927 5.65757 10.4797 4.36698 10.6381C2.34159 10.8869 0.414878 9.48427 0.0632914 7.52507C0.0201408 7.2846 -0.00378258 7.0438 0.000489424 6.785ZM3.46813 5.80247C3.47711 5.6558 3.48565 5.51038 3.49505 5.36496C3.49804 5.32121 3.49718 5.27995 3.44848 5.25953C3.4032 5.24037 3.37458 5.27121 3.34638 5.29704C3.02469 5.59788 2.71966 5.91248 2.47018 6.27457C2.05706 6.8742 1.86653 7.5346 1.99426 8.24967C2.13012 9.01133 2.60047 9.53133 3.3562 9.76013C4.1239 9.9926 4.81298 9.80473 5.36322 9.23553C5.91048 8.66967 6.04209 7.99753 5.78915 7.25793C5.74173 7.1192 5.70798 7.1138 5.58879 7.21087C4.94157 7.7388 3.97566 7.52673 3.62279 6.78C3.4754 6.46791 3.47241 6.13498 3.46813 5.80247Z"
        fill={`url(#${gradientId})`}
      />
      <defs>
        <linearGradient id={gradientId} x1="7.87286" y1="9.62963" x2="6.93561e-05" y2="9.60621" gradientUnits="userSpaceOnUse">
          <stop stopColor="#48BDFF" />
          <stop offset="0.479167" stopColor="#786CFF" />
          <stop offset="1" stopColor="#BD00FF" />
        </linearGradient>
      </defs>
    </svg>
  );
}

function CampaignsDropdownPanel({ onEnter, onLeave }: { onEnter: () => void; onLeave: () => void }) {
  const items = CAMPAIGN_ITEMS;
  return (
    <motion.div
      className="absolute top-full left-0 z-50 pt-[8px]"
      onMouseEnter={onEnter}
      onMouseLeave={onLeave}
      initial={{ opacity: 0, y: -6, scale: 0.98 }}
      animate={{ opacity: 1, y: 0, scale: 1 }}
      exit={{ opacity: 0, y: -6, scale: 0.98 }}
      transition={{ duration: 0.16, ease: [0.22, 0.61, 0.36, 1] }}
    >
      <div className="w-[320px] rounded-lg bg-[#38235D33] p-2 backdrop-blur-[25px]">
        <div className="flex flex-col gap-[16px] rounded-[8px] bg-[#1e122f] p-[16px] shadow-[4px_4px_24px_0px_rgba(17,6,33,0.8)]">
          {items.map((item) => (
            <a
              key={item.href}
              href={item.href}
              target="_blank"
              rel="noopener noreferrer"
              className="relative h-[80px] w-full overflow-hidden rounded-[8px] border no-underline transition-opacity hover:opacity-90"
              onClick={() =>
                posthog.capture("header_nav_clicked", {
                  tab_name: `campaigns_${item.titleVariant}`,
                  source_page: "homepage",
                  device_layout: "desktop",
                  section: "header",
                })
              }
            >
              <div className="absolute inset-0 opacity-20 overflow-hidden pointer-events-none rounded-[8px]" aria-hidden>
                <img alt="" className={item.backgroundImageClassName} src={item.backgroundImageSrc} />
              </div>
              <div aria-hidden className={item.borderClassName} />
              <div className="flex flex-row items-center size-full">
                <div className="content-stretch flex gap-[8px] items-center p-[16px] relative size-full flex-1">
                  {item.titleVariant === "perps" ? (
                    <div className="basis-0 content-stretch flex gap-[16px] grow items-center min-h-px min-w-px relative shrink-0">
                      <div className="basis-0 content-stretch flex flex-col gap-[4px] grow items-start justify-center min-h-px min-w-px not-italic relative shrink-0">
                        <p className="font-['Atyp_BL:Medium',sans-serif] font-medium leading-[1.6] relative shrink-0 text-[14px] text-[rgba(255,255,255,0.98)] w-full flex items-center gap-1" style={{ letterSpacing: "0.042em" }}>
                          I{" "}
                          <img src="/campaigns/icons/heart-o.svg" alt="heart" className="h-[1em] w-[1em] inline" />{" "}
                          Perps Competition
                        </p>
                        <p className="font-['Atyp_BL:Medium',sans-serif] font-medium leading-[1.6] relative shrink-0 text-[12px] text-[rgba(255,255,255,0.5)] w-full">
                          {item.subtitle}
                        </p>
                      </div>
                    </div>
                  ) : (
                    <div className="basis-0 content-stretch flex gap-[16px] grow items-center min-h-px min-w-px relative shrink-0">
                      <p className="font-['Atyp_BL:Medium',sans-serif] font-medium leading-[1.6] h-full not-italic relative shrink-0 text-[14px] text-[rgba(255,255,255,0.5)] w-[256px]" style={{ letterSpacing: "0.042em" }}>
                        Ultimate Crypto Championship
                      </p>
                    </div>
                  )}
                  {item.status === "Ongoing" ? (
                    <div
                      className="absolute content-stretch flex items-center justify-center px-[8px] py-[4px] right-0 rounded-bl-[8px] rounded-tr-[8px] top-0"
                      style={{ backgroundImage: "linear-gradient(-89.3303deg, rgb(72, 189, 255) 0%, rgb(120, 108, 255) 47.763%, rgb(189, 0, 255) 99.638%)" }}
                    >
                      <p className="font-['Atyp_BL:Medium',sans-serif] font-medium leading-[1.6] not-italic relative shrink-0 text-[10px] text-[rgba(255,255,255,0.98)] text-nowrap">Ongoing</p>
                    </div>
                  ) : (
                    <div className="absolute bg-[rgba(255,255,255,0.36)] content-stretch flex items-center justify-center px-[8px] py-[4px] right-0 rounded-bl-[8px] rounded-tr-[8px] top-0">
                      <p className="font-['Atyp_BL:Medium',sans-serif] font-medium leading-[1.6] not-italic relative shrink-0 text-[10px] text-[rgba(255,255,255,0.98)] text-nowrap">Ended</p>
                    </div>
                  )}
                </div>
              </div>
            </a>
          ))}
        </div>
      </div>
    </motion.div>
  );
}

function CampaignsMenuCell() {
  const { open, handleEnter, handleLeave } = useMenuHover();
  const [isHovered, setIsHovered] = useState(false);
  const shouldOpen = open || isHovered;
  return (
    <div
      className="content-stretch relative flex h-[50px] shrink-0 cursor-pointer items-center gap-[2px]"
      onMouseEnter={() => { setIsHovered(true); handleEnter(); }}
      onMouseLeave={() => { setIsHovered(false); handleLeave(); }}
    >
      <a
        href={CAMPAIGNS_LINK.href}
        target="_blank"
        rel="noopener noreferrer"
        className="relative flex h-[32px] items-center justify-center gap-[6px] rounded-full px-[14px] no-underline overflow-hidden"
        onClick={() =>
          posthog.capture("header_nav_clicked", {
            tab_name: "campaigns",
            source_page: "homepage",
            device_layout: "desktop",
            section: "header",
          })
        }
      >
        <span
          className="absolute inset-0 rounded-full"
          style={{
            background: "linear-gradient(90deg, #48bdff, #786cff, #bd00ff, #48bdff)",
            backgroundSize: "300% 100%",
            animation: "gradient-border 3s linear infinite",
            padding: "1px",
            WebkitMask: "linear-gradient(#fff 0 0) content-box, linear-gradient(#fff 0 0)",
            WebkitMaskComposite: "xor",
            maskComposite: "exclude",
          }}
        />
        <CampaignIcon className="mr-1 relative z-10" />
        <span className="font-['Atyp_BL:Semibold',sans-serif] font-semibold text-[16px] leading-none not-italic text-white relative z-10" style={{ letterSpacing: "0.042em" }}>
          Campaigns
        </span>
        <span className="relative z-10">
          <ChevronIcon open={shouldOpen} />
        </span>
      </a>
      <AnimatePresence>
        {shouldOpen && <CampaignsDropdownPanel onEnter={handleEnter} onLeave={handleLeave} />}
      </AnimatePresence>
    </div>
  );
}

// ─── Nav menu cells ───────────────────────────────────────────────────────────
function NavMenuCell({ label, groupKey, items }: { label: string; groupKey: string; items: HeaderNavItem[] }) {
  const { open, handleEnter, handleLeave } = useMenuHover();
  return (
    <div
      className="content-stretch flex gap-[2px] h-[50px] items-baseline pb-[10px] pt-[16px] relative shrink-0 cursor-pointer"
      onMouseEnter={handleEnter}
      onMouseLeave={handleLeave}
    >
      <div className="content-stretch flex flex-col items-start justify-center relative shrink-0">
        <div className="content-stretch flex gap-[2px] items-center relative shrink-0">
          <p className="font-['Atyp_BL:Semibold',sans-serif] font-semibold leading-none not-italic relative shrink-0 text-[16px] text-center text-white" style={{ letterSpacing: "0.042em" }}>
            {label}
          </p>
        </div>
      </div>
      <ChevronIcon open={open} />
      <AnimatePresence>
        {open && <DropdownPanel onEnter={handleEnter} onLeave={handleLeave} groupKey={groupKey} items={items} />}
      </AnimatePresence>
    </div>
  );
}

function DocsMenuCell() {
  return (
    <a
      href={DOCS_LINK.href}
      target="_blank"
      rel="noopener noreferrer"
      className="content-stretch flex h-[50px] items-baseline pb-[10px] pt-[16px] relative shrink-0 no-underline"
      onClick={() =>
        posthog.capture("header_nav_clicked", {
          tab_name: "docs",
          source_page: "homepage",
          device_layout: "desktop",
          section: "header",
        })
      }
    >
      <div className="content-stretch flex flex-col items-start justify-center relative shrink-0">
        <div className="content-stretch flex gap-[2px] items-center relative shrink-0">
          <p className="font-['Atyp_BL:Semibold',sans-serif] font-semibold leading-none not-italic relative shrink-0 text-[16px] text-center text-white">
            Docs
          </p>
        </div>
      </div>
    </a>
  );
}

function HeaderMenu() {
  return (
    <div className="content-stretch flex flex-[1_0_0] gap-[48px] items-center justify-end min-h-px min-w-px relative">
      <NavMenuCell label="Builders"  groupKey="builders"  items={HEADER_NAV.Builders} />
      <NavMenuCell label="Ecosystem" groupKey="ecosystem" items={HEADER_NAV.Ecosystem} />
      <NavMenuCell label="Traders"   groupKey="traders"   items={HEADER_NAV.Traders} />
      <DocsMenuCell />
      <CampaignsMenuCell />

      <a
        href={HEADER_CTA.href}
        target="_blank"
        rel="noopener noreferrer"
        className="content-stretch flex h-[40px] items-center justify-center px-[20px] py-[12px] relative rounded-[46px] shrink-0 no-underline hover:opacity-90 transition-opacity"
        onClick={() =>
          posthog.capture("homepage_cta_clicked", {
            cta_name: "launch_now",
            source_page: "homepage",
            device_layout: "desktop",
          })
        }
        style={{
          backgroundImage:
            "linear-gradient(90deg, rgb(255, 255, 255) 0%, rgb(255, 255, 255) 100%), linear-gradient(-89.1975deg, rgb(72, 189, 255) 0%, rgb(120, 108, 255) 47.763%, rgb(189, 0, 255) 99.638%)",
        }}
      >
        <p
          className="font-['Atyp_BL:Bold',sans-serif] leading-none not-italic relative shrink-0 text-[#3f0086] text-[16px] tracking-[0.16px]"
          style={{ fontFeatureSettings: "'ss03', 'ss02', 'ss05'" }}
        >
          {HEADER_CTA.label}
        </p>
      </a>
    </div>
  );
}

// ─── MorphingHeader: hides on scroll, reappears when scroll stops ─────────────
export function MorphingHeader() {
  const [isVisible, setIsVisible] = useState(true);
  const [isScrolled, setIsScrolled] = useState(false);
  const scrollTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const lastScrollY = useRef(0);
  const TOP_THRESHOLD = 150;

  useEffect(() => {
    const handleScroll = () => {
      const currentScrollY = window.scrollY;
      setIsScrolled(currentScrollY > 50);
      if (currentScrollY < TOP_THRESHOLD) {
        setIsVisible(true);
        lastScrollY.current = currentScrollY;
        return;
      }
      if (Math.abs(currentScrollY - lastScrollY.current) > 5) {
        setIsVisible(false);
      }
      lastScrollY.current = currentScrollY;
      if (scrollTimerRef.current) clearTimeout(scrollTimerRef.current);
      scrollTimerRef.current = setTimeout(() => setIsVisible(true), 1000);
    };
    window.addEventListener("scroll", handleScroll, { passive: true });
    return () => {
      window.removeEventListener("scroll", handleScroll);
      if (scrollTimerRef.current) clearTimeout(scrollTimerRef.current);
    };
  }, []);

  return (
    <motion.header
      className="flex h-[72px] items-center w-full"
      style={{
        borderRadius: 80,
        maxWidth: 1100,
        marginTop: 24,
        paddingLeft: 24,
        paddingRight: 20,
        gap: 40,
        backdropFilter: "blur(20px) saturate(180%)",
        WebkitBackdropFilter: "blur(20px) saturate(180%)",
        backgroundColor: isScrolled ? "rgba(0, 0, 0, 0.3)" : "rgba(0, 0, 0, 0.15)",
        border: "none",
        boxShadow: isScrolled ? "0 8px 32px rgba(0,0,0,0.3)" : "0 4px 24px rgba(0,0,0,0.15)",
      }}
      initial={{ y: 0, opacity: 1 }}
      animate={{ y: isVisible ? 0 : -100, opacity: isVisible ? 1 : 0 }}
      transition={{ duration: 0.6, ease: [0.22, 0.61, 0.36, 1] }}
    >
      <div className="flex flex-col justify-center shrink-0" style={{ transform: "scale(0.85)", transformOrigin: "left center" }}>
        <HeaderLogo />
      </div>
      <div className="flex-1">
        <HeaderMenu />
      </div>
    </motion.header>
  );
}
