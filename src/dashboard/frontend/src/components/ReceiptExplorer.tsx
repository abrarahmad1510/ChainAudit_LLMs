import { useState } from 'react';
import { gql } from '@apollo/client';
import { useQuery } from '@apollo/client/react';
import {
  Box,
  TextField,
  Button,
  Paper,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  Typography,
  Alert,
  CircularProgress,
} from '@mui/material';
import { DataGrid, GridColDef, GridPaginationModel } from '@mui/x-data-grid';
import { DateTimePicker } from '@mui/x-date-pickers/DateTimePicker';

const GET_RECEIPTS = gql`
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
`;

interface Receipt {
  leafHash: string;
  leafIndex: number;
  rootHash: string;
  context: string;
  receiptJwt: string;
  createdAt: string;
}

interface ReceiptsData {
  receipts: Receipt[];
}

interface ReceiptsVars {
  limit?: number;
  offset?: number;
  modelId?: string;
  startTime?: string;
  endTime?: string;
}

export function ReceiptExplorer() {
  const [paginationModel, setPaginationModel] = useState<GridPaginationModel>({
    page: 0,
    pageSize: 10,
  });
  const [modelId, setModelId] = useState('');
  const [startTime, setStartTime] = useState<Date | null>(null);
  const [endTime, setEndTime] = useState<Date | null>(null);
  const [selectedReceipt, setSelectedReceipt] = useState<Receipt | null>(null);
  const [dialogOpen, setDialogOpen] = useState(false);

  const { loading, error, data, refetch } = useQuery<ReceiptsData, ReceiptsVars>(GET_RECEIPTS, {
    variables: {
      limit: paginationModel.pageSize,
      offset: paginationModel.page * paginationModel.pageSize,
    },
  });

  const handleSearch = () => {
    refetch({
      limit: paginationModel.pageSize,
      offset: paginationModel.page * paginationModel.pageSize,
      modelId: modelId || undefined,
      startTime: startTime?.toISOString(),
      endTime: endTime?.toISOString(),
    });
  };

  const handleViewReceipt = (receipt: Receipt) => {
    setSelectedReceipt(receipt);
    setDialogOpen(true);
  };

  const truncate = (str: string, maxLength: number = 20) => {
    if (str.length <= maxLength) return str;
    return `${str.substring(0, maxLength)}...`;
  };

  const columns: GridColDef[] = [
    {
      field: 'leafHash',
      headerName: 'Leaf Hash',
      width: 200,
      renderCell: (params) => truncate(params.value as string),
    },
    {
      field: 'leafIndex',
      headerName: 'Index',
      width: 100,
      type: 'number',
    },
    {
      field: 'rootHash',
      headerName: 'Root Hash',
      width: 200,
      renderCell: (params) => truncate(params.value as string),
    },
    {
      field: 'createdAt',
      headerName: 'Timestamp',
      width: 200,
      renderCell: (params) => new Date(params.value as string).toLocaleString(),
    },
    {
      field: 'actions',
      headerName: 'Actions',
      width: 120,
      renderCell: (params) => (
        <Button
          size="small"
          variant="outlined"
          onClick={() => handleViewReceipt(params.row as Receipt)}
        >
          View
        </Button>
      ),
    },
  ];

  return (
    <Box>
      <Typography variant="h4" gutterBottom>
        Receipt Explorer
      </Typography>

      <Paper sx={{ p: 2, mb: 2 }}>
        <Box sx={{ display: 'flex', gap: 2, flexWrap: 'wrap', alignItems: 'center' }}>
          <TextField
            label="Model ID"
            value={modelId}
            onChange={(e) => setModelId(e.target.value)}
            size="small"
            sx={{ minWidth: 200 }}
          />
          <DateTimePicker
            label="Start Time"
            value={startTime}
            onChange={(newValue) => setStartTime(newValue)}
            slotProps={{ textField: { size: 'small' } }}
          />
          <DateTimePicker
            label="End Time"
            value={endTime}
            onChange={(newValue) => setEndTime(newValue)}
            slotProps={{ textField: { size: 'small' } }}
          />
          <Button variant="contained" onClick={handleSearch}>
            Search
          </Button>
        </Box>
      </Paper>

      {error && (
        <Alert severity="error" sx={{ mb: 2 }}>
          Error loading receipts: {error.message}
        </Alert>
      )}

      <Paper sx={{ height: 600, width: '100%' }}>
        <DataGrid
          rows={data?.receipts || []}
          columns={columns}
          paginationModel={paginationModel}
          onPaginationModelChange={setPaginationModel}
          pageSizeOptions={[10, 25, 50]}
          loading={loading}
          getRowId={(row) => row.leafHash}
          disableRowSelectionOnClick
        />
      </Paper>

      <Dialog
        open={dialogOpen}
        onClose={() => setDialogOpen(false)}
        maxWidth="md"
        fullWidth
      >
        <DialogTitle>Receipt Details</DialogTitle>
        <DialogContent>
          {selectedReceipt && (
            <Box>
              <pre style={{ overflow: 'auto', fontSize: '0.875rem' }}>
                {JSON.stringify(selectedReceipt, null, 2)}
              </pre>
            </Box>
          )}
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setDialogOpen(false)}>Close</Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
}
