<template>
  <div class="login-container">
    <!-- Left Panel: Login Form -->
    <div class="login-left">
      <div class="login-header">
        <svg class="brand-logo" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M12 2L2 7L12 12L22 7L12 2Z" stroke="#4f46e5" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          <path d="M2 17L12 22L22 17" stroke="#0ea5e9" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          <path d="M2 12L12 17L22 12" stroke="#4f46e5" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        <span class="brand-name">Codex Finance</span>
      </div>

      <div class="form-wrapper">
        <div class="welcome-text">
          <h1>Welcome back</h1>
          <p>Sign in to access your financial automation dashboard</p>
        </div>

        <form @submit.prevent="handleLogin" class="form">
          <div v-if="error" class="alert alert-danger">
            <span>⚠️ {{ error }}</span>
          </div>

          <div class="form-group">
            <label class="form-label" for="email">Work Email</label>
            <div class="input-wrapper">
              <span class="input-icon">✉️</span>
              <input 
                id="email"
                v-model="form.email" 
                type="email" 
                class="form-input with-icon" 
                placeholder="name@company.com" 
                required 
              />
            </div>
          </div>

          <div class="form-group">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
              <label class="form-label" for="password" style="margin-bottom: 0;">Password</label>
              <a href="#" class="forgot-link">Forgot password?</a>
            </div>
            <div class="input-wrapper">
              <span class="input-icon">🔒</span>
              <input 
                id="password"
                v-model="form.password" 
                type="password" 
                class="form-input with-icon" 
                placeholder="••••••••" 
                required 
              />
            </div>
          </div>

          <div class="form-options">
            <label class="checkbox-label">
              <input type="checkbox" checked /> Keep me signed in
            </label>
          </div>

          <button type="submit" class="btn btn-primary btn-block" :disabled="loading">
            <span v-if="loading" class="spinner"></span>
            {{ loading ? 'Authenticating...' : 'Sign In' }}
          </button>
        </form>
      </div>

      <div class="login-footer">
        <p>Security managed by Argon2id & JWT encryption.</p>
      </div>
    </div>

    <!-- Right Panel: Marketing Panel with Gradient -->
    <div class="login-right">
      <div class="marketing-content">
        <span class="pill">🚀 AI Automated Pipeline</span>
        <h2>Accelerating Financial Workflows</h2>
        <p>Digitize paper receipts, process multi-currency journal entries, and track tax liabilities automatically.</p>

        <!-- Floating UI Mockup Card -->
        <div class="ui-mockup-card">
          <div class="mockup-header">
            <div class="dots">
              <span></span>
              <span></span>
              <span></span>
            </div>
            <span class="mockup-title">Invoice Extraction Queue</span>
          </div>
          <div class="mockup-body">
            <div class="mockup-row">
              <div class="mockup-col">
                <span class="mockup-label">Supplier</span>
                <span class="mockup-value">Office Depot Inc.</span>
              </div>
              <div class="mockup-col" style="text-align: right;">
                <span class="mockup-label">Extracted Date</span>
                <span class="mockup-value">Today, 10:42 AM</span>
              </div>
            </div>
            <div class="mockup-row" style="margin-top: 12px; padding-top: 12px; border-top: 1px solid rgba(255, 255, 255, 0.1);">
              <div class="mockup-col">
                <span class="mockup-label">Confidence Score</span>
                <div style="display: flex; align-items: center; gap: 8px; margin-top: 4px;">
                  <span class="badge-dot"></span>
                  <span class="mockup-value" style="margin-top: 0; font-weight: 700;">98.5% Accuracy</span>
                </div>
              </div>
              <div class="mockup-col" style="text-align: right;">
                <span class="mockup-label">Amount</span>
                <span class="mockup-value" style="color: #06b6d4; font-weight: 700;">Rp 12,450,000</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth.store'

const router = useRouter()
const auth   = useAuthStore()

const form    = reactive({ email: '', password: '' })
const loading = ref(false)
const error   = ref<string | null>(null)

async function handleLogin() {
  loading.value = true
  error.value = null
  try {
    await auth.login(form)
    await router.push('/dashboard')
  } catch (e: any) {
    error.value = e.response?.data?.message ?? 'Incorrect email or password.'
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.login-container {
  display: flex;
  min-height: 100vh;
  background-color: #ffffff;
}

.login-left {
  width: 45%;
  padding: 40px 60px;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  background-color: #ffffff;
}

.login-right {
  width: 55%;
  background: linear-gradient(135deg, #1e1b4b 0%, #312e81 100%);
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 80px;
  overflow: hidden;
}

.login-right::before {
  content: '';
  position: absolute;
  top: -20%;
  right: -20%;
  width: 600px;
  height: 600px;
  border-radius: 50%;
  background: radial-gradient(circle, rgba(79, 70, 229, 0.15) 0%, transparent 70%);
  filter: blur(40px);
}

.login-header {
  display: flex;
  align-items: center;
  gap: 12px;
}

.brand-logo {
  width: 32px;
  height: 32px;
}

.brand-name {
  font-family: var(--font-heading);
  font-size: 1.35rem;
  font-weight: 700;
  color: var(--text-primary);
}

.form-wrapper {
  max-width: 420px;
  width: 100%;
  margin: auto 0;
}

.welcome-text h1 {
  font-family: var(--font-heading);
  font-size: 2.25rem;
  font-weight: 700;
  color: var(--text-primary);
  margin-bottom: 8px;
}

.welcome-text p {
  color: var(--text-secondary);
  font-size: 0.95rem;
  margin-bottom: 32px;
}

.alert-danger {
  background-color: var(--danger-bg);
  color: var(--danger);
  border: 1px solid rgba(239, 68, 68, 0.2);
  padding: 12px 16px;
  border-radius: var(--radius-sm);
  font-size: 0.875rem;
  font-weight: 500;
  margin-bottom: 24px;
}

.input-wrapper {
  position: relative;
  display: flex;
  align-items: center;
}

.input-icon {
  position: absolute;
  left: 14px;
  font-size: 1rem;
  color: var(--text-muted);
}

.form-input.with-icon {
  padding-left: 44px;
}

.forgot-link {
  color: var(--accent-primary);
  font-size: 0.85rem;
  text-decoration: none;
  font-weight: 600;
}

.forgot-link:hover {
  text-decoration: underline;
}

.form-options {
  margin-top: -8px;
  margin-bottom: 24px;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 0.85rem;
  color: var(--text-secondary);
  cursor: pointer;
}

.btn-block {
  width: 100%;
  padding: 12px;
  font-size: 0.95rem;
}

.login-footer {
  color: var(--text-muted);
  font-size: 0.75rem;
}

/* Marketing Sidebar Panel Styling */
.marketing-content {
  max-width: 520px;
  color: #ffffff;
  z-index: 1;
}

.marketing-content h2 {
  font-family: var(--font-heading);
  font-size: 2.75rem;
  font-weight: 700;
  line-height: 1.2;
  margin: 16px 0;
  background: linear-gradient(135deg, #ffffff 0%, #e0e7ff 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.marketing-content p {
  color: #c7d2fe;
  font-size: 1.05rem;
  line-height: 1.6;
  margin-bottom: 40px;
}

.pill {
  display: inline-block;
  background-color: rgba(99, 102, 241, 0.2);
  border: 1px solid rgba(165, 180, 252, 0.2);
  color: #e0e7ff;
  font-size: 0.75rem;
  font-weight: 600;
  padding: 6px 12px;
  border-radius: 9999px;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

/* UI Mockup card inside marketing panel */
.ui-mockup-card {
  background-color: rgba(30, 27, 75, 0.4);
  backdrop-filter: blur(16px);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: var(--radius-md);
  box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
  overflow: hidden;
}

.mockup-header {
  padding: 14px 20px;
  background-color: rgba(255, 255, 255, 0.03);
  display: flex;
  align-items: center;
  justify-content: space-between;
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
}

.dots {
  display: flex;
  gap: 6px;
}

.dots span {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background-color: rgba(255, 255, 255, 0.2);
}

.mockup-title {
  color: #a5b4fc;
  font-size: 0.75rem;
  font-weight: 600;
}

.mockup-body {
  padding: 20px;
}

.mockup-row {
  display: flex;
  justify-content: space-between;
}

.mockup-col {
  display: flex;
  flex-direction: column;
}

.mockup-label {
  font-size: 0.7rem;
  text-transform: uppercase;
  color: #818cf8;
  font-weight: 600;
  letter-spacing: 0.05em;
}

.mockup-value {
  color: #ffffff;
  font-size: 0.9rem;
  margin-top: 4px;
}

.badge-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background-color: #06b6d4;
  box-shadow: 0 0 8px #06b6d4;
}

@media (max-width: 1024px) {
  .login-right {
    display: none;
  }
  .login-left {
    width: 100%;
    padding: 40px;
  }
}
</style>
