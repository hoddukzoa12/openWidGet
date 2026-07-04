import { invoke } from "@tauri-apps/api/core";
import "./styles.css";

type AppStatus = {
  app_name: string;
  version: string;
  mode: "shell-preview";
  widgets: WidgetPreview[];
};

type WidgetPreview = {
  id: string;
  name: string;
  size: string;
  status: "bundled-preview";
};

const fallbackStatus: AppStatus = {
  app_name: "OpenWidGet",
  version: "0.1.0-alpha.0",
  mode: "shell-preview",
  widgets: [
    { id: "clock", name: "Clock", size: "2×2", status: "bundled-preview" },
    { id: "memo", name: "Memo", size: "2×2", status: "bundled-preview" },
    { id: "launcher", name: "Project Launcher", size: "4×2", status: "bundled-preview" }
  ]
};

const root = document.querySelector<HTMLDivElement>("#app");

if (!root) {
  throw new Error("#app root not found");
}

const appRoot: HTMLDivElement = root;

function render(status: AppStatus) {
  const widgetCards = status.widgets
    .map(
      (widget) => `
        <article class="widget-card" data-widget-id="${widget.id}">
          <div class="widget-card__meta">${widget.size} · ${widget.status}</div>
          <h3>${widget.name}</h3>
          <p>${widget.id === "clock" ? currentTime() : widget.id === "memo" ? "Plan, build, verify." : "Open project links fast."}</p>
        </article>
      `
    )
    .join("");

  appRoot.innerHTML = `
    <section class="shell">
      <aside class="rail" aria-label="OpenWidGet status">
        <div class="brand-mark">OW</div>
        <div>
          <p class="eyebrow">v${status.version}</p>
          <h1>${status.app_name}</h1>
          <p class="muted">Hackable Windows desktop widgets powered by HTML/CSS/JS.</p>
        </div>
      </aside>

      <section class="hero">
        <p class="eyebrow">Tauri shell preview</p>
        <h2>Desktop widgets should be small, local-first, and easy to hack.</h2>
        <p>
          This first skeleton proves the app shell, Tauri bridge, and bundled-widget preview surface before the real Windows overlay/runtime work starts.
        </p>
        <div class="actions">
          <button id="refresh-status" type="button">Refresh status</button>
          <span class="pill">Mode: ${status.mode}</span>
        </div>
      </section>

      <section class="desktop-preview" aria-label="Widget preview">
        <div class="desktop-preview__grid">
          ${widgetCards}
        </div>
      </section>
    </section>
  `;

  document.querySelector<HTMLButtonElement>("#refresh-status")?.addEventListener("click", loadStatus);
}

function currentTime() {
  return new Intl.DateTimeFormat(undefined, {
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit"
  }).format(new Date());
}

async function loadStatus() {
  try {
    const status = await invoke<AppStatus>("get_app_status");
    render(status);
  } catch (error) {
    console.warn("Falling back to browser preview status:", error);
    render(fallbackStatus);
  }
}

void loadStatus();
setInterval(() => {
  const clockCard = document.querySelector<HTMLElement>('[data-widget-id="clock"] p');
  if (clockCard) clockCard.textContent = currentTime();
}, 1000);
