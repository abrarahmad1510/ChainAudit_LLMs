import { gql } from '@apollo/client';
import { useQuery } from '@apollo/client/react';
import {
  Box,
  Typography,
  Paper,
  Grid,
  Card,
  CardContent,
  LinearProgress,
  Alert,
  CircularProgress,
} from '@mui/material';

const PRIVACY_BUDGET_QUERY = gql`
  query PrivacyBudget {
    privacyBudgets {
      modelId
      epsilon
      delta
      remainingEpsilon
      remainingDelta
    }
  }
`;

interface PrivacyBudget {
  modelId: string;
  epsilon: number;
  delta: number;
  remainingEpsilon: number;
  remainingDelta: number;
}

interface PrivacyBudgetData {
  privacyBudgets: PrivacyBudget[];
}

export function PrivacyBudgetDashboard() {
  const { loading, error, data } = useQuery<PrivacyBudgetData>(PRIVACY_BUDGET_QUERY);

  const calculatePercentage = (remaining: number, total: number) => {
    if (total === 0) return 0;
    return (remaining / total) * 100;
  };

  const getColor = (percentage: number) => {
    if (percentage > 50) return 'success';
    if (percentage > 25) return 'warning';
    return 'error';
  };

  if (loading) {
    return (
      <Box>
        <Typography variant="h4" gutterBottom>
          Privacy Budget Dashboard
        </Typography>
        <Box sx={{ display: 'flex', justifyContent: 'center', p: 4 }}>
          <CircularProgress />
        </Box>
      </Box>
    );
  }

  if (error) {
    return (
      <Box>
        <Typography variant="h4" gutterBottom>
          Privacy Budget Dashboard
        </Typography>
        <Alert severity="info">
          Privacy budget tracking is coming soon. The backend endpoint is not yet implemented.
        </Alert>
      </Box>
    );
  }

  return (
    <Box>
      <Typography variant="h4" gutterBottom>
        Privacy Budget Dashboard
      </Typography>

      {data && data.privacyBudgets.length === 0 ? (
        <Alert severity="info">
          No privacy budget data available. This feature will track differential privacy budgets
          for each model.
        </Alert>
      ) : (
        <Grid container spacing={3}>
          {data?.privacyBudgets.map((budget) => {
            const epsilonPercentage = calculatePercentage(
              budget.remainingEpsilon,
              budget.epsilon
            );
            const deltaPercentage = calculatePercentage(
              budget.remainingDelta,
              budget.delta
            );

            return (
              <Grid item xs={12} md={6} key={budget.modelId}>
                <Card>
                  <CardContent>
                    <Typography variant="h6" gutterBottom>
                      {budget.modelId}
                    </Typography>

                    <Box sx={{ mb: 3 }}>
                      <Box sx={{ display: 'flex', justifyContent: 'space-between', mb: 1 }}>
                        <Typography variant="body2" color="text.secondary">
                          Epsilon Budget
                        </Typography>
                        <Typography variant="body2" color="text.secondary">
                          {budget.remainingEpsilon.toFixed(2)} / {budget.epsilon.toFixed(2)}
                        </Typography>
                      </Box>
                      <LinearProgress
                        variant="determinate"
                        value={epsilonPercentage}
                        color={getColor(epsilonPercentage)}
                        sx={{ height: 8, borderRadius: 1 }}
                      />
                    </Box>

                    <Box>
                      <Box sx={{ display: 'flex', justifyContent: 'space-between', mb: 1 }}>
                        <Typography variant="body2" color="text.secondary">
                          Delta Budget
                        </Typography>
                        <Typography variant="body2" color="text.secondary">
                          {budget.remainingDelta.toExponential(2)} /{' '}
                          {budget.delta.toExponential(2)}
                        </Typography>
                      </Box>
                      <LinearProgress
                        variant="determinate"
                        value={deltaPercentage}
                        color={getColor(deltaPercentage)}
                        sx={{ height: 8, borderRadius: 1 }}
                      />
                    </Box>
                  </CardContent>
                </Card>
              </Grid>
            );
          })}
        </Grid>
      )}
    </Box>
  );
}