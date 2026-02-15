import json
import random
import string
from datetime import datetime, timedelta
from pathlib import Path

OUT_DIR = Path("verillm_mock_data")
OUT_DIR.mkdir(exist_ok=True)

# -------------------
# Theme (Light Mode)
# -------------------
THEME = {
    "background": "#ffffff",
    "text": "#000000",
    "primary_blue": "#64b5f6",
    "secondary_orange": "#ffb74d",
    "border": "#e0e0e0",
    "card_bg": "#fafafa",
    "table_header_bg": "#f5f5f5"
}

# -------------------
# Helpers
# -------------------
def random_hash(n=20):
    return ''.join(random.choices("abcdef0123456789", k=n))

# -------------------
# Receipt Explorer
# -------------------
receipt_explorer = {
    "filters": {
        "model_id": "gpt-3.5-turbo",
        "start_time": "2026-02-01",
        "end_time": "2026-02-14"
    },
    "rows": [
        {
            "leaf_hash": random_hash(),
            "index": i + 1,
            "root_hash": random_hash(),
            "timestamp": (
                datetime(2026, 2, 14, 10, 30) + timedelta(minutes=i)
            ).strftime("%Y-%m-%d %H:%M:%S"),
            "action": "View"
        }
        for i in range(12)
    ]
}

# -------------------
# Model Lineage
# -------------------
model_lineage = {
    "title": "Model Usage",
    "date_range": {
        "start_time": "2026-02-01",
        "end_time": "2026-02-14"
    },
    "x_axis": ["gpt-3.5-turbo", "llama-2-7b", "claude-v2"],
    "y_axis": {
        "min": 0,
        "max": 100,
        "ticks": [0, 25, 50, 75, 100]
    },
    "values": {
        "gpt-3.5-turbo": 75,
        "llama-2-7b": 45,
        "claude-v2": 90
    },
    "colors": {
        "gpt-3.5-turbo": THEME["primary_blue"],
        "llama-2-7b": THEME["secondary_orange"],
        "claude-v2": THEME["primary_blue"]
    }
}

# -------------------
# Privacy Budget
# -------------------
privacy_budget = [
    {
        "model": "gpt-3.5-turbo",
        "epsilon": {"remaining": 8.2, "total": 10, "progress": 0.82},
        "delta": {"remaining": 9.5e-6, "total": 1e-5, "progress": 0.95}
    },
    {
        "model": "llama-2-7b",
        "epsilon": {"remaining": 6.5, "total": 10, "progress": 0.65},
        "delta": {"remaining": 8.0e-6, "total": 1e-5, "progress": 0.80}
    },
    {
        "model": "claude-v2",
        "epsilon": {"remaining": 9.1, "total": 10, "progress": 0.91},
        "delta": {"remaining": 9.8e-6, "total": 1e-5, "progress": 0.98}
    }
]

# -------------------
# Audit Export Wizard
# -------------------
audit_export = {
    "stepper": ["Select Models", "Select Date Range", "Confirm and Export"],
    "active_step": 1,
    "models": [
        {"name": "gpt-3.5-turbo", "checked": True},
        {"name": "llama-2-7b", "checked": False},
        {"name": "claude-v2", "checked": True},
        {"name": "mistral-7b", "checked": False}
    ],
    "buttons": {
        "back": {"enabled": False},
        "next": {"enabled": True}
    },
    "hint": "Choose one or more models to include in the export."
}

# -------------------
# Write All Files
# -------------------
(Path(OUT_DIR) / "theme.json").write_text(json.dumps(THEME, indent=2))
(Path(OUT_DIR) / "receipt_explorer.json").write_text(json.dumps(receipt_explorer, indent=2))
(Path(OUT_DIR) / "model_lineage.json").write_text(json.dumps(model_lineage, indent=2))
(Path(OUT_DIR) / "privacy_budget.json").write_text(json.dumps(privacy_budget, indent=2))
(Path(OUT_DIR) / "audit_export.json").write_text(json.dumps(audit_export, indent=2))

print("✅ Mock screenshot data generated in ./verillm_mock_data/")
