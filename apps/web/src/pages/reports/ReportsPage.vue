<template>
  <MainLayout>
    <!-- Header Section -->
    <div class="page-header">
      <div>
        <h1>Financial & Tax Reports</h1>
        <p class="page-title-desc">Generate real-time cash position, aging profiles, financial statements, ledgers, and tax summaries.</p>
      </div>
    </div>

    <!-- Alert Notifications -->
    <div v-if="errorMsg" class="alert alert-danger alert-dismissible">
      <span>{{ errorMsg }}</span>
      <button class="alert-close" @click="errorMsg = null">&times;</button>
    </div>

    <!-- Navigation Tabs -->
    <div class="tabs-container">
      <div class="tabs-scroll-wrapper">
        <div class="tabs">
          <button 
            :class="['tab-btn', activeTab === 'cash-position' ? 'active' : '']" 
            @click="changeTab('cash-position')"
          >
            🏦 Cash Position
          </button>
          <button 
            :class="['tab-btn', activeTab === 'profit-loss' ? 'active' : '']" 
            @click="changeTab('profit-loss')"
          >
            📈 Profit & Loss
          </button>
          <button 
            :class="['tab-btn', activeTab === 'ar-aging' ? 'active' : '']" 
            @click="changeTab('ar-aging')"
          >
            📤 A/R Aging
          </button>
          <button 
            :class="['tab-btn', activeTab === 'ap-aging' ? 'active' : '']" 
            @click="changeTab('ap-aging')"
          >
            📥 A/P Aging
          </button>
          <button 
            :class="['tab-btn', activeTab === 'trial-balance' ? 'active' : '']" 
            @click="changeTab('trial-balance')"
          >
            ⚖️ Trial Balance
          </button>
          <button 
            :class="['tab-btn', activeTab === 'general-ledger' ? 'active' : '']" 
            @click="changeTab('general-ledger')"
          >
            📖 General Ledger
          </button>
          <button 
            :class="['tab-btn', activeTab === 'tax-summary' ? 'active' : '']" 
            @click="changeTab('tax-summary')"
          >
            💸 Tax Summary
          </button>
        </div>
      </div>
    </div>

    <!-- Dynamic Filter Bar -->
    <div class="card filter-card">
      <form @submit.prevent="applyFilters" class="filter-form">
        <!-- Date / As Of Filter -->
        <div v-if="isAsOfReport" class="filter-group">
          <label class="form-label">As Of Date</label>
          <input v-model="filterAsOfDate" type="date" class="form-input" required />
        </div>

        <template v-else>
          <div class="filter-group">
            <label class="form-label">Start Date</label>
            <input v-model="filterStartDate" type="date" class="form-input" required />
          </div>
          <div class="filter-group">
            <label class="form-label">End Date</label>
            <input v-model="filterEndDate" type="date" class="form-input" required />
          </div>
        </template>

        <!-- Account Selector (General Ledger only) -->
        <div v-if="activeTab === 'general-ledger'" class="filter-group filter-grow">
          <label class="form-label">Filter Account</label>
          <select v-model="filterAccountId" class="form-select">
            <option value="">All Accounts</option>
            <option v-for="acc in sortedAccounts" :key="acc.id" :value="acc.id">
              {{ acc.code }} - {{ acc.name }}
            </option>
          </select>
        </div>

        <!-- Period Shortcuts (For Range Reports) -->
        <div v-if="!isAsOfReport" class="filter-group shortcuts-group">
          <label class="form-label">Quick Periods</label>
          <div class="shortcuts-buttons">
            <button type="button" class="btn-shortcut" @click="setQuickPeriod('this-month')">This Month</button>
            <button type="button" class="btn-shortcut" @click="setQuickPeriod('last-month')">Last Month</button>
            <button type="button" class="btn-shortcut" @click="setQuickPeriod('this-quarter')">This Quarter</button>
            <button type="button" class="btn-shortcut" @click="setQuickPeriod('this-year')">This Year</button>
          </div>
        </div>

        <!-- Filter Action Buttons -->
        <div class="filter-actions">
          <button type="submit" class="btn btn-primary" :disabled="loading">
            {{ loading ? 'Loading...' : 'Generate Report' }}
          </button>
        </div>
      </form>
    </div>

    <!-- Report Contents -->
    <div class="report-content-area">
      <!-- Loading State -->
      <div v-if="loading" class="loading-state card">
        <div class="spinner"></div>
        <p>Calculating and generating dynamic report data, please wait...</p>
      </div>

      <!-- No Data State (but not loading) -->
      <div v-else-if="!hasReportData" class="empty-state card">
        <span class="empty-icon">📁</span>
        <h3>No Report Generated</h3>
        <p>Select your parameter filters above and click "Generate Report".</p>
      </div>

      <!-- Render Tab Panels -->
      <template v-else>
        <!-- 1. Cash Position -->
        <div v-if="activeTab === 'cash-position' && cashReport" class="tab-pane">
          <div class="report-summary-cards">
            <div class="summary-card cash-card">
              <div class="card-glow"></div>
              <div class="card-content">
                <span class="card-title">Total Cash Balance</span>
                <h2 class="card-amount text-primary-gradient">{{ formatIDR(cashReport.totalCash) }}</h2>
                <p class="card-desc">Aggregated balance as of {{ formatDate(cashReport.asOf) }}</p>
              </div>
            </div>
          </div>

          <div class="card table-wrapper-card">
            <h3 class="table-title">Cash & Bank Accounts</h3>
            <div class="table-container">
              <table>
                <thead>
                  <tr>
                    <th style="width: 80px;">No</th>
                    <th>Bank Name</th>
                    <th>Account Name</th>
                    <th class="text-right">Balance</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="(acc, index) in cashReport.accounts" :key="index">
                    <td>{{ index + 1 }}</td>
                    <td class="font-bold">{{ acc.bankName }}</td>
                    <td>{{ acc.accountName }}</td>
                    <td class="text-right font-mono font-bold">{{ formatIDR(acc.balance) }}</td>
                  </tr>
                  <tr v-if="cashReport.accounts.length === 0">
                    <td colspan="4" class="text-center text-muted">No cash or bank accounts found.</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>

        <!-- 2. Profit & Loss -->
        <div v-if="activeTab === 'profit-loss' && profitLossReport" class="tab-pane">
          <div class="report-summary-cards">
            <div class="summary-card revenue-card">
              <div class="card-content">
                <span class="card-title">Total Revenue</span>
                <h2 class="card-amount">{{ formatIDR(profitLossReport.totalRevenue) }}</h2>
                <p class="card-desc">Period: {{ formatDate(profitLossReport.periodFrom) }} - {{ formatDate(profitLossReport.periodTo) }}</p>
              </div>
            </div>
            <div class="summary-card expense-card">
              <div class="card-content">
                <span class="card-title">Total Expense</span>
                <h2 class="card-amount text-danger">{{ formatIDR(profitLossReport.totalExpense) }}</h2>
                <p class="card-desc">Period: {{ formatDate(profitLossReport.periodFrom) }} - {{ formatDate(profitLossReport.periodTo) }}</p>
              </div>
            </div>
            <div :class="['summary-card', profitLossReport.netProfit >= 0 ? 'profit-positive-card' : 'profit-negative-card']">
              <div class="card-glow"></div>
              <div class="card-content">
                <span class="card-title">Net Profit / (Loss)</span>
                <h2 class="card-amount">{{ formatIDR(profitLossReport.netProfit) }}</h2>
                <p class="card-desc">{{ profitLossReport.netProfit >= 0 ? 'Net income earned' : 'Net deficit incurred' }}</p>
              </div>
            </div>
          </div>

          <div class="pl-layout-grid">
            <!-- Revenue Accounts Panel -->
            <div class="card table-wrapper-card">
              <div class="panel-header">
                <h3>Revenue Details</h3>
                <span class="badge badge-success">Total: {{ formatIDR(profitLossReport.totalRevenue) }}</span>
              </div>
              <div class="table-container">
                <table>
                  <thead>
                    <tr>
                      <th style="width: 140px;">Code</th>
                      <th>Account Name</th>
                      <th class="text-right" style="width: 160px;">Balance</th>
                    </tr>
                  </thead>
                  <tbody>
                    <tr v-for="acc in profitLossReport.revenueAccounts" :key="acc.accountId">
                      <td class="font-mono text-muted">{{ acc.accountCode }}</td>
                      <td>{{ acc.accountName }}</td>
                      <td class="text-right font-mono font-bold">{{ formatIDR(acc.balance) }}</td>
                    </tr>
                    <tr v-if="profitLossReport.revenueAccounts.length === 0">
                      <td colspan="3" class="text-center text-muted">No revenue recorded in this period.</td>
                    </tr>
                  </tbody>
                </table>
              </div>
            </div>

            <!-- Expense Accounts Panel -->
            <div class="card table-wrapper-card">
              <div class="panel-header">
                <h3>Expense Details</h3>
                <span class="badge badge-danger">Total: {{ formatIDR(profitLossReport.totalExpense) }}</span>
              </div>
              <div class="table-container">
                <table>
                  <thead>
                    <tr>
                      <th style="width: 140px;">Code</th>
                      <th>Account Name</th>
                      <th class="text-right" style="width: 160px;">Balance</th>
                    </tr>
                  </thead>
                  <tbody>
                    <tr v-for="acc in profitLossReport.expenseAccounts" :key="acc.accountId">
                      <td class="font-mono text-muted">{{ acc.accountCode }}</td>
                      <td>{{ acc.accountName }}</td>
                      <td class="text-right font-mono font-bold text-danger">{{ formatIDR(acc.balance) }}</td>
                    </tr>
                    <tr v-if="profitLossReport.expenseAccounts.length === 0">
                      <td colspan="3" class="text-center text-muted">No expenses recorded in this period.</td>
                    </tr>
                  </tbody>
                </table>
              </div>
            </div>
          </div>
        </div>

        <!-- 3. Accounts Receivable Aging -->
        <div v-if="activeTab === 'ar-aging' && arAgingReport" class="tab-pane">
          <div class="report-summary-cards">
            <div class="summary-card ar-card">
              <div class="card-glow"></div>
              <div class="card-content">
                <span class="card-title">Total Outstanding A/R</span>
                <h2 class="card-amount text-primary-gradient">{{ formatIDR(arAgingReport.totalOutstanding) }}</h2>
                <p class="card-desc">Customer receivables aging as of {{ formatDate(arAgingReport.asOf) }}</p>
              </div>
            </div>
          </div>

          <div class="card table-wrapper-card">
            <h3 class="table-title">Customer Aging Breakdown</h3>
            <div class="table-container">
              <table>
                <thead>
                  <tr>
                    <th>Customer Name</th>
                    <th class="text-right">Current</th>
                    <th class="text-right">1 - 30 Days</th>
                    <th class="text-right">31 - 60 Days</th>
                    <th class="text-right">61 - 90 Days</th>
                    <th class="text-right">Over 90 Days</th>
                    <th class="text-right">Total Outstanding</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="(line, index) in arAgingReport.lines" :key="index">
                    <td class="font-bold">{{ line.counterpartyName }}</td>
                    <td class="text-right font-mono">{{ formatIDR(line.current) }}</td>
                    <td class="text-right font-mono text-warning">{{ formatIDR(line.days130) }}</td>
                    <td class="text-right font-mono text-warning">{{ formatIDR(line.days3160) }}</td>
                    <td class="text-right font-mono text-danger">{{ formatIDR(line.days6190) }}</td>
                    <td class="text-right font-mono text-danger font-bold">{{ formatIDR(line.days90Plus) }}</td>
                    <td class="text-right font-mono font-bold bg-muted">{{ formatIDR(line.totalOutstanding) }}</td>
                  </tr>
                  <tr v-if="arAgingReport.lines.length === 0">
                    <td colspan="7" class="text-center text-muted">No outstanding accounts receivable found.</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>

        <!-- 4. Accounts Payable Aging -->
        <div v-if="activeTab === 'ap-aging' && apAgingReport" class="tab-pane">
          <div class="report-summary-cards">
            <div class="summary-card ap-card">
              <div class="card-glow"></div>
              <div class="card-content">
                <span class="card-title">Total Outstanding A/P</span>
                <h2 class="card-amount text-danger-gradient">{{ formatIDR(apAgingReport.totalOutstanding) }}</h2>
                <p class="card-desc">Supplier payables aging as of {{ formatDate(apAgingReport.asOf) }}</p>
              </div>
            </div>
          </div>

          <div class="card table-wrapper-card">
            <h3 class="table-title">Supplier Aging Breakdown</h3>
            <div class="table-container">
              <table>
                <thead>
                  <tr>
                    <th>Supplier Name</th>
                    <th class="text-right">Current</th>
                    <th class="text-right">1 - 30 Days</th>
                    <th class="text-right">31 - 60 Days</th>
                    <th class="text-right">61 - 90 Days</th>
                    <th class="text-right">Over 90 Days</th>
                    <th class="text-right">Total Outstanding</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="(line, index) in apAgingReport.lines" :key="index">
                    <td class="font-bold">{{ line.counterpartyName }}</td>
                    <td class="text-right font-mono">{{ formatIDR(line.current) }}</td>
                    <td class="text-right font-mono text-warning">{{ formatIDR(line.days130) }}</td>
                    <td class="text-right font-mono text-warning">{{ formatIDR(line.days3160) }}</td>
                    <td class="text-right font-mono text-danger">{{ formatIDR(line.days6190) }}</td>
                    <td class="text-right font-mono text-danger font-bold">{{ formatIDR(line.days90Plus) }}</td>
                    <td class="text-right font-mono font-bold bg-muted">{{ formatIDR(line.totalOutstanding) }}</td>
                  </tr>
                  <tr v-if="apAgingReport.lines.length === 0">
                    <td colspan="7" class="text-center text-muted">No outstanding accounts payable found.</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>

        <!-- 5. Trial Balance -->
        <div v-if="activeTab === 'trial-balance' && trialBalanceReport" class="tab-pane">
          <div class="report-summary-cards">
            <div class="summary-card debit-card">
              <div class="card-content">
                <span class="card-title">Total Debit</span>
                <h2 class="card-amount">{{ formatIDR(trialBalanceReport.totalDebit) }}</h2>
                <p class="card-desc">Sum of debit balances as of {{ formatDate(trialBalanceReport.asOf) }}</p>
              </div>
            </div>
            <div class="summary-card credit-card">
              <div class="card-content">
                <span class="card-title">Total Credit</span>
                <h2 class="card-amount">{{ formatIDR(trialBalanceReport.totalCredit) }}</h2>
                <p class="card-desc">Sum of credit balances as of {{ formatDate(trialBalanceReport.asOf) }}</p>
              </div>
            </div>
            <div :class="['summary-card', isTbBalanced ? 'profit-positive-card' : 'profit-negative-card']">
              <div class="card-glow"></div>
              <div class="card-content">
                <span class="card-title">Balance Status</span>
                <h2 class="card-amount">{{ isTbBalanced ? 'Balanced ⚖️' : 'Out of Balance 🚨' }}</h2>
                <p class="card-desc">Difference: {{ formatIDR(Math.abs(trialBalanceReport.totalDebit - trialBalanceReport.totalCredit)) }}</p>
              </div>
            </div>
          </div>

          <div class="card table-wrapper-card">
            <h3 class="table-title">Trial Balance Ledger</h3>
            <div class="table-container">
              <table>
                <thead>
                  <tr>
                    <th style="width: 160px;">Account Code</th>
                    <th>Account Name</th>
                    <th class="text-right" style="width: 220px;">Debit Balance</th>
                    <th class="text-right" style="width: 220px;">Credit Balance</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="line in trialBalanceReport.lines" :key="line.accountId">
                    <td class="font-mono text-muted">{{ line.accountCode }}</td>
                    <td class="font-bold">{{ line.accountName }}</td>
                    <td class="text-right font-mono">{{ line.debitBalance > 0 ? formatIDR(line.debitBalance) : '-' }}</td>
                    <td class="text-right font-mono">{{ line.creditBalance > 0 ? formatIDR(line.creditBalance) : '-' }}</td>
                  </tr>
                  <tr class="table-footer-row font-bold bg-muted">
                    <td>TOTAL</td>
                    <td>Total Balances</td>
                    <td class="text-right font-mono">{{ formatIDR(trialBalanceReport.totalDebit) }}</td>
                    <td class="text-right font-mono">{{ formatIDR(trialBalanceReport.totalCredit) }}</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>

        <!-- 6. General Ledger -->
        <div v-if="activeTab === 'general-ledger' && generalLedgerReport" class="tab-pane">
          <div class="gl-header-info">
            <h2>General Ledger Statement</h2>
            <p class="text-muted">Period: {{ formatDate(generalLedgerReport.startDate) }} to {{ formatDate(generalLedgerReport.endDate) }}</p>
          </div>

          <div v-for="group in generalLedgerReport.accounts" :key="group.accountId" class="gl-account-card card">
            <div class="gl-account-header">
              <div>
                <span class="gl-account-code font-mono">{{ group.accountCode }}</span>
                <h3 class="gl-account-name">{{ group.accountName }}</h3>
              </div>
              <div class="gl-balances">
                <div class="balance-item">
                  <span class="balance-label">Opening Balance:</span>
                  <span class="balance-val font-mono">{{ formatIDR(group.openingBalance) }}</span>
                </div>
                <div class="balance-item font-bold">
                  <span class="balance-label">Closing Balance:</span>
                  <span class="balance-val font-mono">{{ formatIDR(group.closingBalance) }}</span>
                </div>
              </div>
            </div>

            <div class="table-container no-margin">
              <table>
                <thead>
                  <tr>
                    <th style="width: 140px;">Date</th>
                    <th style="width: 160px;">Reference</th>
                    <th>Description</th>
                    <th class="text-right" style="width: 160px;">Debit</th>
                    <th class="text-right" style="width: 160px;">Credit</th>
                    <th class="text-right" style="width: 180px;">Running Balance</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="(line, idx) in group.lines" :key="idx">
                    <td>{{ formatDate(line.transactionDate) }}</td>
                    <td class="font-mono text-secondary font-bold">{{ line.referenceNumber || '-' }}</td>
                    <td>{{ line.description }}</td>
                    <td class="text-right font-mono text-success">{{ line.debit > 0 ? formatIDR(line.debit) : '-' }}</td>
                    <td class="text-right font-mono text-danger">{{ line.credit > 0 ? formatIDR(line.credit) : '-' }}</td>
                    <td class="text-right font-mono font-bold">{{ formatIDR(line.runningBalance) }}</td>
                  </tr>
                  <tr v-if="group.lines.length === 0">
                    <td colspan="6" class="text-center text-muted">No journal postings found for this account in the selected period.</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>

          <div v-if="generalLedgerReport.accounts.length === 0" class="card text-center text-muted py-8">
            No accounts match the selected filters or contain transactions.
          </div>
        </div>

        <!-- 7. Tax Summary -->
        <div v-if="activeTab === 'tax-summary' && taxSummaryReport" class="tab-pane">
          <div class="report-summary-cards">
            <div class="summary-card output-card">
              <div class="card-glow"></div>
              <div class="card-content">
                <span class="card-title">VAT Output (PPN Keluaran)</span>
                <h2 class="card-amount">{{ formatIDR(taxSummaryReport.totalVatOutput) }}</h2>
                <p class="card-desc">Collected from sales invoices</p>
              </div>
            </div>
            <div class="summary-card input-card">
              <div class="card-glow"></div>
              <div class="card-content">
                <span class="card-title">VAT Input (PPN Masukan)</span>
                <h2 class="card-amount">{{ formatIDR(taxSummaryReport.totalVatInput) }}</h2>
                <p class="card-desc">Paid on purchase invoices</p>
              </div>
            </div>
            <div :class="['summary-card', taxSummaryReport.netTaxDue >= 0 ? 'payable-card' : 'refundable-card']">
              <div class="card-glow"></div>
              <div class="card-content">
                <span class="card-title">
                  {{ taxSummaryReport.netTaxDue >= 0 ? 'Net VAT Payable' : 'Net VAT Receivable (Credit)' }}
                </span>
                <h2 class="card-amount">{{ formatIDR(Math.abs(taxSummaryReport.netTaxDue)) }}</h2>
                <p class="card-desc">{{ taxSummaryReport.netTaxDue >= 0 ? 'Due to tax authority' : 'Available for refund / carryover' }}</p>
              </div>
            </div>
          </div>

          <div class="card table-wrapper-card">
            <h3 class="table-title">VAT Ledger Transaction Detail</h3>
            <div class="table-container">
              <table>
                <thead>
                  <tr>
                    <th>Period</th>
                    <th>Doc Type</th>
                    <th>Category</th>
                    <th>Counterparty</th>
                    <th class="text-right">Base Amount (DPP)</th>
                    <th class="text-right">Rate</th>
                    <th class="text-right">Tax Amount (PPN)</th>
                    <th>Status</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="record in taxSummaryReport.records" :key="record.id">
                    <td class="font-bold">{{ formatPeriod(record.taxPeriod) }}</td>
                    <td>
                      <span class="doc-link">
                        {{ formatDocType(record.sourceDocumentType) }}
                      </span>
                    </td>
                    <td>
                      <span :class="['badge', record.sourceDocumentType === 'SalesInvoice' ? 'badge-info' : 'badge-secondary']">
                        {{ record.sourceDocumentType === 'SalesInvoice' ? 'VAT Output' : 'VAT Input' }}
                      </span>
                    </td>
                    <td>
                      <div class="counterparty-info">
                        <span class="cp-name">{{ record.counterpartyName || '-' }}</span>
                        <span class="cp-npwp" v-if="record.counterpartyNpwp">NPWP: {{ record.counterpartyNpwp }}</span>
                      </div>
                    </td>
                    <td class="text-right font-mono">{{ formatIDR(record.taxBaseAmount) }}</td>
                    <td class="text-right">{{ (record.taxRate * 100).toFixed(0) }}%</td>
                    <td class="text-right font-mono font-bold">{{ formatIDR(record.taxAmount) }}</td>
                    <td>
                      <span :class="['badge', getStatusBadgeClass(record.status)]">
                        {{ formatStatusText(record.status) }}
                      </span>
                    </td>
                  </tr>
                  <tr v-if="taxSummaryReport.records.length === 0">
                    <td colspan="8" class="text-center text-muted">No tax records found in this range.</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </template>
    </div>
  </MainLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import MainLayout from '@/components/MainLayout.vue'
import { formatIDR, formatDate } from '@/utils/format'
import { reportsApi } from '@/api/reports.api'
import { accountApi } from '@/api/master-data.api'
import { useAuthStore } from '@/stores/auth.store'
import type { Account } from '@/types/master-data.types'
import type { 
  CashPositionReport, 
  ProfitLossReport, 
  AgingReport, 
  TrialBalanceReport, 
  GeneralLedgerReport 
} from '@/types/report.types'
import type { TaxSummaryResponse } from '@/types/master-data.types'

const authStore = useAuthStore()

// State variables
const activeTab = ref<
  'cash-position' | 'profit-loss' | 'ar-aging' | 'ap-aging' | 'trial-balance' | 'general-ledger' | 'tax-summary'
>('cash-position')

const loading = ref(false)
const errorMsg = ref<string | null>(null)
const accounts = ref<Account[]>([])

// Filter parameters state
const now = new Date()
const defaultStartDate = new Date(now.getFullYear(), now.getMonth(), 1).toISOString().split('T')[0]
const defaultEndDate = new Date(now.getFullYear(), now.getMonth() + 1, 0).toISOString().split('T')[0]
const defaultAsOfDate = now.toISOString().split('T')[0]

const filterStartDate = ref(defaultStartDate)
const filterEndDate = ref(defaultEndDate)
const filterAsOfDate = ref(defaultAsOfDate)
const filterAccountId = ref('')

// Report Data States
const cashReport = ref<CashPositionReport | null>(null)
const profitLossReport = ref<ProfitLossReport | null>(null)
const arAgingReport = ref<AgingReport | null>(null)
const apAgingReport = ref<AgingReport | null>(null)
const trialBalanceReport = ref<TrialBalanceReport | null>(null)
const generalLedgerReport = ref<GeneralLedgerReport | null>(null)
const taxSummaryReport = ref<TaxSummaryResponse | null>(null)

// Computed helpers
const isAsOfReport = computed(() => {
  return ['cash-position', 'ar-aging', 'ap-aging', 'trial-balance'].includes(activeTab.value)
})

const sortedAccounts = computed(() => {
  return [...accounts.value].sort((a, b) => a.code.localeCompare(b.code))
})

const isTbBalanced = computed(() => {
  if (!trialBalanceReport.value) return false
  return Math.abs(trialBalanceReport.value.totalDebit - trialBalanceReport.value.totalCredit) < 1
})

const hasReportData = computed(() => {
  switch (activeTab.value) {
    case 'cash-position': return !!cashReport.value
    case 'profit-loss': return !!profitLossReport.value
    case 'ar-aging': return !!arAgingReport.value
    case 'ap-aging': return !!apAgingReport.value
    case 'trial-balance': return !!trialBalanceReport.value
    case 'general-ledger': return !!generalLedgerReport.value
    case 'tax-summary': return !!taxSummaryReport.value
    default: return false
  }
})

// Methods
function changeTab(tabName: typeof activeTab.value) {
  activeTab.value = tabName
  errorMsg.value = null
  // Automatically trigger fetch if we don't have cached data, or re-run
  fetchReportData()
}

async function fetchReportData() {
  const companyId = authStore.currentUser?.companyId
  if (!companyId) {
    errorMsg.value = 'Failed to identify current company. Please login again.'
    return
  }

  loading.value = true
  errorMsg.value = null

  try {
    switch (activeTab.value) {
      case 'cash-position':
        cashReport.value = await reportsApi.getCashPosition({ asOf: filterAsOfDate.value })
        break
      case 'profit-loss':
        profitLossReport.value = await reportsApi.getProfitLoss({ 
          periodFrom: filterStartDate.value, 
          periodTo: filterEndDate.value 
        })
        break
      case 'ar-aging':
        arAgingReport.value = await reportsApi.getAccountsReceivable({ asOf: filterAsOfDate.value })
        break
      case 'ap-aging':
        apAgingReport.value = await reportsApi.getAccountsPayable({ asOf: filterAsOfDate.value })
        break
      case 'trial-balance':
        trialBalanceReport.value = await reportsApi.getTrialBalance({ asOf: filterAsOfDate.value })
        break
      case 'general-ledger':
        generalLedgerReport.value = await reportsApi.getGeneralLedger({
          startDate: filterStartDate.value,
          endDate: filterEndDate.value,
          accountId: filterAccountId.value || undefined
        })
        break
      case 'tax-summary':
        taxSummaryReport.value = await reportsApi.getTaxSummary({
          startDate: filterStartDate.value,
          endDate: filterEndDate.value
        })
        break
    }
  } catch (err: any) {
    console.error('Failed to load report data', err)
    errorMsg.value = err.response?.data?.message || `Failed to generate report details. Please verify your connection.`
  } finally {
    loading.value = false
  }
}

async function loadAccounts() {
  const companyId = authStore.currentUser?.companyId
  if (!companyId) return
  try {
    accounts.value = await accountApi.listByCompany(companyId)
  } catch (err) {
    console.error('Failed to load chart of accounts filter', err)
  }
}

function applyFilters() {
  fetchReportData()
}

function setQuickPeriod(period: 'this-month' | 'last-month' | 'this-quarter' | 'this-year') {
  const today = new Date()
  let start = new Date()
  let end = new Date()

  switch (period) {
    case 'this-month':
      start = new Date(today.getFullYear(), today.getMonth(), 1)
      end = new Date(today.getFullYear(), today.getMonth() + 1, 0)
      break
    case 'last-month':
      start = new Date(today.getFullYear(), today.getMonth() - 1, 1)
      end = new Date(today.getFullYear(), today.getMonth(), 0)
      break
    case 'this-quarter':
      const q = Math.floor(today.getMonth() / 3)
      start = new Date(today.getFullYear(), q * 3, 1)
      end = new Date(today.getFullYear(), (q + 1) * 3, 0)
      break
    case 'this-year':
      start = new Date(today.getFullYear(), 0, 1)
      end = new Date(today.getFullYear(), 12, 0)
      break
  }

  filterStartDate.value = start.toISOString().split('T')[0]
  filterEndDate.value = end.toISOString().split('T')[0]
  fetchReportData()
}

// Formatting helpers for Tax Summary
function formatPeriod(periodStr: string): string {
  if (!periodStr) return ''
  // Handle standard dates or YYYY-MM period inputs
  try {
    const parts = periodStr.split('-')
    if (parts.length >= 2) {
      const year = parts[0]
      const monthIndex = parseInt(parts[1], 10) - 1
      const date = new Date(parseInt(year, 10), monthIndex, 1)
      return date.toLocaleDateString('id-ID', { year: 'numeric', month: 'long' })
    }
    return periodStr
  } catch {
    return periodStr
  }
}

function formatDocType(type: string): string {
  if (!type) return ''
  return type.replace(/([A-Z])/g, ' $1').trim()
}

function formatStatusText(status: string): string {
  if (!status) return ''
  return status.replace(/_/g, ' ').replace(/\b\w/g, c => c.toUpperCase())
}

function getStatusBadgeClass(status: string): string {
  switch (status.toLowerCase()) {
    case 'paid':
    case 'filed':
      return 'badge-success'
    case 'reported':
    case 'validated':
      return 'badge-info'
    case 'required':
    case 'drafted':
    case 'unpaid':
      return 'badge-warning'
    case 'not_required':
      return 'badge-secondary'
    default:
      return 'badge-secondary'
  }
}

onMounted(async () => {
  await loadAccounts()
  await fetchReportData()
})
</script>

<style scoped>
/* Page layout */
.tabs-container {
  margin-bottom: 24px;
  border-bottom: 1px solid var(--border-color);
}

.tabs-scroll-wrapper {
  overflow-x: auto;
  white-space: nowrap;
  -webkit-overflow-scrolling: touch;
}

.tabs {
  display: flex;
  gap: 8px;
  padding-bottom: 4px;
}

.tab-btn {
  background: none;
  border: none;
  color: var(--text-secondary);
  font-family: var(--font-heading);
  font-size: 0.95rem;
  font-weight: 500;
  padding: 12px 18px;
  cursor: pointer;
  transition: all var(--transition-fast);
  border-radius: var(--radius-sm) var(--radius-sm) 0 0;
  display: inline-flex;
  align-items: center;
  gap: 8px;
}

.tab-btn:hover {
  color: var(--text-primary);
  background-color: var(--bg-tertiary);
}

.tab-btn.active {
  color: var(--accent-primary);
  font-weight: 600;
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-bottom-color: transparent;
  position: relative;
  z-index: 2;
}

/* Filter Card Layout */
.filter-card {
  padding: 20px;
  margin-bottom: 30px;
}

.filter-form {
  display: flex;
  flex-wrap: wrap;
  align-items: flex-end;
  gap: 16px;
}

.filter-group {
  flex: 1 1 200px;
  min-width: 150px;
}

.filter-grow {
  flex-grow: 2;
}

.shortcuts-group {
  flex: 1 1 300px;
}

.shortcuts-buttons {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.btn-shortcut {
  background-color: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  color: var(--text-secondary);
  padding: 8px 12px;
  border-radius: var(--radius-sm);
  font-size: 0.8rem;
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.btn-shortcut:hover {
  background-color: var(--border-color);
  color: var(--text-primary);
}

.filter-actions {
  display: flex;
  gap: 12px;
}

/* Summary Cards Grid */
.report-summary-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: 20px;
  margin-bottom: 30px;
}

.summary-card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  padding: 24px;
  position: relative;
  overflow: hidden;
  box-shadow: var(--shadow-sm);
}

.card-glow {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  opacity: 0.05;
  background: radial-gradient(circle at top right, var(--accent-primary), transparent 60%);
  pointer-events: none;
}

.card-content {
  position: relative;
  z-index: 1;
}

.card-title {
  font-size: 0.875rem;
  color: var(--text-secondary);
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  display: block;
  margin-bottom: 8px;
}

.card-amount {
  font-family: var(--font-heading);
  font-size: 2rem;
  font-weight: 700;
  color: var(--text-primary);
  line-height: 1.2;
  margin-bottom: 6px;
}

.text-primary-gradient {
  background: linear-gradient(135deg, var(--accent-primary) 0%, var(--accent-secondary) 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.text-danger-gradient {
  background: linear-gradient(135deg, var(--danger) 0%, #f43f5e 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.card-desc {
  font-size: 0.8rem;
  color: var(--text-muted);
}

.revenue-card {
  border-left: 4px solid var(--success);
}
.expense-card {
  border-left: 4px solid var(--danger);
}

.profit-positive-card {
  border-left: 4px solid var(--success);
  background: linear-gradient(to right, var(--bg-secondary), rgba(16, 185, 129, 0.02));
}

.profit-negative-card {
  border-left: 4px solid var(--danger);
  background: linear-gradient(to right, var(--bg-secondary), rgba(239, 68, 68, 0.02));
}

.payable-card {
  border-left: 4px solid var(--warning);
}
.refundable-card {
  border-left: 4px solid var(--success);
}

/* Tables and Panels */
.table-wrapper-card {
  padding: 24px;
  margin-bottom: 30px;
}

.table-title {
  font-family: var(--font-heading);
  font-size: 1.1rem;
  font-weight: 600;
  margin-bottom: 16px;
}

.pl-layout-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 24px;
  margin-bottom: 30px;
}

@media (max-width: 1024px) {
  .pl-layout-grid {
    grid-template-columns: 1fr;
  }
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.table-footer-row td {
  background-color: var(--bg-tertiary);
  font-weight: 700;
}

/* General Ledger Styling */
.gl-header-info {
  margin-bottom: 20px;
}

.gl-header-info h2 {
  font-size: 1.3rem;
  margin-bottom: 4px;
}

.gl-account-card {
  margin-bottom: 24px;
  padding: 20px;
}

.gl-account-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  border-bottom: 1px solid var(--border-color);
  padding-bottom: 16px;
  margin-bottom: 16px;
  flex-wrap: wrap;
  gap: 16px;
}

.gl-account-code {
  font-size: 0.85rem;
  color: var(--accent-primary);
  background-color: var(--accent-primary-glow);
  padding: 2px 8px;
  border-radius: var(--radius-sm);
  font-weight: 600;
  display: inline-block;
  margin-bottom: 4px;
}

.gl-account-name {
  font-size: 1.15rem;
  font-weight: 600;
}

.gl-balances {
  display: flex;
  gap: 24px;
}

.balance-item {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
}

.balance-label {
  font-size: 0.75rem;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.balance-val {
  font-size: 0.95rem;
  color: var(--text-primary);
  margin-top: 2px;
}

.no-margin {
  margin: 0;
}

.counterparty-info {
  display: flex;
  flex-direction: column;
}

.cp-name {
  font-weight: 500;
}

.cp-npwp {
  font-size: 0.75rem;
  color: var(--text-muted);
  margin-top: 2px;
}

.doc-link {
  font-weight: 500;
  color: var(--accent-primary);
}

/* Loading & Empty States */
.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  text-align: center;
  gap: 16px;
}

.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid var(--border-color);
  border-top-color: var(--accent-primary);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 80px 20px;
  text-align: center;
}

.empty-icon {
  font-size: 3rem;
  margin-bottom: 16px;
}

.empty-state h3 {
  font-size: 1.2rem;
  margin-bottom: 8px;
}

.empty-state p {
  color: var(--text-secondary);
  max-width: 400px;
}

/* Global alerts */
.alert {
  padding: 14px 20px;
  border-radius: var(--radius-sm);
  margin-bottom: 24px;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.alert-danger {
  background-color: var(--danger-bg);
  border: 1px solid rgba(239, 68, 68, 0.2);
  color: var(--danger);
}

.alert-close {
  background: none;
  border: none;
  color: currentColor;
  font-size: 1.2rem;
  cursor: pointer;
  opacity: 0.8;
  line-height: 1;
}

.alert-close:hover {
  opacity: 1;
}

.bg-muted {
  background-color: rgba(241, 245, 249, 0.5) !important;
}

.font-bold {
  font-weight: 600;
}
</style>
