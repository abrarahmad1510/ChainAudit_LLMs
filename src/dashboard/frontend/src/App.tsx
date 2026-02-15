import { lazy, Suspense, useMemo } from 'react';
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { ApolloClient, InMemoryCache, createHttpLink } from '@apollo/client';
import { ApolloProvider } from '@apollo/client/react';
import { ThemeProvider, CssBaseline, Box, CircularProgress } from '@mui/material';
import { LocalizationProvider } from '@mui/x-date-pickers/LocalizationProvider';
import { AdapterDateFns } from '@mui/x-date-pickers/AdapterDateFns';
import { getTheme } from './theme';
import { useThemeMode } from './hooks/useThemeMode';
import { Layout } from './components/Layout';

const ReceiptExplorer = lazy(() =>
  import('./components/ReceiptExplorer').then((module) => ({
    default: module.ReceiptExplorer,
  }))
);

const ModelLineageViewer = lazy(() =>
  import('./components/ModelLineageViewer').then((module) => ({
    default: module.ModelLineageViewer,
  }))
);

const PrivacyBudgetDashboard = lazy(() =>
  import('./components/PrivacyBudgetDashboard').then((module) => ({
    default: module.PrivacyBudgetDashboard,
  }))
);

const AuditExportWizard = lazy(() =>
  import('./components/AuditExportWizard').then((module) => ({
    default: module.AuditExportWizard,
  }))
);

const LoadingFallback = () => (
  <Box sx={{ display: 'flex', justifyContent: 'center', alignItems: 'center', minHeight: '50vh' }}>
    <CircularProgress />
  </Box>
);

function AppContent() {
  return (
    <BrowserRouter>
      <Layout>
        <Suspense fallback={<LoadingFallback />}>
          <Routes>
            <Route path="/" element={<ReceiptExplorer />} />
            <Route path="/lineage" element={<ModelLineageViewer />} />
            <Route path="/privacy" element={<PrivacyBudgetDashboard />} />
            <Route path="/export" element={<AuditExportWizard />} />
          </Routes>
        </Suspense>
      </Layout>
    </BrowserRouter>
  );
}

function App() {
  const { mode } = useThemeMode();
  const theme = useMemo(() => getTheme(mode), [mode]);

  // Create an HTTP link for Apollo Client
  const httpLink = useMemo(
    () =>
      createHttpLink({
        uri: import.meta.env.VITE_GRAPHQL_URL || 'http://localhost:8080/graphql',
      }),
    []
  );

  const client = useMemo(
    () =>
      new ApolloClient({
        link: httpLink,
        cache: new InMemoryCache(),
      }),
    [httpLink]
  );

  return (
    <ApolloProvider client={client}>
      <ThemeProvider theme={theme}>
        <LocalizationProvider dateAdapter={AdapterDateFns}>
          <CssBaseline />
          <AppContent />
        </LocalizationProvider>
      </ThemeProvider>
    </ApolloProvider>
  );
}

export default App;