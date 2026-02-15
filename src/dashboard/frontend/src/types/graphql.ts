export interface Receipt {
  leafHash: string;
  leafIndex: number;
  rootHash: string;
  context: string;
  receiptJwt: string;
  createdAt: string;
}

export interface ModelUsage {
  modelId: string;
  count: number;
}

export interface PrivacyBudget {
  modelId: string;
  epsilon: number;
  delta: number;
  remainingEpsilon: number;
  remainingDelta: number;
}

export interface ReceiptsQueryVariables {
  limit?: number;
  offset?: number;
  modelId?: string;
  startTime?: string;
  endTime?: string;
}

export interface ModelUsageQueryVariables {
  startTime?: string;
  endTime?: string;
}

export interface ExportRequest {
  models: string[];
  startDate?: string;
  endDate?: string;
}
