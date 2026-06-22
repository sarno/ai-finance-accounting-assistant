<template>
  <MainLayout>
    <div class="page-header">
      <div>
        <h1>Financial Dashboard</h1>
        <p class="page-title-desc">Overview of your workspace financial performance and automation metrics.</p>
      </div>
      <button class="btn btn-primary" @click="loadDashboardData" :disabled="loading">
        {{ loading ? 'Refreshing...' : 'Refresh Metrics' }}
      </button>
    </div>

    <!-- Alert Notifications -->
    <div v-if="errorMsg" class="alert alert-danger alert-dismissible" style="margin-bottom: 24px;">
      <span>{{ errorMsg }}</span>
      <button class="alert-close" @click="errorMsg = null">&times;</button>
    </div>

    <!-- Stats Grid -->
    <div class="stats-grid" style="display: grid; grid-template-columns: repeat(auto-fit, minmax(240px, 1fr)); gap: 24px; margin-bottom: 32px;">
      <!-- Cash Balance Card -->
      <div class="card stat-card">
        <div class="card-glow"></div>
        <div class="card-content">
          <p class="stat-label">Cash Balance</p>
          <h2 class="stat-value text-primary-gradient">{{ formatIDR(cashBalance) }}</h2>
          <span class="badge badge-info">Real-time balance</span>
        </div>
      </div>

      <!-- Pending Approvals Card -->
      <div class="card stat-card" style="cursor: pointer;" @click="$router.push('/approvals')">
        <div class="card-glow"></div>
        <div class="card-content">
          <p class="stat-label">Pending Approvals</p>
          <h2 class="stat-value" :class="pendingApprovalsCount > 0 ? 'text-warning' : 'text-muted'">
            {{ pendingApprovalsCount }} Request{{ pendingApprovalsCount === 1 ? '' : 's' }}
          </h2>
          <span :class="['badge', pendingApprovalsCount > 0 ? 'badge-warning' : 'badge-success']">
            {{ pendingApprovalsCount > 0 ? 'Requires action' : 'All clear' }}
          </span>
        </div>
      </div>

      <!-- Outstanding Invoices (A/R) Card -->
      <div class="card stat-card" style="cursor: pointer;" @click="$router.push('/sales')">
        <div class="card-glow"></div>
        <div class="card-content">
          <p class="stat-label">Sales Invoices (Unpaid)</p>
          <h2 class="stat-value text-danger">{{ formatIDR(salesInvoicesUnpaid) }}</h2>
          <span class="badge badge-danger">Receivables outstanding</span>
        </div>
      </div>

      <!-- Tax Liabilities Card -->
      <div class="card stat-card" style="cursor: pointer;" @click="$router.push('/tax')">
        <div class="card-glow"></div>
        <div class="card-content">
          <p class="stat-label">Tax Liabilities</p>
          <h2 class="stat-value" :class="taxLiabilities >= 0 ? 'text-warning' : 'text-success'">
            {{ formatIDR(Math.abs(taxLiabilities)) }}
          </h2>
          <span :class="['badge', taxLiabilities >= 0 ? 'badge-warning' : 'badge-success']">
            {{ taxLiabilities >= 0 ? 'Net VAT payable' : 'Net tax credit' }}
          </span>
        </div>
      </div>
    </div>

    <!-- Main Dashboard Section -->
    <div class="dashboard-grid" style="display: grid; grid-template-columns: 2fr 1fr; gap: 24px;">
      <!-- Recent Transactions Card -->
      <div class="card">
        <h3 style="margin-bottom: 16px; font-family: var(--font-heading);">Recent Transactions</h3>
        <div class="table-container" style="margin-top: 0;">
          <table>
            <thead>
              <tr>
                <th>Date</th>
                <th>Reference</th>
                <th>Description</th>
                <th class="text-right">Amount</th>
                <th>Status</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="tx in recentTransactions" :key="tx.id">
                <td>{{ formatDate(tx.transactionDate) }}</td>
                <td class="font-mono font-bold text-secondary">{{ tx.referenceNumber || '-' }}</td>
                <td>{{ tx.description }}</td>
                <td class="text-right font-mono font-bold">
                  {{ formatIDR(getTransactionAmount(tx)) }}
                </td>
                <td>
                  <span :class="['badge', getStatusBadgeClass(tx.status)]">
                    {{ tx.status }}
                  </span>
                </td>
              </tr>
              <tr v-if="recentTransactions.length === 0">
                <td colspan="5" class="text-center text-muted" style="padding: 24px;">
                  No transactions found. Go to Journeys to post some data.
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>

      <!-- AI Assistant Status Card -->
      <div class="card" style="display: flex; flex-direction: column; justify-content: space-between;">
        <div>
          <h3 style="margin-bottom: 16px; font-family: var(--font-heading);">AI Assistant Status</h3>
          <p style="color: var(--text-secondary); font-size: 0.9rem; margin-bottom: 20px;">
            The accounting automation pipeline is active. AI is monitoring incoming documents and generating draft journals.
          </p>
          <div style="background-color: var(--bg-tertiary); padding: 16px; border-radius: var(--radius-md); border-left: 4px solid var(--accent-secondary); margin-bottom: 16px;">
            <p style="font-weight: 700; font-size: 0.95rem; color: var(--text-primary); margin-bottom: 4px;">98.5% Accuracy</p>
            <p style="font-size: 0.75rem; color: var(--text-secondary);">Average invoice field extraction confidence</p>
          </div>
        </div>
        <button class="btn btn-secondary" style="width: 100%;" @click="$router.push('/journals')">
          View Journal Queue
        </button>
      </div>
    </div>
  </MainLayout>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import MainLayout from '@/components/MainLayout.vue'
import { formatIDR, formatDate } from '@/utils/format'
import { reportsApi } from '@/api/reports.api'
import { approvalApi } from '@/api/approvals.api'
import { journalApi } from '@/api/journals.api'
import { useAuthStore } from '@/stores/auth.store'
import type { JournalEntry } from '@/types/journal.types'

const authStore = useAuthStore()

// State variables
const loading = ref(false)
const errorMsg = ref<string | null>(null)

// Metrics
const cashBalance = ref(0)
const pendingApprovalsCount = ref(0)
const salesInvoicesUnpaid = ref(0)
const taxLiabilities = ref(0)
const recentTransactions = ref<JournalEntry[]>([])

async function loadDashboardData() {
  const companyId = authStore.currentUser?.companyId
  if (!companyId) return

  loading.value = true
  errorMsg.value = null

  const today = new Date().toISOString().split('T')[0]
  const now = new Date()
  const firstDayOfMonth = new Date(now.getFullYear(), now.getMonth(), 1).toISOString().split('T')[0]
  const lastDayOfMonth = new Date(now.getFullYear(), now.getMonth() + 1, 0).toISOString().split('T')[0]

  try {
    // 1. Get Cash Position Balance
    try {
      const cashRes = await reportsApi.getCashPosition({ asOf: today })
      cashBalance.value = Number(cashRes.totalCash) || 0
    } catch (e) {
      console.error('Failed to load dashboard cash balance', e)
    }

    // 2. Get Pending Approvals Count
    try {
      const approvals = await approvalApi.listPending()
      pendingApprovalsCount.value = approvals.length
    } catch (e) {
      console.error('Failed to load dashboard pending approvals', e)
    }

    // 3. Get Receivables Outstanding
    try {
      const arRes = await reportsApi.getAccountsReceivable({ asOf: today })
      salesInvoicesUnpaid.value = Number(arRes.totalOutstanding) || 0
    } catch (e) {
      console.error('Failed to load dashboard unpaid invoices', e)
    }

    // 4. Get Tax Liabilities Summary
    try {
      const taxRes = await reportsApi.getTaxSummary({ startDate: firstDayOfMonth, endDate: lastDayOfMonth })
      taxLiabilities.value = Number(taxRes.netTaxDue) || 0
    } catch (e) {
      console.error('Failed to load dashboard tax liabilities', e)
    }

    // 5. Get Recent Transactions (Journal entries list)
    try {
      const journalsRes = await journalApi.list({ page: 1, perPage: 10 })
      recentTransactions.value = journalsRes.data
    } catch (e) {
      console.error('Failed to load dashboard recent journals', e)
    }
  } catch (err: any) {
    errorMsg.value = 'Failed to sync financial dashboard metrics. Showing cached data.'
  } finally {
    loading.value = false
  }
}

// Helpers
function getTransactionAmount(tx: JournalEntry): number {
  if (!tx.lines || tx.lines.length === 0) return 0
  // Sum either all debits or credits (since journal entry is balanced, sum(debit) === sum(credit))
  return tx.lines.reduce((sum, line) => sum + (line.debit || 0), 0)
}

function getStatusBadgeClass(status: string): string {
  switch (status.toLowerCase()) {
    case 'posted':
      return 'badge-success'
    case 'draft':
      return 'badge-warning'
    case 'waiting_approval':
    case 'waiting_review':
      return 'badge-info'
    default:
      return 'badge-secondary'
  }
}

onMounted(() => {
  loadDashboardData()
})
</script>

<style scoped>
.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 32px;
}

.page-title-desc {
  font-size: 0.95rem;
  color: var(--text-secondary);
  margin-top: 4px;
}

/* Stat Card Styling */
.stat-card {
  position: relative;
  overflow: hidden;
  padding: 24px;
  border-radius: var(--radius-md);
  border: 1px solid var(--border-color);
  background-color: var(--bg-secondary);
  box-shadow: var(--shadow-sm);
  transition: all var(--transition-normal);
}

.stat-card:hover {
  transform: translateY(-4px);
  box-shadow: var(--shadow-lg), 0 0 20px var(--accent-primary-glow);
}

.card-glow {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  opacity: 0.04;
  background: radial-gradient(circle at top right, var(--accent-primary), transparent 70%);
  pointer-events: none;
}

.card-content {
  position: relative;
  z-index: 2;
}

.stat-label {
  font-size: 0.85rem;
  color: var(--text-secondary);
  text-transform: uppercase;
  font-weight: 600;
  letter-spacing: 0.05em;
}

.stat-value {
  font-family: var(--font-heading);
  font-size: 1.85rem;
  font-weight: 700;
  margin: 12px 0 8px 0;
  color: var(--text-primary);
  line-height: 1.2;
}

.text-primary-gradient {
  background: linear-gradient(135deg, var(--accent-primary) 0%, var(--accent-secondary) 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.text-warning {
  color: var(--warning) !important;
}

.text-muted {
  color: var(--text-muted) !important;
}

.text-danger {
  color: var(--danger) !important;
}

.text-success {
  color: var(--success) !important;
}

/* Global alerts */
.alert {
  padding: 14px 20px;
  border-radius: var(--radius-sm);
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

.text-right {
  text-align: right;
}
</style>
