import type { SVGProps } from "react";

interface IconProps extends SVGProps<SVGSVGElement> {
  size?: number;
}

function icon(props: IconProps, d: string, fillRule?: "nonzero" | "evenodd") {
  const { size = 20, ...rest } = props;
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth={1.8}
      strokeLinecap="round"
      strokeLinejoin="round"
      {...rest}
    >
      <path d={d} fillRule={fillRule} />
    </svg>
  );
}

export function RouteIcon(p: IconProps) {
  const { size = 20, ...rest } = p;
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth={1.8}
      strokeLinecap="round"
      strokeLinejoin="round"
      {...rest}
    >
      {/* Diamond outline */}
      <rect x="3" y="3" width="18" height="18" rx="3" ry="3"
            transform="rotate(45 12 12)"/>
      {/* Router symbol: 1 input → 3 outputs */}
      <line x1="7" y1="12" x2="10.5" y2="12"/>
      <circle cx="11.5" cy="12" r="1.2" fill="currentColor" stroke="none"/>
      <line x1="12.5" y1="12" x2="17" y2="12"/>
      <line x1="12.5" y1="12" x2="16" y2="8"/>
      <line x1="12.5" y1="12" x2="16" y2="16"/>
      {/* Endpoint dots */}
      <circle cx="18" cy="12" r="1" fill="currentColor" stroke="none"/>
      <circle cx="16.8" cy="7.2" r="1" fill="currentColor" stroke="none"/>
      <circle cx="16.8" cy="16.8" r="1" fill="currentColor" stroke="none"/>
    </svg>
  );
}

export function RefreshIcon(p: IconProps) {
  return icon(p, "M21 12A9 9 0 1 0 17.5 20.5M21 12H16M21 12V7");
}

export function GearIcon(p: IconProps) {
  return icon(
    p,
    "M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6Z M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09a1.65 1.65 0 0 0-1.08-1.51 1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09a1.65 1.65 0 0 0 1.51-1.08 1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9c.26.604.852.997 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1Z"
  );
}

export function CloseIcon(p: IconProps) {
  return icon(p, "M18 6L6 18M6 6l12 12");
}

export function CheckIcon(p: IconProps) {
  return icon(p, "M20 6L9 17l-5-5");
}

export function BotIcon(p: IconProps) {
  return icon(
    p,
    "M12 4V2M9 4a3 3 0 0 1 6 0v4a3 3 0 0 1-6 0V4ZM5 12h14M5 12a3 3 0 0 0-3 3v2a3 3 0 0 0 3 3h14a3 3 0 0 0 3-3v-2a3 3 0 0 0-3-3ZM9 16h.01M15 16h.01"
  );
}

export function DocIcon(p: IconProps) {
  return icon(
    p,
    "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8ZM14 2v6h6M16 13H8M16 17H8M10 9H8"
  );
}

export function FolderIcon(p: IconProps) {
  return icon(
    p,
    "M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2Z"
  );
}

export function SlidersIcon(p: IconProps) {
  return icon(
    p,
    "M4 21v-7M4 10V3M12 21v-9M12 8V3M20 21v-5M20 12V3M1 14h6M9 8h6M17 16h6"
  );
}

export function KeyIcon(p: IconProps) {
  return icon(
    p,
    "M21 2l-2 2m-7.61 7.61a5.5 5.5 0 1 1-7.778 7.778 5.5 5.5 0 0 1 7.777-7.777Zm0 0L15.5 7.5m0 0l3 3L22 7l-3-3m-3.5 3.5L19 4"
  );
}

export function LightbulbIcon(p: IconProps) {
  return icon(
    p,
    "M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 1 1 7.072 0l-.548.547A3.374 3.374 0 0 0 14 18.469V19a2 2 0 1 1-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547Z"
  );
}

export function ListIcon(p: IconProps) {
  return icon(
    p,
    "M8 6h13M8 12h13M8 18h13M3 6h.01M3 12h.01M3 18h.01"
  );
}

export function PersonIcon(p: IconProps) {
  return icon(
    p,
    "M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2M12 11a4 4 0 1 0 0-8 4 4 0 0 0 0 8Z"
  );
}

export function ExternalLinkIcon(p: IconProps) {
  return icon(
    p,
    "M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6M15 3h6v6M10 14L21 3"
  );
}

export function PlusIcon(p: IconProps) {
  return icon(p, "M12 5v14M5 12h14");
}

export function TrashIcon(p: IconProps) {
  return icon(
    p,
    "M3 6h18M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2M10 11v6M14 11v6"
  );
}

export function ProfileIcon(p: IconProps) {
  return icon(
    p,
    "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2M22 21v-2a4 4 0 0 0-3-3.87M16 3.13a4 4 0 0 1 0 7.75M13 7a4 4 0 1 1-8 0 4 4 0 0 1 8 0Z"
  );
}

export function EyeIcon(p: IconProps) {
  return icon(
    p,
    "M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8Z M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6Z"
  );
}

export function EyeOffIcon(p: IconProps) {
  return icon(
    p,
    "M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24 M1 1l22 22"
  );
}

export function ServerIcon(p: IconProps) {
  return icon(
    p,
    "M5 12h14M5 12a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2M5 12a2 2 0 0 0-2 2v4a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-4a2 2 0 0 0-2-2m-8-2h.01M17 16h.01"
  );
}
