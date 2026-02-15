# VeriLLM Audit Dashboard

A comprehensive React + TypeScript frontend for VeriLLM - a cryptographic audit dashboard for LLM inference receipts.

## Features

- **Receipt Explorer**: Browse and search cryptographic receipts with advanced filtering
- **Model Lineage Viewer**: Visualize model usage statistics with interactive charts
- **Privacy Budget Dashboard**: Track differential privacy budgets for each model
- **Audit Export Wizard**: Multi-step wizard to export audit data

## Tech Stack

- **Framework**: React 18 + TypeScript
- **Build Tool**: Vite
- **GraphQL Client**: Apollo Client
- **UI Components**: Material-UI (MUI) v5
- **Charts**: Recharts
- **Routing**: React Router v6
- **Date Handling**: date-fns

## Project Structure

```
src/
├── components/
│   ├── ReceiptExplorer.tsx         # Receipt browsing and search
│   ├── ModelLineageViewer.tsx      # Model usage visualization
│   ├── PrivacyBudgetDashboard.tsx  # Privacy budget tracking
│   ├── AuditExportWizard.tsx       # Audit export wizard
│   ├── Layout.tsx                  # Main layout with navigation
│   └── ThemeToggle.tsx             # Light/dark mode toggle
├── hooks/
│   ├── useLocalStorage.ts          # localStorage hook
│   └── useThemeMode.ts             # Theme preference hook
├── types/
│   └── graphql.ts                  # GraphQL type definitions
├── App.tsx                         # Main app with providers
├── main.tsx                        # Entry point
└── theme.ts                        # MUI theme configuration
```

## Getting Started

### Prerequisites

- Node.js 16+
- npm or yarn

### Installation

1. Install dependencies:
```bash
npm install
```

2. Create a `.env` file (optional):
```bash
cp .env.example .env
```

3. Configure the GraphQL endpoint (default: `http://localhost:8080/graphql`):
```env
VITE_GRAPHQL_URL=http://localhost:8080/graphql
```

### Development

Start the development server:
```bash
npm run dev
```

The app will be available at `http://localhost:5173`

### Build

Build for production:
```bash
npm run build
```

### Preview

Preview the production build:
```bash
npm run preview
```

## GraphQL Schema

The dashboard expects the following GraphQL queries to be available:

### GetReceipts
```graphql
query GetReceipts($limit: Int, $offset: Int, $modelId: String, $startTime: DateTime, $endTime: DateTime) {
  receipts(limit: $limit, offset: $offset, modelId: $modelId, startTime: $startTime, endTime: $endTime) {
    leafHash
    leafIndex
    rootHash
    context
    receiptJwt
    createdAt
  }
}
```

### ModelUsage
```graphql
query ModelUsage($startTime: DateTime, $endTime: DateTime) {
  modelUsage(startTime: $startTime, endTime: $endTime) {
    modelId
    count
  }
}
```

### PrivacyBudget
```graphql
query PrivacyBudget {
  privacyBudgets {
    modelId
    epsilon
    delta
    remainingEpsilon
    remainingDelta
  }
}
```

## REST API

### Export Endpoint
```
POST /export
Content-Type: application/json

{
  "models": ["gpt-4", "claude-3-opus"],
  "startDate": "2024-01-01T00:00:00.000Z",
  "endDate": "2024-12-31T23:59:59.999Z"
}

Response: ZIP file download
```

## Features in Detail

### Receipt Explorer
- DataGrid view with pagination
- Filter by model ID
- Date range filtering
- View full receipt JSON in modal dialog

### Model Lineage Viewer
- Bar chart visualization using Recharts
- Date range filtering
- Real-time data updates

### Privacy Budget Dashboard
- Progress bars for epsilon and delta budgets
- Color-coded status (green/warning/error)
- Card-based layout for multiple models

### Audit Export Wizard
- Step 1: Select models (multi-select)
- Step 2: Choose date range
- Step 3: Confirm and download ZIP file

## Theme Support

The dashboard supports both light and dark modes. The preference is persisted in localStorage and can be toggled using the sun/moon icon in the app bar.

## License

MIT
