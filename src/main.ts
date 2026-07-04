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

type AnchorLifecycleStatus = {
  platform: string;
  enabled: boolean;
  session_id: string;
  desktop_dir: string;
  state_dir: string;
  anchors_planned: number;
  anchors_created: number;
  stale_detected: number;
  stale_deleted: number;
  anchor_files: string[];
  last_error: string | null;
};

type PermissionKind = "time" | "network" | "filesystem" | "system" | "process";

type RuntimeWidget = {
  id: string;
  name: string;
  size: string;
  grid: string;
  accent: "mint" | "sky" | "violet" | "amber";
  anchorCount: number;
  manifest: string;
  body: string;
  metric: string;
  status: string;
  permissions: Array<{ kind: PermissionKind; label: string }>;
  actions: string[];
};

type DesktopIcon = {
  name: string;
  glyph: string;
  gridColumn: string;
  gridRow: string;
};

const fallbackStatus: AppStatus = {
  app_name: "OpenWidGet",
  version: "0.1.0-alpha.0",
  mode: "shell-preview",
  widgets: [
    { id: "clock", name: "Clock", size: "2×2", status: "bundled-preview" },
    { id: "weather", name: "Weather", size: "2×2", status: "bundled-preview" },
    { id: "launcher", name: "Project Launcher", size: "4×2", status: "bundled-preview" },
    { id: "system", name: "System Monitor", size: "2×2", status: "bundled-preview" }
  ]
};

const fallbackAnchorLifecycle: AnchorLifecycleStatus = {
  platform: "browser-preview",
  enabled: false,
  session_id: "preview-only",
  desktop_dir: "unavailable outside Tauri",
  state_dir: "unavailable outside Tauri",
  anchors_planned: 4,
  anchors_created: 0,
  stale_detected: 0,
  stale_deleted: 0,
  anchor_files: [],
  last_error: "Tauri runtime unavailable; Anchor Shortcuts are materialized only by the Windows app."
};

const widgets: RuntimeWidget[] = [
  {
    id: "clock",
    name: "Clock",
    size: "2×2",
    grid: "2 / 2 / span 2 / span 2",
    accent: "mint",
    anchorCount: 4,
    manifest: "widgets/clock/widget.json",
    body: "Local-first time widget pinned to a 2×2 desktop icon footprint.",
    metric: currentTime(),
    status: "overlay preview",
    permissions: [{ kind: "time", label: "local time" }],
    actions: ["open calendar", "copy current time"]
  },
  {
    id: "weather",
    name: "Weather",
    size: "2×2",
    grid: "2 / 5 / span 2 / span 2",
    accent: "sky",
    anchorCount: 4,
    manifest: "widgets/weather/widget.json",
    body: "Network-backed widget rendered above reserved Anchor Shortcut slots.",
    metric: "Seoul · 23°",
    status: "data source mock",
    permissions: [{ kind: "network", label: "weather API" }],
    actions: ["refresh forecast", "open weather source"]
  },
  {
    id: "launcher",
    name: "Project Launcher",
    size: "4×2",
    grid: "4 / 2 / span 2 / span 4",
    accent: "violet",
    anchorCount: 8,
    manifest: "widgets/project-launcher/widget.json",
    body: "Opens repos, folders, dashboards, and local dev URLs.",
    metric: "4 actions",
    status: "action bridge mock",
    permissions: [
      { kind: "filesystem", label: "open folders" },
      { kind: "process", label: "launch apps" }
    ],
    actions: ["open repo", "open terminal", "open localhost", "open docs"]
  },
  {
    id: "system",
    name: "System Monitor",
    size: "2×2",
    grid: "4 / 7 / span 2 / span 2",
    accent: "amber",
    anchorCount: 4,
    manifest: "widgets/system-monitor/widget.json",
    body: "Metrics stay behind the Rust permission boundary.",
    metric: "CPU 18%",
    status: "permission gate mock",
    permissions: [{ kind: "system", label: "read metrics" }],
    actions: ["open task manager", "copy diagnostics"]
  }
];

const desktopIcons: DesktopIcon[] = [
  { name: "Recycle", glyph: "♻", gridColumn: "1", gridRow: "1" },
  { name: "Files", glyph: "▣", gridColumn: "1", gridRow: "2" },
  { name: "Browser", glyph: "◎", gridColumn: "1", gridRow: "3" },
  { name: "Terminal", glyph: "›_", gridColumn: "1", gridRow: "4" },
  { name: "Notes", glyph: "✎", gridColumn: "8", gridRow: "1" }
];

let activeWidgetId = "launcher";
let latestStatus: AppStatus = fallbackStatus;
let latestAnchorLifecycle: AnchorLifecycleStatus = fallbackAnchorLifecycle;

const root = document.querySelector<HTMLDivElement>("#app");

if (!root) {
  throw new Error("#app root not found");
}

const appRoot: HTMLDivElement = root;

function render(status: AppStatus, anchorLifecycle: AnchorLifecycleStatus = latestAnchorLifecycle) {
  latestStatus = status;
  latestAnchorLifecycle = anchorLifecycle;
  const activeWidget = widgets.find((widget) => widget.id === activeWidgetId) ?? widgets[0];

  appRoot.innerHTML = `
    <section class="runtime-shell" aria-label="OpenWidGet product proof surface">
      <header class="runtime-topbar">
        <div class="brand-lockup">
          <span class="brand-mark" aria-hidden="true">OW</span>
          <span>
            <span class="eyebrow">v${status.version} · ${status.mode}</span>
            <strong>${status.app_name}</strong>
          </span>
        </div>
        <div class="thesis-line">
          <span>Windows desktop widget runtime</span>
          <code>Live Shortcut Group + Overlay + Actions</code>
        </div>
      </header>

      <main class="proof-layout">
        <section class="desktop-surface" aria-label="Windows desktop widget surface preview">
          <div class="surface-copy">
            <p class="eyebrow">Product proof surface</p>
            <h1>HTML/CSS/JS widgets, pinned to the Windows desktop grid.</h1>
            <p>
              OpenWidGet reserves desktop icon-grid space with runtime Anchor Shortcuts, then renders live WebView widgets above those slots.
            </p>
          </div>

          <div class="desktop-frame">
            <div class="desktop-frame__chrome">
              <span>Windows desktop · primary monitor</span>
              <span>8×6 icon grid · ${widgets.reduce((total, widget) => total + widget.anchorCount, 0)} anchor slots</span>
            </div>
            <div class="desktop-grid" aria-label="Icon grid with overlay widgets">
              ${desktopIcons.map(renderDesktopIcon).join("")}
              ${widgets.map((widget) => renderWidget(widget, activeWidget.id)).join("")}
              <div class="taskbar" aria-hidden="true">
                <span class="start-dot">⊞</span>
                <span class="taskbar-search">Search</span>
                <span class="taskbar-app is-active">OpenWidGet</span>
                <span class="taskbar-tray">Tray · runtime on</span>
              </div>
            </div>
          </div>
        </section>

        <aside class="runtime-receipt" aria-label="Selected widget manifest and runtime receipt">
          ${renderReceipt(activeWidget, anchorLifecycle)}
        </aside>
      </main>
    </section>
  `;

  document.querySelector<HTMLButtonElement>("#refresh-status")?.addEventListener("click", loadStatus);
  document.querySelectorAll<HTMLButtonElement>("[data-widget-select]").forEach((button) => {
    button.addEventListener("click", () => {
      activeWidgetId = button.dataset.widgetSelect ?? activeWidgetId;
      render(latestStatus, latestAnchorLifecycle);
    });
  });
}

function renderDesktopIcon(icon: DesktopIcon) {
  return `
    <span class="desktop-icon" style="grid-column: ${icon.gridColumn}; grid-row: ${icon.gridRow};">
      <span>${icon.glyph}</span>
      <small>${icon.name}</small>
    </span>
  `;
}

function renderWidget(widget: RuntimeWidget, activeId: string) {
  const isActive = widget.id === activeId;
  const permissions = widget.permissions.map((permission) => permission.kind).join(" · ");

  return `
    <button
      class="runtime-widget runtime-widget--${widget.accent}${isActive ? " is-active" : ""}"
      style="grid-area: ${widget.grid};"
      data-widget-select="${widget.id}"
      type="button"
      aria-pressed="${isActive}"
    >
      <span class="anchor-matrix" aria-hidden="true">${renderAnchorDots(widget.anchorCount)}</span>
      <span class="widget-topline">
        <span>${widget.size} Live Shortcut Group</span>
        <span>${widget.status}</span>
      </span>
      <strong>${widget.name}</strong>
      <span class="widget-metric" data-live-clock="${widget.id === "clock" ? "true" : "false"}">${widget.metric}</span>
      <span class="widget-body">${widget.body}</span>
      <span class="widget-permissions">${permissions}</span>
    </button>
  `;
}

function renderAnchorDots(count: number) {
  return Array.from({ length: count }, (_, index) => `<i style="--i:${index}"></i>`).join("");
}

function renderReceipt(widget: RuntimeWidget, anchorLifecycle: AnchorLifecycleStatus) {
  const permissionRows = widget.permissions
    .map(
      (permission) => `
        <li>
          <span class="permission-kind">${permission.kind}</span>
          <span>${permission.label}</span>
        </li>
      `
    )
    .join("");

  const actionRows = widget.actions.map((action) => `<li>${action}</li>`).join("");
  const lifecycleState = anchorLifecycle.enabled ? "active" : "preview";
  const lifecycleError = anchorLifecycle.last_error
    ? `<p class="anchor-lifecycle__warning">${anchorLifecycle.last_error}</p>`
    : "";

  return `
    <div class="receipt-section receipt-section--hero">
      <p class="eyebrow">Selected widget receipt</p>
      <h2>${widget.name}</h2>
      <p>${widget.manifest}</p>
    </div>

    <div class="manifest-card">
      <div class="manifest-card__title">widget.json</div>
      <pre>{
  "id": "${widget.id}",
  "runtime": "desktop-grid",
  "size": "${widget.size}",
  "anchors": ${widget.anchorCount},
  "entry": "index.html"
}</pre>
    </div>

    <div class="anchor-lifecycle-card">
      <div>
        <span class="permission-kind">Anchor Shortcut lifecycle</span>
        <strong>${anchorLifecycle.anchors_created}/${anchorLifecycle.anchors_planned} created · ${lifecycleState}</strong>
      </div>
      <dl>
        <div><dt>session</dt><dd>${anchorLifecycle.session_id}</dd></div>
        <div><dt>platform</dt><dd>${anchorLifecycle.platform}</dd></div>
        <div><dt>stale cleanup</dt><dd>${anchorLifecycle.stale_deleted}/${anchorLifecycle.stale_detected}</dd></div>
        <div><dt>desktop</dt><dd>${anchorLifecycle.desktop_dir}</dd></div>
      </dl>
      ${lifecycleError}
    </div>

    <div class="receipt-section">
      <h3>Permissions</h3>
      <ul class="permission-list">${permissionRows}</ul>
    </div>

    <div class="receipt-section">
      <h3>Actions</h3>
      <ul class="action-list">${actionRows}</ul>
    </div>

    <div class="runtime-lifecycle">
      <span>startup</span>
      <i></i>
      <span>materialize anchors</span>
      <i></i>
      <span>render overlay</span>
      <i></i>
      <span>cleanup on quit</span>
    </div>

    <div class="receipt-actions">
      <button id="refresh-status" type="button">Refresh status</button>
      <span class="runtime-pill">Tray agent: ready</span>
    </div>
  `;
}

function currentTime() {
  return new Intl.DateTimeFormat(undefined, {
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit"
  }).format(new Date());
}

function refreshClockText() {
  widgets[0].metric = currentTime();
  document.querySelectorAll<HTMLElement>('[data-live-clock="true"]').forEach((clock) => {
    clock.textContent = widgets[0].metric;
  });
}

async function loadStatus() {
  try {
    const [status, anchorLifecycle] = await Promise.all([
      invoke<AppStatus>("get_app_status"),
      invoke<AnchorLifecycleStatus>("get_anchor_lifecycle_status").catch((error) => {
        console.warn("Falling back to preview Anchor Shortcut lifecycle:", error);
        return fallbackAnchorLifecycle;
      })
    ]);
    render(status, anchorLifecycle);
  } catch (error) {
    console.warn("Falling back to browser preview status:", error);
    render(fallbackStatus, fallbackAnchorLifecycle);
  }
}

void loadStatus();
setInterval(refreshClockText, 1000);
