export interface CashPositionReport {
  asOf: string
  totalCash: number
  currency: string
  accounts: CashAccountLine[]
}

export interface CashAccountLine {
  bankName: string
  accountName: string
  balance: number
}

export interface ProfitLossReport {
  periodFrom: string
  periodTo: string
  revenueAccounts: AccountBalanceLine[]
  totalRevenue: number
  expenseAccounts: AccountBalanceLine[]
  totalExpense: number
  netProfit: number
}

export interface AccountBalanceLine {
  accountId: string
  accountCode: string
  accountName: string
  balance: number
}

export interface AgingReport {
  asOf: string
  totalOutstanding: number
  lines: AgingLine[]
}

export interface AgingLine {
  counterpartyId?: string
  counterpartyName: string
  totalOutstanding: number
  current: number
  days130: number
  days3160: number
  days6190: number
  days90Plus: number
}

export interface TrialBalanceReport {
  asOf: string
  totalDebit: number
  totalCredit: number
  lines: TrialBalanceLine[]
}

export interface TrialBalanceLine {
  accountId: string
  accountCode: string
  accountName: string
  debitBalance: number
  creditBalance: number
}

export interface GeneralLedgerReport {
  startDate: string
  endDate: string
  accounts: GeneralLedgerAccountGroup[]
}

export interface GeneralLedgerAccountGroup {
  accountId: string
  accountCode: string
  accountName: string
  openingBalance: number
  closingBalance: number
  lines: GeneralLedgerLine[]
}

export interface GeneralLedgerLine {
  transactionDate: string
  referenceNumber: string
  description: string
  debit: number
  credit: number
  runningBalance: number
}
