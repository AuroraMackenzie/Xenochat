import { jsx as _jsx, jsxs as _jsxs } from "react/jsx-runtime";
import { useMemo, useState } from "react";
const platformStatus = [
    { name: "Telegram", mode: "L3", state: "ready" },
    { name: "Discord", mode: "L3", state: "ready" },
    { name: "Slack", mode: "L2", state: "ready" },
    { name: "Teams", mode: "L2", state: "ready" },
    { name: "LINE", mode: "L2", state: "planned" },
    { name: "Messenger", mode: "L2", state: "planned" },
    { name: "Viber", mode: "L2", state: "planned" },
    { name: "Google Chat", mode: "L2", state: "planned" },
    { name: "QQ", mode: "L1", state: "planned" },
    { name: "WeChat", mode: "L1", state: "planned" },
    { name: "WhatsApp", mode: "L2", state: "planned" },
    { name: "Zoom", mode: "L1", state: "planned" },
    { name: "Signal", mode: "L1", state: "planned" },
    { name: "Skype", mode: "L1", state: "planned" },
    { name: "iMessage", mode: "L1", state: "planned" },
    { name: "Instagram", mode: "L1", state: "planned" },
    { name: "KakaoTalk", mode: "L1", state: "planned" },
];
export function App() {
    const [query, setQuery] = useState("");
    const filtered = useMemo(() => {
        const normalized = query.trim().toLowerCase();
        if (!normalized) {
            return platformStatus;
        }
        return platformStatus.filter((platform) => platform.name.toLowerCase().includes(normalized));
    }, [query]);
    const readyCount = filtered.filter((item) => item.state === "ready").length;
    return (_jsxs("div", { className: "app-shell", children: [_jsx("div", { className: "orb orb-left" }), _jsx("div", { className: "orb orb-right" }), _jsxs("header", { className: "hero", children: [_jsx("img", { className: "brand-icon", src: "/crescent.png", alt: "Xenochat" }), _jsxs("div", { children: [_jsx("p", { className: "brand-kicker", children: "Original Multi-platform AI Bot" }), _jsx("h1", { children: "Xenochat Operations Deck" }), _jsx("p", { className: "brand-copy", children: "Rust-first orchestration, bounded queue adapters, strict security defaults, and Apple Silicon Metal acceleration." })] })] }), _jsxs("main", { className: "grid-layout", children: [_jsxs("section", { className: "panel panel-spotlight", children: [_jsx("h2", { children: "Runtime Profile" }), _jsxs("div", { className: "stat-row", children: [_jsx("span", { children: "Backend" }), _jsx("strong", { children: "Rust Workspace" })] }), _jsxs("div", { className: "stat-row", children: [_jsx("span", { children: "GPU Path" }), _jsx("strong", { children: "Metal + CPU fallback" })] }), _jsxs("div", { className: "stat-row", children: [_jsx("span", { children: "API Baseline" }), _jsx("strong", { children: "127.0.0.1, strict CORS" })] }), _jsxs("div", { className: "stat-row", children: [_jsx("span", { children: "Package Manager" }), _jsx("strong", { children: "pnpm only" })] })] }), _jsxs("section", { className: "panel", children: [_jsxs("div", { className: "panel-head", children: [_jsx("h2", { children: "Platform Coverage" }), _jsxs("span", { className: "badge", children: [readyCount, " ready"] })] }), _jsx("input", { className: "search", value: query, onChange: (event) => setQuery(event.target.value), placeholder: "Filter platform", "aria-label": "Filter platform" }), _jsx("ul", { className: "platform-list", children: filtered.map((platform) => (_jsxs("li", { children: [_jsx("span", { children: platform.name }), _jsxs("div", { className: "tags", children: [_jsx("span", { className: "mode-tag", children: platform.mode }), _jsx("span", { className: platform.state === "ready" ? "state-tag ready" : "state-tag", children: platform.state })] })] }, platform.name))) })] }), _jsxs("section", { className: "panel panel-wide", children: [_jsx("h2", { children: "Execution Standards" }), _jsxs("div", { className: "timeline", children: [_jsxs("article", { children: [_jsx("h3", { children: "MVP" }), _jsx("p", { children: "Core runtime, secure API baseline, shared adapter contract, and first delivery set for Telegram, Discord, Slack, and Teams." })] }), _jsxs("article", { children: [_jsx("h3", { children: "Beta" }), _jsx("p", { children: "Expanded realtime adapters, stronger plugin lifecycle controls, memory quality, and operational telemetry." })] }), _jsxs("article", { children: [_jsx("h3", { children: "GA" }), _jsx("p", { children: "Full platform matrix completion, lifecycle governance, and compliance gate evidence for each adapter release stream." })] })] })] })] })] }));
}
