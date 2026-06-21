<template>
  <MainLayout>
    <div class="page-header">
      <div>
        <h1>Financial Dashboard</h1>
        <p class="page-title-desc">Overview of your workspace financial performance and automation metrics.</p>
      </div>
      <button class="btn btn-primary">Refresh Metrics</button>
    </div>

    <!-- Stats Grid -->
    <div class="stats-grid" style="display: grid; grid-template-columns: repeat(auto-fit, minmax(240px, 1fr)); gap: 24px; margin-bottom: 32px;">
      <div class="card">
        <p class="stat-label" style="font-size: 0.85rem; color: var(--text-secondary); text-transform: uppercase; font-weight: 600;">Cash Balance</p>
        <h2 class="stat-value" style="font-size: 2rem; margin: 8px 0; color: #ffffff;">Rp 482,900,000</h2>
        <span class="badge badge-success">↑ 12% vs last month</span>
      </div>
      <div class="card">
        <p class="stat-label" style="font-size: 0.85rem; color: var(--text-secondary); text-transform: uppercase; font-weight: 600;">Pending Approvals</p>
        <h2 class="stat-value" style="font-size: 2rem; margin: 8px 0; color: #ffffff;">5 Requests</h2>
        <span class="badge badge-warning">Requires action</span>
      </div>
      <div class="card">
        <p class="stat-label" style="font-size: 0.85rem; color: var(--text-secondary); text-transform: uppercase; font-weight: 600;">Sales Invoices (Unpaid)</p>
        <h2 class="stat-value" style="font-size: 2rem; margin: 8px 0; color: #ffffff;">Rp 125,400,000</h2>
        <span class="badge badge-info">14 active invoices</span>
      </div>
      <div class="card">
        <p class="stat-label" style="font-size: 0.85rem; color: var(--text-secondary); text-transform: uppercase; font-weight: 600;">Tax Liabilities</p>
        <h2 class="stat-value" style="font-size: 2rem; margin: 8px 0; color: #ffffff;">Rp 18,920,000</h2>
        <span class="badge badge-danger">Due in 15 days</span>
      </div>
    </div>

    <!-- Main Dashboard Section -->
    <div class="dashboard-grid" style="display: grid; grid-template-columns: 2fr 1fr; gap: 24px;">
      <div class="card">
        <h3 style="margin-bottom: 16px;">Recent Transactions</h3>
        <div class="table-container" style="margin-top: 0;">
          <table>
            <thead>
              <tr>
                <th>Date</th>
                <th>Reference</th>
                <th>Description</th>
                <th>Amount</th>
                <th>Status</th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td>2026-06-20</td>
                <td>JNL-2026-0034</td>
                <td>Sales invoice collection BCA</td>
                <td style="font-weight: 600; color: var(--success);">+Rp 15,000,000</td>
                <td><span class="badge badge-success">Posted</span></td>
              </tr>
              <tr>
                <td>2026-06-19</td>
                <td>JNL-2026-0033</td>
                <td>Server hosting payment</td>
                <td style="font-weight: 600; color: var(--danger);">-Rp 2,400,000</td>
                <td><span class="badge badge-success">Posted</span></td>
              </tr>
              <tr>
                <td>2026-06-18</td>
                <td>JNL-2026-0032</td>
                <td>Draft: Office rent prepayment</td>
                <td style="font-weight: 600; color: var(--text-primary);">Rp 24,000,000</td>
                <td><span class="badge badge-warning">Draft</span></td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>

      <div class="card" style="display: flex; flex-direction: column; justify-content: space-between;">
        <div>
          <h3 style="margin-bottom: 16px;">AI Assistant Status</h3>
          <p style="color: var(--text-secondary); font-size: 0.9rem; margin-bottom: 16px;">
            The accounting automation pipeline is active. 8 OCR invoices extracted today.
          </p>
          <div style="background-color: var(--bg-primary); padding: 12px; border-radius: var(--radius-sm); border-left: 3px solid var(--accent-secondary); margin-bottom: 12px;">
            <p style="font-weight: 600; font-size: 0.85rem; color: #ffffff;">98.5% Accuracy</p>
            <p style="font-size: 0.75rem; color: var(--text-secondary);">Average invoice field extraction confidence</p>
          </div>
        </div>
        <button class="btn btn-secondary" style="width: 100%;">View OCR Queue</button>
      </div>
    </div>
  </MainLayout>
</template>

<script setup lang="ts">
import MainLayout from '@/components/MainLayout.vue'
</script>
