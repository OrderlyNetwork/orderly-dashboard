import svgPaths from "./svg-zguoq44uln";
const imgImage3 = "/images/svg/CamelotLogo.svg";

export default function CamelotLogo({ className }: { className?: string }) {
  return (
    <div className={className || "relative size-[96px]"} data-name="CamelotLogo">
      <svg className="absolute block size-full" fill="none" preserveAspectRatio="none" viewBox="0 0 96 96">
        <path d={svgPaths.p363b37c0} fill="var(--fill-0, #10151D)" id="base" />
      </svg>
      <div className="absolute aspect-[287/434] left-[26.67%] right-[26.67%] top-[11.51%]" data-name="image 3">
        <img alt="" className="absolute inset-0 max-w-none object-cover pointer-events-none size-full" src={imgImage3} />
      </div>
    </div>
  );
}