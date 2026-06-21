<template>
  <div class="app-container">
    <!-- Sidebar -->
    <aside class="sidebar">
      <div class="brand">
        <svg class="brand-logo" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M12 2L2 7L12 12L22 7L12 2Z" stroke="#4f46e5" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          <path d="M2 17L12 22L22 17" stroke="#06b6d4" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          <path d="M2 12L12 17L22 12" stroke="#4f46e5" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        <span class="brand-name">Codex Finance</span>
      </div>

      <nav class="nav-menu">
        <router-link to="/dashboard" class="nav-link">
          <span class="nav-icon">📊</span> Dashboard
        </router-link>
        <router-link to="/journals" class="nav-link">
          <span class="nav-icon">📓</span> Journals
        </router-link>
        <router-link to="/sales" class="nav-link">
          <span class="nav-icon">📈</span> Sales Invoices
        </router-link>
        <router-link to="/purchases" class="nav-link">
          <span class="nav-icon">📉</span> Purchases
        </router-link>
        <router-link to="/payments" class="nav-link">
          <span class="nav-icon">💳</span> Payments
        </router-link>
        <router-link to="/approvals" class="nav-link">
          <span class="nav-icon">🛡️</span> Approvals
        </router-link>
        <router-link to="/tax" class="nav-link">
          <span class="nav-icon">⚖️</span> Tax Module
        </router-link>
        <router-link to="/reports" class="nav-link">
          <span class="nav-icon">📑</span> Reports
        </router-link>
        <router-link to="/settings" class="nav-link">
          <span class="nav-icon">⚙️</span> Settings
        </router-link>
      </nav>

      <div class="user-profile">
        <div class="user-info">
          <p class="user-name">{{ auth.currentUser?.fullName ?? 'User' }}</p>
          <span class="user-role">{{ auth.userRoles[0] ?? 'Staff' }}</span>
        </div>
        <button @click="handleLogout" class="btn-logout">
          🚪 Logout
        </button>
      </div>
    </aside>

    <!-- Main Content Panel -->
    <main class="main-content">
      <slot />
    </main>
  </div>
</template>

<script setup lang="ts">
import { useAuthStore } from '@/stores/auth.store'
import { useRouter } from 'vue-router'

const auth = useAuthStore()
const router = useRouter()

async function handleLogout() {
  await auth.logout()
  router.push('/login')
}
</script>

<style scoped>
.brand {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 32px;
  padding: 0 8px;
}

.brand-logo {
  width: 28px;
  height: 28px;
}

.brand-name {
  font-family: var(--font-heading);
  font-size: 1.25rem;
  font-weight: 700;
  background: linear-gradient(135deg, #ffffff 0%, #a5b4fc 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.nav-menu {
  display: flex;
  flex-direction: column;
  gap: 4px;
  flex-grow: 1;
}

.nav-icon {
  font-size: 1.1rem;
}

.user-profile {
  border-top: 1px solid var(--border-color);
  padding-top: 20px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.user-info {
  padding: 0 8px;
}

.user-name {
  font-weight: 600;
  font-size: 0.9rem;
  color: #ffffff;
}

.user-role {
  font-size: 0.75rem;
  color: var(--text-secondary);
}

.btn-logout {
  background: none;
  border: none;
  color: var(--text-secondary);
  font-family: var(--font-body);
  font-size: 0.85rem;
  font-weight: 500;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px;
  border-radius: var(--radius-sm);
  transition: all var(--transition-fast);
  text-align: left;
}

.btn-logout:hover {
  background-color: var(--danger-bg);
  color: var(--danger);
}
</style>
