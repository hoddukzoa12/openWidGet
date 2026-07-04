# Widget Format Draft

OpenWidGet widgets are intended to be small HTML/CSS/JS packages.

```txt
widgets/<widget-id>/
├─ widget.json
├─ index.html
├─ style.css
├─ widget.js
└─ README.md
```

## Draft `widget.json`

```json
{
  "id": "clock",
  "name": "Clock",
  "version": "0.1.0",
  "license": "MIT",
  "entry": "index.html",
  "size": {
    "default": [2, 2],
    "min": [2, 2],
    "max": [4, 4]
  },
  "permissions": {
    "network": [],
    "systemInfo": false,
    "filesystem": false,
    "commands": false
  }
}
```

This is not final. The validator and admission rules will be implemented in a later issue.
