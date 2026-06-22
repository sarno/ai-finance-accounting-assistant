import client from './client'
import type {
  CashPositionReport,
  ProfitLossReport,
  AgingReport,
  TrialBalanceReport,
  GeneralLedgerReport
} from '@/types/report.types'
import type { TaxSummaryResponse } from '@/types/master-data.types'

export const reportsApi = {
  getCashPosition: async (params: { asOf?: string }): Promise<CashPositionReport> => {
    const { data } = await client.get<CashPositionReport>('/reports/cash-position', { params })
    return data
  },
  getProfitLoss: async (params: { periodFrom?: string; periodTo?: string; period?: string }): Promise<ProfitLossReport> => {
    const { data } = await client.get<ProfitLossReport>('/reports/profit-loss', { params })
    return data
  },
  getAccountsReceivable: async (params: { asOf?: string }): Promise<AgingReport> => {
    const { data } = await client.get<AgingReport>('/reports/accounts-receivable', { params })
    return data
  },
  getAccountsPayable: async (params: { asOf?: string }): Promise<AgingReport> => {
    const { data } = await client.get<AgingReport>('/reports/accounts-payable', { params })
    return data
  },
  getTrialBalance: async (params: { asOf?: string }): Promise<TrialBalanceReport> => {
    const { data } = await client.get<TrialBalanceReport>('/reports/trial-balance', { params })
    return data
  },
  getGeneralLedger: async (params: { startDate?: string; endDate?: string; accountId?: string }): Promise<GeneralLedgerReport> => {
    const { data } = await client.get<GeneralLedgerReport>('/reports/general-ledger', { params })
    return data
  },
  getTaxSummary: async (params: { startDate?: string; endDate?: string; period?: string }): Promise<TaxSummaryResponse> => {
    const { data } = await client.get<TaxSummaryResponse>('/reports/tax-summary', { params })
    return data
  }
}
