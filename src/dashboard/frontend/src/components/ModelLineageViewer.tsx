import { useState } from 'react';
import { gql } from '@apollo/client';
import { useQuery } from '@apollo/client/react';
import {
  Box,
  Paper,
  Typography,
  Alert,
  CircularProgress,
} from '@mui/material';
import { DateTimePicker } from '@mui/x-date-pickers/DateTimePicker';
import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
} from 'recharts';

const MODEL_USAGE_QUERY = gql`
  query ModelUsage($startTime: DateTime, $endTime: DateTime) {
    modelUsage(startTime: $startTime, endTime: $endTime) {
      modelId
      count
    }
  }
`;

interface ModelUsage {
  modelId: string;
  count: number;
}

interface ModelUsageData {
  modelUsage: ModelUsage[];
}

interface ModelUsageVars {
  startTime?: string;
  endTime?: string;
}

export function ModelLineageViewer() {
  const [startTime, setStartTime] = useState<Date | null>(null);
  const [endTime, setEndTime] = useState<Date | null>(null);

  const { loading, error, data, refetch } = useQuery<ModelUsageData, ModelUsageVars>(
    MODEL_USAGE_QUERY,
    {
      variables: {
        startTime: startTime?.toISOString(),
        endTime: endTime?.toISOString(),
      },
    }
  );

  const handleDateChange = () => {
    refetch({
      startTime: startTime?.toISOString(),
      endTime: endTime?.toISOString(),
    });
  };

  return (
    <Box>
      <Typography variant="h4" gutterBottom>
        Model Lineage Viewer
      </Typography>

      <Paper sx={{ p: 2, mb: 2 }}>
        <Box sx={{ display: 'flex', gap: 2, flexWrap: 'wrap', alignItems: 'center' }}>
          <DateTimePicker
            label="Start Time"
            value={startTime}
            onChange={(newValue) => {
              setStartTime(newValue);
              handleDateChange();
            }}
            slotProps={{ textField: { size: 'small' } }}
          />
          <DateTimePicker
            label="End Time"
            value={endTime}
            onChange={(newValue) => {
              setEndTime(newValue);
              handleDateChange();
            }}
            slotProps={{ textField: { size: 'small' } }}
          />
        </Box>
      </Paper>

      {loading && (
        <Box sx={{ display: 'flex', justifyContent: 'center', p: 4 }}>
          <CircularProgress />
        </Box>
      )}

      {error && (
        <Alert severity="error">
          Error loading model usage data: {error.message}
        </Alert>
      )}

      {!loading && !error && data && (
        <>
          {data.modelUsage.length === 0 ? (
            <Paper sx={{ p: 4, textAlign: 'center' }}>
              <Typography variant="body1" color="text.secondary">
                No model usage data available for the selected time range.
              </Typography>
            </Paper>
          ) : (
            <Paper sx={{ p: 2 }}>
              <ResponsiveContainer width="100%" height={400}>
                <BarChart
                  data={data.modelUsage}
                  margin={{
                    top: 5,
                    right: 30,
                    left: 20,
                    bottom: 5,
                  }}
                >
                  <CartesianGrid strokeDasharray="3 3" />
                  <XAxis dataKey="modelId" />
                  <YAxis />
                  <Tooltip />
                  <Legend />
                  <Bar dataKey="count" fill="#1976d2" name="Usage Count" />
                </BarChart>
              </ResponsiveContainer>
            </Paper>
          )}
        </>
      )}
    </Box>
  );
}
