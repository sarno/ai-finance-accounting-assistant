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
    </aside>

    <!-- Main Content Panel Wrapper -->
    <div class="main-wrapper">
      <!-- Header Bar -->
      <header class="header-bar">
        <!-- Breadcrumb -->
        <nav class="breadcrumb">
          <div v-for="(crumb, index) in breadcrumbs" :key="index" class="breadcrumb-item">
            <span v-if="index > 0" class="breadcrumb-separator">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="separator-icon">
                <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7" />
              </svg>
            </span>
            <router-link v-if="crumb.to" :to="crumb.to" class="breadcrumb-link">
              <span v-if="index === 0" class="home-icon">🏠</span>
              <span>{{ crumb.label }}</span>
            </router-link>
            <span v-else class="breadcrumb-current">
              <span v-if="index === 0" class="home-icon">🏠</span>
              <span>{{ crumb.label }}</span>
            </span>
          </div>
        </nav>

        <!-- User Profile Dropdown -->
        <div class="profile-container" ref="dropdownRef">
          <button class="profile-trigger" @click="toggleDropdown">
            <div class="avatar">
              {{ userInitials }}
            </div>
            <div class="profile-details">
              <span class="profile-name">{{ auth.currentUser?.fullName ?? 'User' }}</span>
              <span class="profile-role">{{ auth.userRoles[0] ?? 'Staff' }}</span>
            </div>
            <svg class="chevron-icon" :class="{ 'chevron-rotated': isDropdownOpen }" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7" />
            </svg>
          </button>

          <!-- Dropdown Menu -->
          <transition name="dropdown-fade">
            <div v-if="isDropdownOpen" class="profile-dropdown">
              <div class="dropdown-header">
                <p class="dropdown-name">{{ auth.currentUser?.fullName ?? 'User' }}</p>
                <p class="dropdown-email">{{ auth.currentUser?.email ?? 'user@codexfinance.com' }}</p>
                <span class="dropdown-badge">{{ auth.userRoles[0] ?? 'Staff' }}</span>
              </div>
              <div class="dropdown-divider"></div>
              <ul class="dropdown-menu">
                <li>
                  <router-link to="/settings" class="dropdown-item" @click="isDropdownOpen = false">
                    <span class="dropdown-icon">⚙️</span> Settings
                  </router-link>
                </li>
                <li>
                  <a href="#" class="dropdown-item" @click.prevent="isDropdownOpen = false">
                    <span class="dropdown-icon">👤</span> My Profile
                  </a>
                </li>
              </ul>
              <div class="dropdown-divider"></div>
              <div class="dropdown-footer">
                <button @click="handleLogout" class="dropdown-item logout-item">
                  <span class="dropdown-icon">🚪</span> Logout
                </button>
              </div>
            </div>
          </transition>
        </div>
      </header>

      <!-- Main Content Panel -->
      <main class="main-content">
        <slot />
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useAuthStore } from '@/stores/auth.store'
import { useRouter, useRoute } from 'vue-router'

const auth = useAuthStore()
const router = useRouter()
const route = useRoute()

// ─── Dropdown State ──────────────────────────────────────────────────────────
const isDropdownOpen = ref(false)
const dropdownRef = ref<HTMLElement | null>(null)

function toggleDropdown() {
  isDropdownOpen.value = !isDropdownOpen.value
}

function closeDropdown(event: MouseEvent) {
  if (dropdownRef.value && !dropdownRef.value.contains(event.target as Node)) {
    isDropdownOpen.value = false
  }
}

onMounted(() => {
  document.addEventListener('click', closeDropdown)
})

onUnmounted(() => {
  document.removeEventListener('click', closeDropdown)
})

// ─── User Initials ──────────────────────────────────────────────────────────
const userInitials = computed(() => {
  const name = auth.currentUser?.fullName || 'User'
  const parts = name.split(' ')
  if (parts.length >= 2) {
    return (parts[0][0] + parts[1][0]).toUpperCase()
  }
  return name.slice(0, 2).toUpperCase()
})

// ─── Breadcrumb Mapping ──────────────────────────────────────────────────────
const breadcrumbMap: Record<string, string> = {
  dashboard: 'Dashboard',
  journals: 'Journals',
  sales: 'Sales Invoices',
  purchases: 'Purchases',
  payments: 'Payments',
  approvals: 'Approvals',
  tax: 'Tax Module',
  reports: 'Reports',
  settings: 'Settings',
  new: 'New',
  edit: 'Edit',
  create: 'Create',
  details: 'Details'
}

interface BreadcrumbItem {
  label: string
  to: string | null
}

const breadcrumbs = computed<BreadcrumbItem[]>(() => {
  const path = route.path
  
  // Base item is always Dashboard
  const items: BreadcrumbItem[] = [{ label: 'Dashboard', to: '/dashboard' }]
  
  if (path === '/' || path === '/dashboard') {
    return [{ label: 'Dashboard', to: null }]
  }

  const segments = path.split('/').filter(Boolean)
  let currentPath = ''
  
  segments.forEach((segment, index) => {
    // If the first segment is dashboard, we already have it as base
    if (segment === 'dashboard') return
    
    currentPath += `/${segment}`
    const isLast = index === segments.length - 1
    
    let label = breadcrumbMap[segment] || segment.charAt(0).toUpperCase() + segment.slice(1)
    
    // If it looks like an ID, format it nicely
    if (/^\d+$/.test(segment)) {
      label = `#${segment}`
    } else if (segment.length > 20) {
      label = segment.substring(0, 8) + '...'
    }

    items.push({
      label,
      to: isLast ? null : currentPath
    })
  })

  return items
})

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
  background: linear-gradient(135deg, var(--accent-primary) 0%, var(--accent-secondary) 100%);
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

/* ─── Header Bar Styles ──────────────────────────────────────────────────────── */
.header-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 40px;
  height: 70px;
  background-color: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
  flex-shrink: 0;
}

/* ─── Breadcrumb Styles ─────────────────────────────────────────────────────── */
.breadcrumb {
  display: flex;
  align-items: center;
  gap: 6px;
}

.breadcrumb-item {
  display: flex;
  align-items: center;
}

.breadcrumb-separator {
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-muted);
  margin: 0 6px;
}

.separator-icon {
  width: 12px;
  height: 12px;
}

.breadcrumb-link {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  color: var(--text-secondary);
  text-decoration: none;
  font-size: 0.875rem;
  font-weight: 500;
  transition: color var(--transition-fast);
}

.breadcrumb-link:hover {
  color: var(--accent-primary);
}

.breadcrumb-current {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  color: var(--text-primary);
  font-size: 0.875rem;
  font-weight: 600;
}

.home-icon {
  font-size: 1rem;
  opacity: 0.8;
}

/* ─── User Profile Trigger Styles ───────────────────────────────────────────── */
.profile-container {
  position: relative;
}

.profile-trigger {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 6px 12px;
  background: none;
  border: 1px solid transparent;
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.profile-trigger:hover {
  background-color: var(--bg-tertiary);
  border-color: var(--border-color);
}

.avatar {
  width: 36px;
  height: 36px;
  border-radius: 50%;
  background: linear-gradient(135deg, var(--accent-primary) 0%, var(--accent-secondary) 100%);
  color: #ffffff;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 600;
  font-size: 0.9rem;
  font-family: var(--font-heading);
  box-shadow: 0 2px 8px rgba(79, 70, 229, 0.15);
}

.profile-details {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  text-align: left;
}

.profile-name {
  font-size: 0.875rem;
  font-weight: 600;
  color: var(--text-primary);
  line-height: 1.2;
}

.profile-role {
  font-size: 0.75rem;
  color: var(--text-secondary);
  line-height: 1.2;
  margin-top: 2px;
}

.chevron-icon {
  width: 16px;
  height: 16px;
  color: var(--text-secondary);
  transition: transform var(--transition-fast);
}

.chevron-rotated {
  transform: rotate(180deg);
}

/* ─── Profile Dropdown Menu Styles ─────────────────────────────────────────── */
.profile-dropdown {
  position: absolute;
  top: calc(100% + 8px);
  right: 0;
  width: 240px;
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-lg), 0 0 0 1px rgba(0, 0, 0, 0.05);
  z-index: 200;
  transform-origin: top right;
}

.dropdown-header {
  padding: 16px 20px;
}

.dropdown-name {
  font-weight: 600;
  font-size: 0.9rem;
  color: var(--text-primary);
}

.dropdown-email {
  font-size: 0.75rem;
  color: var(--text-secondary);
  margin-top: 2px;
  word-break: break-all;
}

.dropdown-badge {
  display: inline-block;
  margin-top: 8px;
  padding: 2px 8px;
  font-size: 0.7rem;
  font-weight: 600;
  border-radius: 9999px;
  background-color: var(--accent-primary-glow);
  color: var(--accent-primary);
}

.dropdown-divider {
  height: 1px;
  background-color: var(--border-color);
}

.dropdown-menu {
  list-style: none;
  padding: 6px;
}

.dropdown-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 14px;
  color: var(--text-secondary);
  text-decoration: none;
  font-size: 0.85rem;
  font-weight: 500;
  border-radius: var(--radius-sm);
  cursor: pointer;
  width: 100%;
  background: none;
  border: none;
  text-align: left;
  transition: all var(--transition-fast);
}

.dropdown-item:hover {
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
}

.dropdown-icon {
  font-size: 1rem;
}

.logout-item {
  color: var(--danger);
}

.logout-item:hover {
  background-color: var(--danger-bg);
  color: var(--danger);
}

/* ─── Transition Animations ────────────────────────────────────────────────── */
.dropdown-fade-enter-active,
.dropdown-fade-leave-active {
  transition: all var(--transition-fast);
}

.dropdown-fade-enter-from,
.dropdown-fade-leave-to {
  opacity: 0;
  transform: scale(0.95) translateY(-8px);
}
</style>
