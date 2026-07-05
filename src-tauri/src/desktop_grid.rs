use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GridRect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GridCell {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DpiInfo {
    pub x: f32,
    pub y: f32,
    pub scale: f32,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct AnchorSlotRect {
    pub slot: String,
    pub row: u8,
    pub column: u8,
    pub rect: GridRect,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct WidgetGridPlan {
    pub widget_id: String,
    pub size: String,
    pub start_row: u8,
    pub start_column: u8,
    pub rows: u8,
    pub columns: u8,
    pub rect: GridRect,
    pub anchor_slots: Vec<AnchorSlotRect>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DesktopPositioningStatus {
    pub mode: &'static str,
    pub commit_policy: &'static str,
    pub api_path: Vec<&'static str>,
    pub fallback: &'static str,
    pub limitations: Vec<&'static str>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ObservedAnchorPosition {
    pub name: String,
    pub index: u32,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct AnchorPositionMatch {
    pub name: String,
    pub target_widget_id: String,
    pub target_slot: String,
    pub target_rect: Option<GridRect>,
    pub target_view_x: Option<i32>,
    pub target_view_y: Option<i32>,
    pub observed_x: i32,
    pub observed_y: i32,
    pub delta_x: Option<i32>,
    pub delta_y: Option<i32>,
    pub status: &'static str,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DesktopGridReconciliation {
    pub mode: &'static str,
    pub observed_anchors: Vec<ObservedAnchorPosition>,
    pub matches: Vec<AnchorPositionMatch>,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DesktopGridStatus {
    pub platform: &'static str,
    pub enabled: bool,
    pub source: String,
    pub work_area: GridRect,
    pub monitor_bounds: GridRect,
    pub dpi: DpiInfo,
    pub icon_cell: GridCell,
    pub grid_columns: u32,
    pub grid_rows: u32,
    pub plans: Vec<WidgetGridPlan>,
    pub positioning: DesktopPositioningStatus,
    pub reconciliation: DesktopGridReconciliation,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct WindowsGridProbe {
    work_area: GridRect,
    monitor_bounds: GridRect,
    dpi: DpiInfo,
    icon_cell: GridCell,
    source: String,
}

#[derive(Debug, Clone, Deserialize)]
struct WindowsListViewAnchorPosition {
    name: String,
    index: u32,
    x: i32,
    y: i32,
}

pub fn get_desktop_grid_status() -> DesktopGridStatus {
    let (measurement, enabled, source, last_error) = match probe_windows_grid() {
        Ok(probe) => (
            DesktopGridMeasurement {
                work_area: probe.work_area,
                monitor_bounds: probe.monitor_bounds,
                dpi: probe.dpi,
                icon_cell: sanitize_icon_cell(probe.icon_cell),
            },
            true,
            probe.source,
            None,
        ),
        Err(error) => {
            let measurement = fallback_grid_measurement();
            (
                measurement,
                false,
                "deterministic-preview-fallback".to_string(),
                Some(error),
            )
        }
    };

    build_grid_status(measurement, enabled, source, last_error)
}

pub fn fallback_desktop_grid_status(error: String) -> DesktopGridStatus {
    build_grid_status(
        fallback_grid_measurement(),
        false,
        "deterministic-preview-fallback".to_string(),
        Some(error),
    )
}

#[derive(Debug, Clone, PartialEq)]
struct DesktopGridMeasurement {
    work_area: GridRect,
    monitor_bounds: GridRect,
    dpi: DpiInfo,
    icon_cell: GridCell,
}

fn build_grid_status(
    measurement: DesktopGridMeasurement,
    enabled: bool,
    source: String,
    last_error: Option<String>,
) -> DesktopGridStatus {
    let grid_columns = measurement.work_area.width / measurement.icon_cell.width.max(1);
    let grid_rows = measurement.work_area.height / measurement.icon_cell.height.max(1);
    let plans = vec![
        build_widget_grid_plan("clock", 2, 2, 1, 1, &measurement),
        build_widget_grid_plan("launcher", 4, 2, 3, 1, &measurement),
    ];
    let reconciliation = if enabled {
        reconcile_anchor_positions(&plans, &measurement)
    } else {
        fallback_reconciliation(last_error.clone())
    };

    DesktopGridStatus {
        platform: std::env::consts::OS,
        enabled,
        source,
        work_area: measurement.work_area,
        monitor_bounds: measurement.monitor_bounds,
        dpi: measurement.dpi,
        icon_cell: measurement.icon_cell,
        grid_columns,
        grid_rows,
        plans,
        positioning: positioning_status(),
        reconciliation,
        last_error,
    }
}

fn build_widget_grid_plan(
    widget_id: &str,
    columns: u8,
    rows: u8,
    start_column: u8,
    start_row: u8,
    measurement: &DesktopGridMeasurement,
) -> WidgetGridPlan {
    let cell = &measurement.icon_cell;
    let origin_x =
        measurement.work_area.x + i32::from(start_column.saturating_sub(1)) * cell.width as i32;
    let origin_y =
        measurement.work_area.y + i32::from(start_row.saturating_sub(1)) * cell.height as i32;
    let rect = GridRect {
        x: origin_x,
        y: origin_y,
        width: u32::from(columns) * cell.width,
        height: u32::from(rows) * cell.height,
    };

    let mut anchor_slots = Vec::with_capacity(usize::from(columns) * usize::from(rows));
    for row in 1..=rows {
        for column in 1..=columns {
            let x = origin_x + i32::from(column - 1) * cell.width as i32;
            let y = origin_y + i32::from(row - 1) * cell.height as i32;
            anchor_slots.push(AnchorSlotRect {
                slot: format!("r{row}c{column}"),
                row,
                column,
                rect: GridRect {
                    x,
                    y,
                    width: cell.width,
                    height: cell.height,
                },
            });
        }
    }

    WidgetGridPlan {
        widget_id: widget_id.to_string(),
        size: format!("{columns}×{rows}"),
        start_row,
        start_column,
        rows,
        columns,
        rect,
        anchor_slots,
    }
}

fn positioning_status() -> DesktopPositioningStatus {
    DesktopPositioningStatus {
        mode: "reconcile-only",
        commit_policy: "move overlay during drag; batch-commit Anchor Shortcut positions only on drop",
        api_path: vec![
            "Find Progman/WorkerW desktop ListView (SHELLDLL_DefView/SysListView32)",
            "read current icon cells with LVM_GETITEMPOSITION for reconciliation",
            "batch-commit final drop positions with LVM_SETITEMPOSITION only after Windows proof",
        ],
        fallback: "if direct ListView positioning is unreliable, keep Anchor Shortcuts materialized and align the WebView overlay from the computed grid receipt",
        limitations: vec![
            "primary monitor only in this spike",
            "auto-arrange icons can override direct shortcut positioning",
            "icon size/spacing/DPI changes require a fresh grid probe",
            "direct mutation of Explorer icon positions is intentionally not executed on every drag frame",
        ],
    }
}

fn fallback_reconciliation(last_error: Option<String>) -> DesktopGridReconciliation {
    DesktopGridReconciliation {
        mode: "unavailable",
        observed_anchors: Vec::new(),
        matches: Vec::new(),
        last_error: Some(last_error.unwrap_or_else(|| {
            "desktop grid probing is disabled; reconciliation skipped".to_string()
        })),
    }
}

fn reconcile_anchor_positions(
    plans: &[WidgetGridPlan],
    measurement: &DesktopGridMeasurement,
) -> DesktopGridReconciliation {
    match probe_windows_anchor_positions() {
        Ok(observed_anchors) => {
            let matches = observed_anchors
                .iter()
                .map(|anchor| match_anchor_to_target(anchor, plans, measurement))
                .collect();
            DesktopGridReconciliation {
                mode: "windows-listview-probe",
                observed_anchors,
                matches,
                last_error: None,
            }
        }
        Err(error) => DesktopGridReconciliation {
            mode: "unavailable",
            observed_anchors: Vec::new(),
            matches: Vec::new(),
            last_error: Some(error),
        },
    }
}

fn match_anchor_to_target(
    anchor: &ObservedAnchorPosition,
    plans: &[WidgetGridPlan],
    measurement: &DesktopGridMeasurement,
) -> AnchorPositionMatch {
    let (target_widget_id, target_slot) = parse_anchor_widget_and_slot(&anchor.name)
        .unwrap_or_else(|| ("unknown".to_string(), "unknown".to_string()));
    let target_rect = plans
        .iter()
        .find(|plan| plan.widget_id == target_widget_id)
        .into_iter()
        .flat_map(|plan| plan.anchor_slots.iter())
        .find(|slot| slot.slot == target_slot)
        .map(|slot| slot.rect.clone());

    let target_view_x = target_rect
        .as_ref()
        .map(|rect| rect.x - measurement.monitor_bounds.x);
    let target_view_y = target_rect
        .as_ref()
        .map(|rect| rect.y - measurement.monitor_bounds.y);

    let (delta_x, delta_y, status) = match (target_view_x, target_view_y) {
        (Some(x), Some(y)) => (
            Some(anchor.x - x),
            Some(anchor.y - y),
            "observed-target-view-delta",
        ),
        _ => (None, None, "no-target-slot"),
    };

    AnchorPositionMatch {
        name: anchor.name.clone(),
        target_widget_id,
        target_slot,
        target_rect,
        target_view_x,
        target_view_y,
        observed_x: anchor.x,
        observed_y: anchor.y,
        delta_x,
        delta_y,
        status,
    }
}

fn parse_anchor_widget_and_slot(name: &str) -> Option<(String, String)> {
    let stem = name.strip_suffix(".lnk").unwrap_or(name);
    let parts = stem.split(" - ").collect::<Vec<_>>();
    if parts.len() < 4 || parts.first()? != &"OpenWidGet Anchor" {
        return None;
    }

    let widget_id = parts.get(1)?.trim();
    let slot = parts.last()?.trim();
    if widget_id.is_empty() || !(slot.starts_with('r') && slot.contains('c')) {
        return None;
    }

    Some((widget_id.to_string(), slot.to_string()))
}

#[cfg(windows)]
const WINDOWS_PROBE_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(4);

#[cfg(windows)]
fn run_powershell_probe(script: &str, label: &str) -> Result<std::process::Output, String> {
    use std::process::{Command, Stdio};
    use std::thread;
    use std::time::Instant;

    let mut child = Command::new("powershell.exe")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            script,
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("failed to start {label}: {error}"))?;

    let started = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(_status)) => {
                return child
                    .wait_with_output()
                    .map_err(|error| format!("failed to collect {label} output: {error}"));
            }
            Ok(None) if started.elapsed() >= WINDOWS_PROBE_TIMEOUT => {
                let _ = child.kill();
                let _ = child.wait_with_output();
                return Err(format!(
                    "{label} timed out after {}s",
                    WINDOWS_PROBE_TIMEOUT.as_secs()
                ));
            }
            Ok(None) => thread::sleep(std::time::Duration::from_millis(50)),
            Err(error) => return Err(format!("failed while waiting for {label}: {error}")),
        }
    }
}

#[cfg(windows)]
fn probe_windows_anchor_positions() -> Result<Vec<ObservedAnchorPosition>, String> {
    let script = r#"
$ErrorActionPreference = 'Stop'
$OutputEncoding = [Console]::OutputEncoding = [System.Text.Encoding]::UTF8
Add-Type -TypeDefinition @'
using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;

public class DesktopIconProbeItem {
    public int index;
    public string name;
    public int x;
    public int y;
}

public static class DesktopListViewProbe {
    private const UInt32 LVM_FIRST = 0x1000;
    private const UInt32 LVM_GETITEMCOUNT = LVM_FIRST + 4;
    private const UInt32 LVM_GETITEMPOSITION = LVM_FIRST + 16;
    private const UInt32 LVM_GETITEMTEXTW = LVM_FIRST + 115;
    private const UInt32 LVIF_TEXT = 0x0001;
    private const UInt32 MEM_COMMIT = 0x1000;
    private const UInt32 MEM_RELEASE = 0x8000;
    private const UInt32 PAGE_READWRITE = 0x04;
    private const UInt32 PROCESS_VM_OPERATION = 0x0008;
    private const UInt32 PROCESS_VM_READ = 0x0010;
    private const UInt32 PROCESS_VM_WRITE = 0x0020;
    private const UInt32 PROCESS_QUERY_INFORMATION = 0x0400;

    [StructLayout(LayoutKind.Sequential)]
    private struct POINT { public int X; public int Y; }

    [StructLayout(LayoutKind.Sequential, CharSet = CharSet.Unicode)]
    private struct LVITEMW {
        public UInt32 mask;
        public Int32 iItem;
        public Int32 iSubItem;
        public UInt32 state;
        public UInt32 stateMask;
        public IntPtr pszText;
        public Int32 cchTextMax;
        public Int32 iImage;
        public IntPtr lParam;
        public Int32 iIndent;
        public Int32 iGroupId;
        public UInt32 cColumns;
        public IntPtr puColumns;
        public IntPtr piColFmt;
        public Int32 iGroup;
    }

    private delegate bool EnumWindowsProc(IntPtr hWnd, IntPtr lParam);

    [DllImport("user32.dll", SetLastError = true, CharSet = CharSet.Unicode)]
    private static extern IntPtr FindWindow(string lpClassName, string lpWindowName);

    [DllImport("user32.dll", SetLastError = true, CharSet = CharSet.Unicode)]
    private static extern IntPtr FindWindowEx(IntPtr hwndParent, IntPtr hwndChildAfter, string lpszClass, string lpszWindow);

    [DllImport("user32.dll")]
    private static extern bool EnumWindows(EnumWindowsProc lpEnumFunc, IntPtr lParam);

    [DllImport("user32.dll")]
    private static extern IntPtr SendMessage(IntPtr hWnd, UInt32 Msg, IntPtr wParam, IntPtr lParam);

    [DllImport("user32.dll", SetLastError = true)]
    private static extern UInt32 GetWindowThreadProcessId(IntPtr hWnd, out UInt32 lpdwProcessId);

    [DllImport("kernel32.dll", SetLastError = true)]
    private static extern IntPtr OpenProcess(UInt32 dwDesiredAccess, bool bInheritHandle, UInt32 dwProcessId);

    [DllImport("kernel32.dll", SetLastError = true)]
    private static extern bool CloseHandle(IntPtr hObject);

    [DllImport("kernel32.dll", SetLastError = true)]
    private static extern IntPtr VirtualAllocEx(IntPtr hProcess, IntPtr lpAddress, UIntPtr dwSize, UInt32 flAllocationType, UInt32 flProtect);

    [DllImport("kernel32.dll", SetLastError = true)]
    private static extern bool VirtualFreeEx(IntPtr hProcess, IntPtr lpAddress, UIntPtr dwSize, UInt32 dwFreeType);

    [DllImport("kernel32.dll", SetLastError = true)]
    private static extern bool ReadProcessMemory(IntPtr hProcess, IntPtr lpBaseAddress, byte[] lpBuffer, Int32 dwSize, out IntPtr lpNumberOfBytesRead);

    [DllImport("kernel32.dll", SetLastError = true)]
    private static extern bool WriteProcessMemory(IntPtr hProcess, IntPtr lpBaseAddress, byte[] lpBuffer, Int32 nSize, out IntPtr lpNumberOfBytesWritten);

    public static IntPtr FindDesktopListView() {
        IntPtr progman = FindWindow("Progman", "Program Manager");
        IntPtr defView = FindWindowEx(progman, IntPtr.Zero, "SHELLDLL_DefView", null);
        if (defView != IntPtr.Zero) {
            IntPtr list = FindWindowEx(defView, IntPtr.Zero, "SysListView32", "FolderView");
            if (list != IntPtr.Zero) return list;
        }

        IntPtr found = IntPtr.Zero;
        EnumWindows(delegate (IntPtr hwnd, IntPtr lParam) {
            IntPtr shell = FindWindowEx(hwnd, IntPtr.Zero, "SHELLDLL_DefView", null);
            if (shell != IntPtr.Zero) {
                IntPtr list = FindWindowEx(shell, IntPtr.Zero, "SysListView32", "FolderView");
                if (list != IntPtr.Zero) {
                    found = list;
                    return false;
                }
            }
            return true;
        }, IntPtr.Zero);
        return found;
    }

    public static List<DesktopIconProbeItem> GetOpenWidGetAnchors() {
        IntPtr list = FindDesktopListView();
        if (list == IntPtr.Zero) throw new Exception("Desktop SysListView32 FolderView was not found");

        UInt32 pid;
        GetWindowThreadProcessId(list, out pid);
        IntPtr process = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_OPERATION | PROCESS_VM_READ | PROCESS_VM_WRITE, false, pid);
        if (process == IntPtr.Zero) throw new Exception("Failed to open Explorer process for ListView read");

        List<DesktopIconProbeItem> result = new List<DesktopIconProbeItem>();
        IntPtr remotePoint = IntPtr.Zero;
        IntPtr remoteText = IntPtr.Zero;
        IntPtr remoteItem = IntPtr.Zero;
        try {
            int count = SendMessage(list, LVM_GETITEMCOUNT, IntPtr.Zero, IntPtr.Zero).ToInt32();
            remotePoint = VirtualAllocEx(process, IntPtr.Zero, (UIntPtr)Marshal.SizeOf(typeof(POINT)), MEM_COMMIT, PAGE_READWRITE);
            remoteText = VirtualAllocEx(process, IntPtr.Zero, (UIntPtr)520, MEM_COMMIT, PAGE_READWRITE);
            remoteItem = VirtualAllocEx(process, IntPtr.Zero, (UIntPtr)Marshal.SizeOf(typeof(LVITEMW)), MEM_COMMIT, PAGE_READWRITE);
            for (int i = 0; i < count; i++) {
                SendMessage(list, LVM_GETITEMPOSITION, (IntPtr)i, remotePoint);
                byte[] pointBytes = new byte[Marshal.SizeOf(typeof(POINT))];
                IntPtr bytesRead;
                ReadProcessMemory(process, remotePoint, pointBytes, pointBytes.Length, out bytesRead);
                int x = BitConverter.ToInt32(pointBytes, 0);
                int y = BitConverter.ToInt32(pointBytes, 4);

                LVITEMW item = new LVITEMW();
                item.mask = LVIF_TEXT;
                item.iSubItem = 0;
                item.pszText = remoteText;
                item.cchTextMax = 260;
                byte[] itemBytes = new byte[Marshal.SizeOf(typeof(LVITEMW))];
                IntPtr localItem = Marshal.AllocHGlobal(itemBytes.Length);
                try {
                    Marshal.StructureToPtr(item, localItem, false);
                    Marshal.Copy(localItem, itemBytes, 0, itemBytes.Length);
                } finally {
                    Marshal.FreeHGlobal(localItem);
                }
                IntPtr bytesWritten;
                WriteProcessMemory(process, remoteItem, itemBytes, itemBytes.Length, out bytesWritten);
                SendMessage(list, LVM_GETITEMTEXTW, (IntPtr)i, remoteItem);
                byte[] textBytes = new byte[520];
                ReadProcessMemory(process, remoteText, textBytes, textBytes.Length, out bytesRead);
                string name = Encoding.Unicode.GetString(textBytes).TrimEnd('\0');
                if (name.StartsWith("OpenWidGet Anchor", StringComparison.OrdinalIgnoreCase)) {
                    result.Add(new DesktopIconProbeItem { index = i, name = name, x = x, y = y });
                }
            }
        } finally {
            if (remotePoint != IntPtr.Zero) VirtualFreeEx(process, remotePoint, UIntPtr.Zero, MEM_RELEASE);
            if (remoteText != IntPtr.Zero) VirtualFreeEx(process, remoteText, UIntPtr.Zero, MEM_RELEASE);
            if (remoteItem != IntPtr.Zero) VirtualFreeEx(process, remoteItem, UIntPtr.Zero, MEM_RELEASE);
            CloseHandle(process);
        }
        return result;
    }
}
'@
[DesktopListViewProbe]::GetOpenWidGetAnchors() | ConvertTo-Json -Depth 5 -Compress
"#;

    let output = run_powershell_probe(script, "Windows desktop ListView anchor probe")?;

    if !output.status.success() {
        return Err(format!(
            "Windows desktop ListView anchor probe failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if stdout.is_empty() {
        return Ok(Vec::new());
    }

    match serde_json::from_str::<Vec<WindowsListViewAnchorPosition>>(&stdout) {
        Ok(items) => Ok(items.into_iter().map(ObservedAnchorPosition::from).collect()),
        Err(vector_error) => serde_json::from_str::<WindowsListViewAnchorPosition>(&stdout)
            .map(|item| vec![ObservedAnchorPosition::from(item)])
            .map_err(|single_error| {
                format!(
                    "failed to parse Windows ListView anchor JSON as vector ({vector_error}) or item ({single_error})"
                )
            }),
    }
}

#[cfg(not(windows))]
fn probe_windows_anchor_positions() -> Result<Vec<ObservedAnchorPosition>, String> {
    Err("Windows desktop ListView reconciliation is unavailable on this platform".to_string())
}

impl From<WindowsListViewAnchorPosition> for ObservedAnchorPosition {
    fn from(value: WindowsListViewAnchorPosition) -> Self {
        Self {
            name: value.name,
            index: value.index,
            x: value.x,
            y: value.y,
        }
    }
}

#[cfg(windows)]
fn probe_windows_grid() -> Result<WindowsGridProbe, String> {
    let script = r#"
$ErrorActionPreference = 'Stop'
$OutputEncoding = [Console]::OutputEncoding = [System.Text.Encoding]::UTF8
Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing
$screen = [System.Windows.Forms.Screen]::PrimaryScreen
$graphics = [System.Drawing.Graphics]::FromHwnd([IntPtr]::Zero)
$dpiX = [Math]::Round($graphics.DpiX, 2)
$dpiY = [Math]::Round($graphics.DpiY, 2)
$metrics = Get-ItemProperty 'HKCU:\Control Panel\Desktop\WindowMetrics'
$cellWidth = [int][Math]::Round([Math]::Abs([int]$metrics.IconSpacing) / 15)
$cellHeight = [int][Math]::Round([Math]::Abs([int]$metrics.IconVerticalSpacing) / 15)
if ($cellWidth -lt 48) { $cellWidth = 75 }
if ($cellHeight -lt 48) { $cellHeight = 75 }
[pscustomobject]@{
  work_area = @{
    x = [int]$screen.WorkingArea.X
    y = [int]$screen.WorkingArea.Y
    width = [int]$screen.WorkingArea.Width
    height = [int]$screen.WorkingArea.Height
  }
  monitor_bounds = @{
    x = [int]$screen.Bounds.X
    y = [int]$screen.Bounds.Y
    width = [int]$screen.Bounds.Width
    height = [int]$screen.Bounds.Height
  }
  dpi = @{
    x = [double]$dpiX
    y = [double]$dpiY
    scale = [double][Math]::Round($dpiX / 96, 2)
  }
  icon_cell = @{
    width = $cellWidth
    height = $cellHeight
  }
  source = 'System.Windows.Forms.Screen.PrimaryScreen + HKCU WindowMetrics IconSpacing/IconVerticalSpacing'
} | ConvertTo-Json -Depth 5 -Compress
"#;

    let output = run_powershell_probe(script, "Windows desktop grid probe")?;

    if !output.status.success() {
        return Err(format!(
            "Windows desktop grid probe failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    serde_json::from_slice::<WindowsGridProbe>(&output.stdout)
        .map_err(|error| format!("failed to parse Windows desktop grid probe JSON: {error}"))
}

#[cfg(not(windows))]
fn probe_windows_grid() -> Result<WindowsGridProbe, String> {
    Err("Windows desktop grid probing is unavailable on this platform; using deterministic preview geometry".to_string())
}

fn fallback_grid_measurement() -> DesktopGridMeasurement {
    DesktopGridMeasurement {
        work_area: GridRect {
            x: 0,
            y: 0,
            width: 1920,
            height: 1040,
        },
        monitor_bounds: GridRect {
            x: 0,
            y: 0,
            width: 1920,
            height: 1080,
        },
        dpi: DpiInfo {
            x: 96.0,
            y: 96.0,
            scale: 1.0,
        },
        icon_cell: GridCell {
            width: 75,
            height: 75,
        },
    }
}

fn sanitize_icon_cell(cell: GridCell) -> GridCell {
    GridCell {
        width: cell.width.clamp(48, 256),
        height: cell.height.clamp(48, 256),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_measurement() -> DesktopGridMeasurement {
        DesktopGridMeasurement {
            work_area: GridRect {
                x: 10,
                y: 20,
                width: 800,
                height: 600,
            },
            monitor_bounds: GridRect {
                x: 0,
                y: 0,
                width: 800,
                height: 640,
            },
            dpi: DpiInfo {
                x: 96.0,
                y: 96.0,
                scale: 1.0,
            },
            icon_cell: GridCell {
                width: 80,
                height: 100,
            },
        }
    }

    #[test]
    fn computes_two_by_two_grid_rectangle_and_slots() {
        let plan = build_widget_grid_plan("clock", 2, 2, 2, 3, &test_measurement());

        assert_eq!(
            plan.rect,
            GridRect {
                x: 90,
                y: 220,
                width: 160,
                height: 200
            }
        );
        assert_eq!(plan.anchor_slots.len(), 4);
        assert_eq!(plan.anchor_slots[0].slot, "r1c1");
        assert_eq!(
            plan.anchor_slots[0].rect,
            GridRect {
                x: 90,
                y: 220,
                width: 80,
                height: 100
            }
        );
        assert_eq!(plan.anchor_slots[3].slot, "r2c2");
        assert_eq!(
            plan.anchor_slots[3].rect,
            GridRect {
                x: 170,
                y: 320,
                width: 80,
                height: 100
            }
        );
    }

    #[test]
    fn computes_four_by_two_grid_rectangle_and_anchor_count() {
        let plan = build_widget_grid_plan("launcher", 4, 2, 1, 1, &test_measurement());

        assert_eq!(
            plan.rect,
            GridRect {
                x: 10,
                y: 20,
                width: 320,
                height: 200
            }
        );
        assert_eq!(plan.anchor_slots.len(), 8);
        assert_eq!(plan.anchor_slots.last().unwrap().slot, "r2c4");
    }

    #[test]
    fn status_reports_reconcile_only_positioning_policy() {
        let status = build_grid_status(test_measurement(), true, "test-probe".to_string(), None);

        assert_eq!(status.grid_columns, 10);
        assert_eq!(status.grid_rows, 6);
        assert_eq!(status.plans.len(), 2);
        assert_eq!(status.positioning.mode, "reconcile-only");
        assert!(status.positioning.commit_policy.contains("batch-commit"));
        assert!(status
            .positioning
            .api_path
            .iter()
            .any(|step| step.contains("LVM_SETITEMPOSITION")));
    }

    #[test]
    fn fallback_status_skips_reconciliation_probe() {
        let status = fallback_desktop_grid_status("grid probe timed out".to_string());

        assert!(!status.enabled);
        assert_eq!(status.reconciliation.mode, "unavailable");
        assert!(status.reconciliation.observed_anchors.is_empty());
        assert!(status.reconciliation.matches.is_empty());
        assert_eq!(
            status.reconciliation.last_error.as_deref(),
            Some("grid probe timed out")
        );
    }

    #[test]
    fn parses_anchor_widget_and_slot_from_shortcut_name() {
        assert_eq!(
            parse_anchor_widget_and_slot("OpenWidGet Anchor - clock - session-1 - r2c1.lnk"),
            Some(("clock".to_string(), "r2c1".to_string()))
        );
        assert_eq!(
            parse_anchor_widget_and_slot("OpenWidGet Anchor - launcher - session-1 - r1c4.lnk"),
            Some(("launcher".to_string(), "r1c4".to_string()))
        );
        assert_eq!(parse_anchor_widget_and_slot("Normal Shortcut.lnk"), None);
    }

    #[test]
    fn matches_observed_anchor_position_to_widget_specific_target_slot() {
        let measurement = test_measurement();
        let clock_plan = build_widget_grid_plan("clock", 2, 2, 2, 3, &measurement);
        let launcher_plan = build_widget_grid_plan("launcher", 4, 2, 1, 1, &measurement);
        let observed = ObservedAnchorPosition {
            name: "OpenWidGet Anchor - launcher - session-1 - r2c2.lnk".to_string(),
            index: 7,
            x: 100,
            y: 130,
        };

        let matched = match_anchor_to_target(&observed, &[clock_plan, launcher_plan], &measurement);

        assert_eq!(matched.target_widget_id, "launcher");
        assert_eq!(matched.target_slot, "r2c2");
        assert_eq!(
            matched.target_rect.unwrap(),
            GridRect {
                x: 90,
                y: 120,
                width: 80,
                height: 100
            }
        );
        assert_eq!(matched.delta_x, Some(10));
        assert_eq!(matched.delta_y, Some(10));
        assert_eq!(matched.status, "observed-target-view-delta");
    }

    #[test]
    fn normalizes_screen_targets_to_listview_coordinates_before_delta() {
        let measurement = DesktopGridMeasurement {
            work_area: GridRect {
                x: 110,
                y: 70,
                width: 800,
                height: 600,
            },
            monitor_bounds: GridRect {
                x: 100,
                y: 50,
                width: 800,
                height: 640,
            },
            dpi: DpiInfo {
                x: 96.0,
                y: 96.0,
                scale: 1.0,
            },
            icon_cell: GridCell {
                width: 80,
                height: 100,
            },
        };
        let plan = build_widget_grid_plan("clock", 2, 2, 1, 1, &measurement);
        let observed = ObservedAnchorPosition {
            name: "OpenWidGet Anchor - clock - session-1 - r1c1.lnk".to_string(),
            index: 2,
            x: 10,
            y: 20,
        };

        let matched = match_anchor_to_target(&observed, &[plan], &measurement);

        assert_eq!(matched.target_view_x, Some(10));
        assert_eq!(matched.target_view_y, Some(20));
        assert_eq!(matched.delta_x, Some(0));
        assert_eq!(matched.delta_y, Some(0));
    }

    #[test]
    fn clamps_unreasonable_icon_cell_probe_values() {
        assert_eq!(
            sanitize_icon_cell(GridCell {
                width: 10,
                height: 999
            }),
            GridCell {
                width: 48,
                height: 256
            }
        );
    }
}
