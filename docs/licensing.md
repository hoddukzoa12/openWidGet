# openWidGet Licensing Policy

openWidGet uses a hybrid licensing strategy so the platform stays open while widget authors can contribute with low friction.

## Summary

- **Core/runtime/desktop app:** AGPL-3.0-or-later
- **Widget templates and example widget starter code:** MIT
- **Community widgets:** Any OSI-approved open-source license, declared with an SPDX identifier in `widget.json`

## Why AGPL for the core?

openWidGet is a desktop platform and runtime, not just a small utility library. The core value lives in the engine that manages:

- Live Shortcut Groups / Anchor Shortcuts
- the desktop overlay runtime
- widget installation and registry logic
- permission checks
- data scheduling
- OAuth/token isolation
- app detection and recommendation logic

AGPL-3.0-or-later keeps improvements to that platform open and returned to the community, including cases where someone modifies the platform for a network-connected service or hosted registry flow.

## Why MIT for widget templates?

Widget authors should be able to start quickly. The starter templates and example snippets are intentionally permissive so contributors can copy them into their own widgets without license confusion.

The template license text is stored at:

```txt
licenses/MIT-WIDGET-TEMPLATES.txt
```

## Community widget rule

Each widget package must declare its own license in `widget.json`:

```json
{
  "id": "weather",
  "name": "Weather",
  "license": "MIT"
}
```

Accepted widget licenses must be OSI-approved, for example:

- MIT
- Apache-2.0
- BSD-2-Clause / BSD-3-Clause
- ISC
- MPL-2.0
- GPL-3.0-or-later
- AGPL-3.0-or-later

Non-commercial, academic-only, source-available, or custom restrictive licenses are not accepted for the official community registry unless the project explicitly documents compatible permission evidence.

## Contributor expectation

By contributing to the openWidGet core repository, contributors agree that their core/runtime contributions are licensed under AGPL-3.0-or-later unless the file path explicitly states another license.

By contributing a widget package, contributors must provide a valid OSI-approved license in the widget manifest and include any required third-party notices.

## Contest note

This policy is designed to satisfy OSS contest requirements that directly written source code use an OSI-approved open-source license and that all third-party libraries, frameworks, models, and assets disclose their sources and licenses.
