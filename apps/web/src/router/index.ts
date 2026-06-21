import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'
import { useAuthStore } from '@/stores/auth.store'

// ─── Lazy-loaded page components ─────────────────────────────────────────────

const LoginPage           = () => import('@/pages/auth/LoginPage.vue')
const DashboardPage       = () => import('@/pages/dashboard/DashboardPage.vue')
const JournalsPage        = () => import('@/pages/journals/JournalsPage.vue')
const SalesInvoicesPage   = () => import('@/pages/sales/SalesInvoicesPage.vue')
const PurchaseInvoicesPage = () => import('@/pages/purchases/PurchaseInvoicesPage.vue')
const PaymentsPage        = () => import('@/pages/payments/PaymentsPage.vue')
const ApprovalsPage       = () => import('@/pages/approvals/ApprovalsPage.vue')
const TaxPage             = () => import('@/pages/tax/TaxPage.vue')
const ReportsPage         = () => import('@/pages/reports/ReportsPage.vue')
const SettingsPage        = () => import('@/pages/settings/SettingsPage.vue')
const NotFoundPage        = () => import('@/pages/NotFoundPage.vue')

// ─── Route definitions ────────────────────────────────────────────────────────

const routes: RouteRecordRaw[] = [
  {
    path: '/login',
    name: 'login',
    component: LoginPage,
    meta: { public: true },
  },
  {
    path: '/',
    redirect: '/dashboard',
  },
  {
    path: '/dashboard',
    name: 'dashboard',
    component: DashboardPage,
    meta: { requiresAuth: true },
  },
  {
    path: '/journals',
    name: 'journals',
    component: JournalsPage,
    meta: { requiresAuth: true },
  },
  {
    path: '/sales',
    name: 'sales',
    component: SalesInvoicesPage,
    meta: { requiresAuth: true },
  },
  {
    path: '/purchases',
    name: 'purchases',
    component: PurchaseInvoicesPage,
    meta: { requiresAuth: true },
  },
  {
    path: '/payments',
    name: 'payments',
    component: PaymentsPage,
    meta: { requiresAuth: true },
  },
  {
    path: '/approvals',
    name: 'approvals',
    component: ApprovalsPage,
    meta: { requiresAuth: true },
  },
  {
    path: '/tax',
    name: 'tax',
    component: TaxPage,
    meta: { requiresAuth: true },
  },
  {
    path: '/reports',
    name: 'reports',
    component: ReportsPage,
    meta: { requiresAuth: true },
  },
  {
    path: '/settings',
    name: 'settings',
    component: SettingsPage,
    meta: { requiresAuth: true },
  },
  {
    path: '/:pathMatch(.*)*',
    name: 'not-found',
    component: NotFoundPage,
  },
]

// ─── Router instance ─────────────────────────────────────────────────────────

const router = createRouter({
  history: createWebHistory(),
  routes,
})

// ─── Navigation guard ─────────────────────────────────────────────────────────

router.beforeEach((to, _from, next) => {
  const auth = useAuthStore()

  if (to.meta.requiresAuth && !auth.isAuthenticated) {
    next({ name: 'login', query: { redirect: to.fullPath } })
  } else if (to.name === 'login' && auth.isAuthenticated) {
    next({ name: 'dashboard' })
  } else {
    next()
  }
})

export default router
