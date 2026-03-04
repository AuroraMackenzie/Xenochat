import { useMemo, useState } from "react";

type PlatformStatus = {
  name: string;
  mode: "L1" | "L2" | "L3";
  state: "ready" | "planned";
};

const platformStatus: PlatformStatus[] = [
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
    return platformStatus.filter((platform) =>
      platform.name.toLowerCase().includes(normalized)
    );
  }, [query]);

  const readyCount = filtered.filter((item) => item.state === "ready").length;

  return (
    <div className="app-shell">
      <div className="orb orb-left" />
      <div className="orb orb-right" />

      <header className="hero">
        <img className="brand-icon" src="/crescent.png" alt="Xenochat" />
        <div>
          <p className="brand-kicker">Original Multi-platform AI Bot</p>
          <h1>Xenochat Operations Deck</h1>
          <p className="brand-copy">
            Rust-first orchestration, bounded queue adapters, strict security defaults, and
            Apple Silicon Metal acceleration.
          </p>
        </div>
      </header>

      <main className="grid-layout">
        <section className="panel panel-spotlight">
          <h2>Runtime Profile</h2>
          <div className="stat-row">
            <span>Backend</span>
            <strong>Rust Workspace</strong>
          </div>
          <div className="stat-row">
            <span>GPU Path</span>
            <strong>Metal + CPU fallback</strong>
          </div>
          <div className="stat-row">
            <span>API Baseline</span>
            <strong>127.0.0.1, strict CORS</strong>
          </div>
          <div className="stat-row">
            <span>Package Manager</span>
            <strong>pnpm only</strong>
          </div>
        </section>

        <section className="panel">
          <div className="panel-head">
            <h2>Platform Coverage</h2>
            <span className="badge">{readyCount} ready</span>
          </div>
          <input
            className="search"
            value={query}
            onChange={(event) => setQuery(event.target.value)}
            placeholder="Filter platform"
            aria-label="Filter platform"
          />
          <ul className="platform-list">
            {filtered.map((platform) => (
              <li key={platform.name}>
                <span>{platform.name}</span>
                <div className="tags">
                  <span className="mode-tag">{platform.mode}</span>
                  <span
                    className={platform.state === "ready" ? "state-tag ready" : "state-tag"}
                  >
                    {platform.state}
                  </span>
                </div>
              </li>
            ))}
          </ul>
        </section>

        <section className="panel panel-wide">
          <h2>Execution Standards</h2>
          <div className="timeline">
            <article>
              <h3>MVP</h3>
              <p>
                Core runtime, secure API baseline, shared adapter contract, and first delivery set
                for Telegram, Discord, Slack, and Teams.
              </p>
            </article>
            <article>
              <h3>Beta</h3>
              <p>
                Expanded realtime adapters, stronger plugin lifecycle controls, memory quality, and
                operational telemetry.
              </p>
            </article>
            <article>
              <h3>GA</h3>
              <p>
                Full platform matrix completion, lifecycle governance, and compliance gate evidence
                for each adapter release stream.
              </p>
            </article>
          </div>
        </section>
      </main>
    </div>
  );
}
