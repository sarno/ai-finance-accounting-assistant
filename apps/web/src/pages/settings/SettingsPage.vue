<template>
  <MainLayout>
    <div class="page-header">
      <div>
        <h1>Master Data Management</h1>
        <p class="page-title-desc">Configure and administer core entities for your accounting workspace.</p>
      </div>
    </div>

    <!-- Tab switcher -->
    <div class="tabs">
      <button 
        v-for="tab in tabItems" 
        :key="tab.id"
        :class="['tab-btn', { active: activeTab === tab.id }]"
        @click="activeTab = tab.id"
      >
        {{ tab.label }}
      </button>
    </div>

    <div v-if="errorMsg" class="alert alert-danger" style="margin-bottom: 20px; padding: 12px; border-radius: var(--radius-sm); background-color: var(--danger-bg); color: var(--danger);">
      ⚠️ {{ errorMsg }}
    </div>
    <div v-if="successMsg" class="alert alert-success" style="margin-bottom: 20px; padding: 12px; border-radius: var(--radius-sm); background-color: var(--success-bg); color: var(--success);">
      ✅ {{ successMsg }}
    </div>

    <!-- ─── Tab Content: Company ─── -->
    <div v-if="activeTab === 'company'" class="card">
      <h2 style="margin-bottom: 16px;">Workspace Details</h2>
      <form @submit.prevent="saveCompany" class="grid-form">
        <div class="form-group">
          <label class="form-label">Company Name</label>
          <input v-model="companyForm.name" type="text" class="form-input" required />
        </div>
        <div class="form-group">
          <label class="form-label">Tax Identification Number (NPWP)</label>
          <input v-model="companyForm.taxNumber" type="text" class="form-input" />
        </div>
        <div class="form-group">
          <label class="form-label">Base Currency</label>
          <select v-model="companyForm.currency" class="form-select">
            <option value="IDR">IDR - Indonesian Rupiah</option>
            <option value="USD">USD - US Dollar</option>
            <option value="EUR">EUR - Euro</option>
            <option value="SGD">SGD - Singapore Dollar</option>
          </select>
        </div>
        <div class="form-group">
          <label class="form-label">Address</label>
          <textarea v-model="companyForm.address" class="form-textarea" rows="3"></textarea>
        </div>
        <div style="margin-top: 16px;">
          <button type="submit" class="btn btn-primary" :disabled="loading">
            {{ loading ? 'Saving...' : 'Save Workspace' }}
          </button>
        </div>
      </form>
    </div>

    <!-- ─── Tab Content: COA ─── -->
    <div v-if="activeTab === 'coa'">
      <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;">
        <h3>Chart of Accounts</h3>
        <button @click="openAddAccountModal" class="btn btn-primary">+ Add Account</button>
      </div>

      <div class="table-container">
        <table>
          <thead>
            <tr>
              <th>Code</th>
              <th>Account Name</th>
              <th>Type</th>
              <th>Status</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="acc in accounts" :key="acc.id">
              <td style="font-weight: 600; color: var(--accent-primary);">{{ acc.code }}</td>
              <td>{{ acc.name }}</td>
              <td><span class="badge badge-info">{{ acc.accountType }}</span></td>
              <td>
                <span :class="['badge', acc.isActive ? 'badge-success' : 'badge-danger']">
                  {{ acc.isActive ? 'Active' : 'Inactive' }}
                </span>
              </td>
              <td>
                <button @click="editAccount(acc)" class="btn btn-secondary" style="padding: 4px 8px; font-size: 0.75rem;">Edit</button>
              </td>
            </tr>
            <tr v-if="accounts.length === 0">
              <td colspan="5" style="text-align: center; color: var(--text-secondary);">No accounts found.</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- ─── Tab Content: Customers ─── -->
    <div v-if="activeTab === 'customers'">
      <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;">
        <h3>Customers Directory</h3>
        <button @click="openAddCustomerModal" class="btn btn-primary">+ Add Customer</button>
      </div>

      <div class="table-container">
        <table>
          <thead>
            <tr>
              <th>Name</th>
              <th>NPWP</th>
              <th>Email</th>
              <th>Phone</th>
              <th>Status</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="cust in customers" :key="cust.id">
              <td style="font-weight: 600;">{{ cust.name }}</td>
              <td>{{ cust.taxNumber || '-' }}</td>
              <td>{{ cust.email || '-' }}</td>
              <td>{{ cust.phone || '-' }}</td>
              <td>
                <span :class="['badge', cust.isActive ? 'badge-success' : 'badge-danger']">
                  {{ cust.isActive ? 'Active' : 'Inactive' }}
                </span>
              </td>
              <td>
                <button @click="editCustomer(cust)" class="btn btn-secondary" style="padding: 4px 8px; font-size: 0.75rem;">Edit</button>
              </td>
            </tr>
            <tr v-if="customers.length === 0">
              <td colspan="6" style="text-align: center; color: var(--text-secondary);">No customers found.</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- ─── Tab Content: Suppliers ─── -->
    <div v-if="activeTab === 'suppliers'">
      <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;">
        <h3>Suppliers Directory</h3>
        <button @click="openAddSupplierModal" class="btn btn-primary">+ Add Supplier</button>
      </div>

      <div class="table-container">
        <table>
          <thead>
            <tr>
              <th>Name</th>
              <th>NPWP</th>
              <th>Email</th>
              <th>Phone</th>
              <th>Status</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="sup in suppliers" :key="sup.id">
              <td style="font-weight: 600;">{{ sup.name }}</td>
              <td>{{ sup.taxNumber || '-' }}</td>
              <td>{{ sup.email || '-' }}</td>
              <td>{{ sup.phone || '-' }}</td>
              <td>
                <span :class="['badge', sup.isActive ? 'badge-success' : 'badge-danger']">
                  {{ sup.isActive ? 'Active' : 'Inactive' }}
                </span>
              </td>
              <td>
                <button @click="editSupplier(sup)" class="btn btn-secondary" style="padding: 4px 8px; font-size: 0.75rem;">Edit</button>
              </td>
            </tr>
            <tr v-if="suppliers.length === 0">
              <td colspan="6" style="text-align: center; color: var(--text-secondary);">No suppliers found.</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- ─── Tab Content: Bank Accounts ─── -->
    <div v-if="activeTab === 'banks'">
      <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;">
        <h3>Bank Accounts</h3>
        <button @click="openAddBankModal" class="btn btn-primary">+ Add Bank Account</button>
      </div>

      <div class="table-container">
        <table>
          <thead>
            <tr>
              <th>Bank Name</th>
              <th>Account Number</th>
              <th>Account Holder</th>
              <th>Currency</th>
              <th>Status</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="bank in bankAccounts" :key="bank.id">
              <td style="font-weight: 600;">{{ bank.bankName }}</td>
              <td>{{ bank.accountNumber }}</td>
              <td>{{ bank.accountName }}</td>
              <td>{{ bank.currency }}</td>
              <td>
                <span :class="['badge', bank.isActive ? 'badge-success' : 'badge-danger']">
                  {{ bank.isActive ? 'Active' : 'Inactive' }}
                </span>
              </td>
              <td>
                <button @click="editBank(bank)" class="btn btn-secondary" style="padding: 4px 8px; font-size: 0.75rem;">Edit</button>
              </td>
            </tr>
            <tr v-if="bankAccounts.length === 0">
              <td colspan="6" style="text-align: center; color: var(--text-secondary);">No bank accounts found.</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- ─── Tab Content: Tax Types ─── -->
    <div v-if="activeTab === 'taxes'">
      <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;">
        <h3>Tax Rates Config</h3>
        <button @click="openAddTaxModal" class="btn btn-primary">+ Add Tax Type</button>
      </div>

      <div class="table-container">
        <table>
          <thead>
            <tr>
              <th>Code</th>
              <th>Name</th>
              <th>Category</th>
              <th>Default Rate</th>
              <th>Effective Dates</th>
              <th>Status</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="tax in taxTypes" :key="tax.id">
              <td style="font-weight: 600; color: var(--accent-primary);">{{ tax.code }}</td>
              <td>{{ tax.name }}</td>
              <td><span class="badge badge-info">{{ tax.category }}</span></td>
              <td>{{ (tax.defaultRate * 100).toFixed(2) }}%</td>
              <td>{{ tax.effectiveFrom }} to {{ tax.effectiveTo || 'Present' }}</td>
              <td>
                <span :class="['badge', tax.isActive ? 'badge-success' : 'badge-danger']">
                  {{ tax.isActive ? 'Active' : 'Inactive' }}
                </span>
              </td>
              <td>
                <button @click="editTax(tax)" class="btn btn-secondary" style="padding: 4px 8px; font-size: 0.75rem;">Edit</button>
              </td>
            </tr>
            <tr v-if="taxTypes.length === 0">
              <td colspan="7" style="text-align: center; color: var(--text-secondary);">No tax types found.</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- ─── Tab Content: Branches ─── -->
    <div v-if="activeTab === 'branches'">
      <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;">
        <h3>Company Branches</h3>
        <button @click="openAddBranchModal" class="btn btn-primary">+ Add Branch</button>
      </div>

      <div class="table-container">
        <table>
          <thead>
            <tr>
              <th>Code</th>
              <th>Name</th>
              <th>Address</th>
              <th>Phone</th>
              <th>Status</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="branch in branches" :key="branch.id">
              <td style="font-weight: 600; color: var(--accent-primary);">{{ branch.code }}</td>
              <td>{{ branch.name }}</td>
              <td>{{ branch.address || '-' }}</td>
              <td>{{ branch.phone || '-' }}</td>
              <td>
                <span :class="['badge', branch.isActive ? 'badge-success' : 'badge-danger']">
                  {{ branch.isActive ? 'Active' : 'Inactive' }}
                </span>
              </td>
              <td>
                <button @click="editBranch(branch)" class="btn btn-secondary" style="padding: 4px 8px; font-size: 0.75rem;">Edit</button>
              </td>
            </tr>
            <tr v-if="branches.length === 0">
              <td colspan="6" style="text-align: center; color: var(--text-secondary);">No branches found.</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- ─── Modals ─── -->
    
    <!-- Account Modal -->
    <div v-if="modals.account" class="modal-overlay">
      <div class="modal-content">
        <div class="modal-header">
          <h2>{{ editingId ? 'Edit Account' : 'New Chart of Account' }}</h2>
          <button @click="modals.account = false" class="modal-close">&times;</button>
        </div>
        <form @submit.prevent="submitAccount">
          <div class="modal-body">
            <div class="form-group">
              <label class="form-label">Account Code</label>
              <input v-model="accountForm.code" type="text" class="form-input" required placeholder="e.g. 1001" />
            </div>
            <div class="form-group">
              <label class="form-label">Account Name</label>
              <input v-model="accountForm.name" type="text" class="form-input" required placeholder="e.g. Petty Cash" />
            </div>
            <div class="form-group">
              <label class="form-label">Account Type</label>
              <select v-model="accountForm.accountType" class="form-select" required>
                <option value="Asset">Asset</option>
                <option value="Liability">Liability</option>
                <option value="Equity">Equity</option>
                <option value="Revenue">Revenue</option>
                <option value="Expense">Expense</option>
              </select>
            </div>
            <div class="form-group">
              <label class="form-label">Parent Account (Optional)</label>
              <select v-model="accountForm.parentId" class="form-select">
                <option :value="undefined">No parent</option>
                <option v-for="acc in accounts.filter(a => a.id !== editingId)" :key="acc.id" :value="acc.id">
                  {{ acc.code }} - {{ acc.name }}
                </option>
              </select>
            </div>
            <div v-if="editingId" class="form-group">
              <label class="form-label">
                <input v-model="accountForm.isActive" type="checkbox" /> Active Status
              </label>
            </div>
          </div>
          <div class="modal-footer">
            <button type="button" @click="modals.account = false" class="btn btn-secondary">Cancel</button>
            <button type="submit" class="btn btn-primary">Save Account</button>
          </div>
        </form>
      </div>
    </div>

    <!-- Customer Modal -->
    <div v-if="modals.customer" class="modal-overlay">
      <div class="modal-content">
        <div class="modal-header">
          <h2>{{ editingId ? 'Edit Customer' : 'New Customer' }}</h2>
          <button @click="modals.customer = false" class="modal-close">&times;</button>
        </div>
        <form @submit.prevent="submitCustomer">
          <div class="modal-body">
            <div class="form-group">
              <label class="form-label">Customer Name</label>
              <input v-model="customerForm.name" type="text" class="form-input" required />
            </div>
            <div class="form-group">
              <label class="form-label">Tax ID (NPWP)</label>
              <input v-model="customerForm.taxNumber" type="text" class="form-input" />
            </div>
            <div class="form-group">
              <label class="form-label">Email</label>
              <input v-model="customerForm.email" type="email" class="form-input" />
            </div>
            <div class="form-group">
              <label class="form-label">Phone</label>
              <input v-model="customerForm.phone" type="text" class="form-input" />
            </div>
            <div class="form-group">
              <label class="form-label">Address</label>
              <textarea v-model="customerForm.address" class="form-textarea" rows="3"></textarea>
            </div>
            <div v-if="editingId" class="form-group">
              <label class="form-label">
                <input v-model="customerForm.isActive" type="checkbox" /> Active Status
              </label>
            </div>
          </div>
          <div class="modal-footer">
            <button type="button" @click="modals.customer = false" class="btn btn-secondary">Cancel</button>
            <button type="submit" class="btn btn-primary">Save Customer</button>
          </div>
        </form>
      </div>
    </div>

    <!-- Supplier Modal -->
    <div v-if="modals.supplier" class="modal-overlay">
      <div class="modal-content">
        <div class="modal-header">
          <h2>{{ editingId ? 'Edit Supplier' : 'New Supplier' }}</h2>
          <button @click="modals.supplier = false" class="modal-close">&times;</button>
        </div>
        <form @submit.prevent="submitSupplier">
          <div class="modal-body">
            <div class="form-group">
              <label class="form-label">Supplier Name</label>
              <input v-model="supplierForm.name" type="text" class="form-input" required />
            </div>
            <div class="form-group">
              <label class="form-label">Tax ID (NPWP)</label>
              <input v-model="supplierForm.taxNumber" type="text" class="form-input" />
            </div>
            <div class="form-group">
              <label class="form-label">Email</label>
              <input v-model="supplierForm.email" type="email" class="form-input" />
            </div>
            <div class="form-group">
              <label class="form-label">Phone</label>
              <input v-model="supplierForm.phone" type="text" class="form-input" />
            </div>
            <div class="form-group">
              <label class="form-label">Address</label>
              <textarea v-model="supplierForm.address" class="form-textarea" rows="3"></textarea>
            </div>
            <div v-if="editingId" class="form-group">
              <label class="form-label">
                <input v-model="supplierForm.isActive" type="checkbox" /> Active Status
              </label>
            </div>
          </div>
          <div class="modal-footer">
            <button type="button" @click="modals.supplier = false" class="btn btn-secondary">Cancel</button>
            <button type="submit" class="btn btn-primary">Save Supplier</button>
          </div>
        </form>
      </div>
    </div>

    <!-- Bank Modal -->
    <div v-if="modals.bank" class="modal-overlay">
      <div class="modal-content">
        <div class="modal-header">
          <h2>{{ editingId ? 'Edit Bank Account' : 'New Bank Account' }}</h2>
          <button @click="modals.bank = false" class="modal-close">&times;</button>
        </div>
        <form @submit.prevent="submitBank">
          <div class="modal-body">
            <div class="form-group">
              <label class="form-label">Bank Name</label>
              <input v-model="bankForm.bankName" type="text" class="form-input" required placeholder="e.g. BCA, Mandiri" />
            </div>
            <div class="form-group">
              <label class="form-label">Account Number</label>
              <input v-model="bankForm.accountNumber" type="text" class="form-input" required />
            </div>
            <div class="form-group">
              <label class="form-label">Account Holder Name</label>
              <input v-model="bankForm.accountName" type="text" class="form-input" required />
            </div>
            <div class="form-group">
              <label class="form-label">Currency</label>
              <input v-model="bankForm.currency" type="text" class="form-input" required />
            </div>
            <div class="form-group">
              <label class="form-label">Linked GL Account</label>
              <select v-model="bankForm.accountId" class="form-select" required>
                <option v-for="acc in accounts.filter(a => a.accountType === 'Asset')" :key="acc.id" :value="acc.id">
                  {{ acc.code }} - {{ acc.name }}
                </option>
              </select>
            </div>
            <div v-if="editingId" class="form-group">
              <label class="form-label">
                <input v-model="bankForm.isActive" type="checkbox" /> Active Status
              </label>
            </div>
          </div>
          <div class="modal-footer">
            <button type="button" @click="modals.bank = false" class="btn btn-secondary">Cancel</button>
            <button type="submit" class="btn btn-primary">Save Bank Account</button>
          </div>
        </form>
      </div>
    </div>

    <!-- Tax Modal -->
    <div v-if="modals.tax" class="modal-overlay">
      <div class="modal-content">
        <div class="modal-header">
          <h2>{{ editingId ? 'Edit Tax Config' : 'New Tax Rate' }}</h2>
          <button @click="modals.tax = false" class="modal-close">&times;</button>
        </div>
        <form @submit.prevent="submitTax">
          <div class="modal-body">
            <div class="form-group">
              <label class="form-label">Tax Code</label>
              <input v-model="taxForm.code" type="text" class="form-input" required placeholder="e.g. PPN11" :disabled="!!editingId" />
            </div>
            <div class="form-group">
              <label class="form-label">Tax Name</label>
              <input v-model="taxForm.name" type="text" class="form-input" required placeholder="e.g. VAT 11%" />
            </div>
            <div class="form-group">
              <label class="form-label">Category</label>
              <select v-model="taxForm.category" class="form-select" required>
                <option value="vat_output">VAT Output</option>
                <option value="vat_input">VAT Input</option>
                <option value="withholding_pph21">Withholding PPh 21</option>
                <option value="withholding_pph23">Withholding PPh 23</option>
                <option value="withholding_pph25">Withholding PPh 25</option>
                <option value="withholding_pph_final">Withholding PPh Final</option>
              </select>
            </div>
            <div class="form-group">
              <label class="form-label">Default Rate (Decimal, e.g. 0.1100 for 11%)</label>
              <input v-model="taxForm.defaultRate" type="number" step="0.0001" min="0" max="1" class="form-input" required />
            </div>
            <div class="form-group">
              <label class="form-label">Payable/Receivable account</label>
              <select v-model="taxForm.payableAccountId" class="form-select" required>
                <option v-for="acc in accounts" :key="acc.id" :value="acc.id">
                  {{ acc.code }} - {{ acc.name }}
                </option>
              </select>
            </div>
            <div class="form-group">
              <label class="form-label">Effective From</label>
              <input v-model="taxForm.effectiveFrom" type="date" class="form-input" required />
            </div>
            <div class="form-group">
              <label class="form-label">Effective To (Optional)</label>
              <input v-model="taxForm.effectiveTo" type="date" class="form-input" />
            </div>
            <div v-if="editingId" class="form-group">
              <label class="form-label">
                <input v-model="taxForm.isActive" type="checkbox" /> Active Status
              </label>
            </div>
          </div>
          <div class="modal-footer">
            <button type="button" @click="modals.tax = false" class="btn btn-secondary">Cancel</button>
            <button type="submit" class="btn btn-primary">Save Tax Config</button>
          </div>
        </form>
      </div>
    </div>
    <!-- Branch Modal -->
    <div v-if="modals.branch" class="modal-overlay">
      <div class="modal-content">
        <div class="modal-header">
          <h2>{{ editingId ? 'Edit Branch' : 'Add New Branch' }}</h2>
          <button @click="modals.branch = false" class="modal-close">&times;</button>
        </div>
        <form @submit.prevent="submitBranch">
          <div class="modal-body">
            <div class="form-group">
              <label class="form-label">Branch Code</label>
              <input v-model="branchForm.code" type="text" class="form-input" placeholder="e.g. HO, BDG, SBY" :disabled="!!editingId" required />
            </div>
            <div class="form-group">
              <label class="form-label">Branch Name</label>
              <input v-model="branchForm.name" type="text" class="form-input" placeholder="e.g. Head Office, Bandung Branch" required />
            </div>
            <div class="form-group">
              <label class="form-label">Address</label>
              <textarea v-model="branchForm.address" class="form-input" placeholder="Branch Address" rows="3"></textarea>
            </div>
            <div class="form-group">
              <label class="form-label">Phone Number</label>
              <input v-model="branchForm.phone" type="text" class="form-input" placeholder="e.g. +62-21-555-0199" />
            </div>
            <div v-if="editingId" class="form-group">
              <label class="form-label">
                <input v-model="branchForm.isActive" type="checkbox" /> Active Status
              </label>
            </div>
          </div>
          <div class="modal-footer">
            <button type="button" @click="modals.branch = false" class="btn btn-secondary">Cancel</button>
            <button type="submit" class="btn btn-primary">Save Branch</button>
          </div>
        </form>
      </div>
    </div>
  </MainLayout>
</template>

<script setup lang="ts">
import { ref, reactive, watch, onMounted } from 'vue'
import MainLayout from '@/components/MainLayout.vue'
import { useAuthStore } from '@/stores/auth.store'
import {
  companyApi,
  accountApi,
  customerApi,
  supplierApi,
  bankAccountApi,
  taxTypeApi,
  branchApi
} from '@/api/master-data.api'
import type {
  Company, Account, Customer, Supplier, BankAccount, TaxType, Branch
} from '@/types/master-data.types'

const auth = useAuthStore()

// State
const activeTab = ref('company')
const loading = ref(false)
const errorMsg = ref<string | null>(null)
const successMsg = ref<string | null>(null)
const editingId = ref<string | null>(null)

const tabItems = [
  { id: 'company', label: '🏢 Company Profile' },
  { id: 'branches', label: '🌿 Branches' },
  { id: 'coa', label: '📓 Chart of Accounts' },
  { id: 'customers', label: '👥 Customers' },
  { id: 'suppliers', label: '🤝 Suppliers' },
  { id: 'banks', label: '🏦 Bank Accounts' },
  { id: 'taxes', label: '⚖️ Tax configurations' }
]

// Data lists
const companies = ref<Company[]>([])
const branches = ref<Branch[]>([])
const accounts = ref<Account[]>([])
const customers = ref<Customer[]>([])
const suppliers = ref<Supplier[]>([])
const bankAccounts = ref<BankAccount[]>([])
const taxTypes = ref<TaxType[]>([])

// Active company ID
const activeCompanyId = ref<string>('')

// Form objects
const companyForm = reactive({
  name: '',
  taxNumber: '',
  address: '',
  currency: 'IDR'
})

const accountForm = reactive({
  code: '',
  name: '',
  accountType: 'Asset',
  parentId: undefined as string | undefined,
  isActive: true
})

const customerForm = reactive({
  name: '',
  taxNumber: '',
  email: '',
  phone: '',
  address: '',
  isActive: true
})

const supplierForm = reactive({
  name: '',
  taxNumber: '',
  email: '',
  phone: '',
  address: '',
  isActive: true
})

const bankForm = reactive({
  bankName: '',
  accountNumber: '',
  accountName: '',
  currency: 'IDR',
  accountId: '',
  isActive: true
})

const taxForm = reactive({
  code: '',
  name: '',
  category: 'vat_output',
  defaultRate: 0.1100,
  payableAccountId: '',
  effectiveFrom: '',
  effectiveTo: undefined as string | undefined,
  isActive: true
})

const branchForm = reactive({
  code: '',
  name: '',
  address: '',
  phone: '',
  isActive: true
})

// Modals Visibility State
const modals = reactive({
  account: false,
  customer: false,
  supplier: false,
  bank: false,
  tax: false,
  branch: false
})

// Lifecycle
onMounted(async () => {
  try {
    loading.value = true
    // Fetch all companies
    const compList = await companyApi.list()
    companies.value = compList
    
    // Default to the first company or the user's company
    const userCompanyId = auth.user?.companyId
    const found = compList.find(c => c.id === userCompanyId) || compList[0]
    
    if (found) {
      activeCompanyId.value = found.id
      Object.assign(companyForm, {
        name: found.name,
        taxNumber: found.taxNumber || '',
        address: found.address || '',
        currency: found.currency
      })
    }
    
    await loadTabSpecificData()
  } catch (err: any) {
    errorMsg.value = 'Failed to bootstrap master data layout'
  } finally {
    loading.value = false
  }
})

// Watch for tab change to fetch data
watch(activeTab, async () => {
  errorMsg.value = null
  successMsg.value = null
  await loadTabSpecificData()
})

async function loadTabSpecificData() {
  if (!activeCompanyId.value) return
  try {
    if (activeTab.value === 'branches') {
      branches.value = await branchApi.listByCompany(activeCompanyId.value)
    } else if (activeTab.value === 'coa') {
      accounts.value = await accountApi.listByCompany(activeCompanyId.value)
    } else if (activeTab.value === 'customers') {
      customers.value = await customerApi.listByCompany(activeCompanyId.value)
    } else if (activeTab.value === 'suppliers') {
      suppliers.value = await supplierApi.listByCompany(activeCompanyId.value)
    } else if (activeTab.value === 'banks') {
      // Must load accounts to bind linked asset account
      accounts.value = await accountApi.listByCompany(activeCompanyId.value)
      bankAccounts.value = await bankAccountApi.listByCompany(activeCompanyId.value)
    } else if (activeTab.value === 'taxes') {
      accounts.value = await accountApi.listByCompany(activeCompanyId.value)
      taxTypes.value = await taxTypeApi.listByCompany(activeCompanyId.value)
    }
  } catch (err: any) {
    errorMsg.value = `Failed to load data for tab: ${activeTab.value}`
  }
}

// ─── Save Company ───
async function saveCompany() {
  if (!activeCompanyId.value) return
  try {
    loading.value = true
    errorMsg.value = null
    successMsg.value = null
    
    await companyApi.update(activeCompanyId.value, {
      name: companyForm.name,
      taxNumber: companyForm.taxNumber || undefined,
      address: companyForm.address || undefined,
      currency: companyForm.currency,
      isActive: true
    })
    
    successMsg.value = 'Company profile updated successfully'
  } catch (err: any) {
    errorMsg.value = err.response?.data?.message || 'Failed to update company profile'
  } finally {
    loading.value = false
  }
}

// ─── Accounts Actions ───
function openAddAccountModal() {
  editingId.value = null
  Object.assign(accountForm, {
    code: '',
    name: '',
    accountType: 'Asset',
    parentId: undefined,
    isActive: true
  })
  modals.account = true
}

function editAccount(acc: Account) {
  editingId.value = acc.id
  Object.assign(accountForm, {
    code: acc.code,
    name: acc.name,
    accountType: acc.accountType,
    parentId: acc.parentId,
    isActive: acc.isActive
  })
  modals.account = true
}

async function submitAccount() {
  try {
    errorMsg.value = null
    successMsg.value = null
    if (editingId.value) {
      await accountApi.update(editingId.value, {
        code: accountForm.code,
        name: accountForm.name,
        accountType: accountForm.accountType,
        parentId: accountForm.parentId,
        isActive: accountForm.isActive
      })
      successMsg.value = 'Account updated successfully'
    } else {
      await accountApi.create({
        companyId: activeCompanyId.value,
        code: accountForm.code,
        name: accountForm.name,
        accountType: accountForm.accountType,
        parentId: accountForm.parentId
      })
      successMsg.value = 'Account created successfully'
    }
    modals.account = false
    await loadTabSpecificData()
  } catch (err: any) {
    errorMsg.value = err.response?.data?.message || 'Failed to save account'
  }
}

// ─── Customer Actions ───
function openAddCustomerModal() {
  editingId.value = null
  Object.assign(customerForm, {
    name: '',
    taxNumber: '',
    email: '',
    phone: '',
    address: '',
    isActive: true
  })
  modals.customer = true
}

function editCustomer(cust: Customer) {
  editingId.value = cust.id
  Object.assign(customerForm, {
    name: cust.name,
    taxNumber: cust.taxNumber || '',
    email: cust.email || '',
    phone: cust.phone || '',
    address: cust.address || '',
    isActive: cust.isActive
  })
  modals.customer = true
}

async function submitCustomer() {
  try {
    errorMsg.value = null
    successMsg.value = null
    if (editingId.value) {
      await customerApi.update(editingId.value, {
        name: customerForm.name,
        taxNumber: customerForm.taxNumber || undefined,
        email: customerForm.email || undefined,
        phone: customerForm.phone || undefined,
        address: customerForm.address || undefined,
        isActive: customerForm.isActive
      })
      successMsg.value = 'Customer profile updated successfully'
    } else {
      await customerApi.create({
        companyId: activeCompanyId.value,
        name: customerForm.name,
        taxNumber: customerForm.taxNumber || undefined,
        email: customerForm.email || undefined,
        phone: customerForm.phone || undefined,
        address: customerForm.address || undefined
      })
      successMsg.value = 'Customer profile created successfully'
    }
    modals.customer = false
    await loadTabSpecificData()
  } catch (err: any) {
    errorMsg.value = err.response?.data?.message || 'Failed to save customer'
  }
}

// ─── Supplier Actions ───
function openAddSupplierModal() {
  editingId.value = null
  Object.assign(supplierForm, {
    name: '',
    taxNumber: '',
    email: '',
    phone: '',
    address: '',
    isActive: true
  })
  modals.supplier = true
}

function editSupplier(sup: Supplier) {
  editingId.value = sup.id
  Object.assign(supplierForm, {
    name: sup.name,
    taxNumber: sup.taxNumber || '',
    email: sup.email || '',
    phone: sup.phone || '',
    address: sup.address || '',
    isActive: sup.isActive
  })
  modals.supplier = true
}

async function submitSupplier() {
  try {
    errorMsg.value = null
    successMsg.value = null
    if (editingId.value) {
      await supplierApi.update(editingId.value, {
        name: supplierForm.name,
        taxNumber: supplierForm.taxNumber || undefined,
        email: supplierForm.email || undefined,
        phone: supplierForm.phone || undefined,
        address: supplierForm.address || undefined,
        isActive: supplierForm.isActive
      })
      successMsg.value = 'Supplier profile updated successfully'
    } else {
      await supplierApi.create({
        companyId: activeCompanyId.value,
        name: supplierForm.name,
        taxNumber: supplierForm.taxNumber || undefined,
        email: supplierForm.email || undefined,
        phone: supplierForm.phone || undefined,
        address: supplierForm.address || undefined
      })
      successMsg.value = 'Supplier profile created successfully'
    }
    modals.supplier = false
    await loadTabSpecificData()
  } catch (err: any) {
    errorMsg.value = err.response?.data?.message || 'Failed to save supplier'
  }
}

// ─── Bank Actions ───
function openAddBankModal() {
  editingId.value = null
  Object.assign(bankForm, {
    bankName: '',
    accountNumber: '',
    accountName: '',
    currency: 'IDR',
    accountId: '',
    isActive: true
  })
  modals.bank = true
}

function editBank(bank: BankAccount) {
  editingId.value = bank.id
  Object.assign(bankForm, {
    bankName: bank.bankName,
    accountNumber: bank.accountNumber,
    accountName: bank.accountName,
    currency: bank.currency,
    accountId: bank.accountId,
    isActive: bank.isActive
  })
  modals.bank = true
}

async function submitBank() {
  try {
    errorMsg.value = null
    successMsg.value = null
    if (editingId.value) {
      await bankAccountApi.update(editingId.value, {
        accountId: bankForm.accountId,
        bankName: bankForm.bankName,
        accountNumber: bankForm.accountNumber,
        accountName: bankForm.accountName,
        currency: bankForm.currency,
        isActive: bankForm.isActive
      })
      successMsg.value = 'Bank account updated successfully'
    } else {
      await bankAccountApi.create({
        companyId: activeCompanyId.value,
        accountId: bankForm.accountId,
        bankName: bankForm.bankName,
        accountNumber: bankForm.accountNumber,
        accountName: bankForm.accountName,
        currency: bankForm.currency
      })
      successMsg.value = 'Bank account linked successfully'
    }
    modals.bank = false
    await loadTabSpecificData()
  } catch (err: any) {
    errorMsg.value = err.response?.data?.message || 'Failed to save bank account'
  }
}

// ─── Tax Actions ───
function openAddTaxModal() {
  editingId.value = null
  Object.assign(taxForm, {
    code: '',
    name: '',
    category: 'vat_output',
    defaultRate: 0.1100,
    payableAccountId: '',
    effectiveFrom: new Date().toISOString().split('T')[0],
    effectiveTo: undefined,
    isActive: true
  })
  modals.tax = true
}

function editTax(tax: TaxType) {
  editingId.value = tax.id
  Object.assign(taxForm, {
    code: tax.code,
    name: tax.name,
    category: tax.category,
    defaultRate: tax.defaultRate,
    payableAccountId: tax.payableAccountId,
    effectiveFrom: tax.effectiveFrom,
    effectiveTo: tax.effectiveTo || undefined,
    isActive: tax.isActive
  })
  modals.tax = true
}

async function submitTax() {
  try {
    errorMsg.value = null
    successMsg.value = null
    if (editingId.value) {
      await taxTypeApi.update(editingId.value, {
        name: taxForm.name,
        category: taxForm.category,
        defaultRate: Number(taxForm.defaultRate),
        payableAccountId: taxForm.payableAccountId,
        effectiveFrom: taxForm.effectiveFrom,
        effectiveTo: taxForm.effectiveTo || undefined,
        isActive: taxForm.isActive
      })
      successMsg.value = 'Tax configuration updated successfully'
    } else {
      await taxTypeApi.create({
        companyId: activeCompanyId.value,
        code: taxForm.code,
        name: taxForm.name,
        category: taxForm.category,
        defaultRate: Number(taxForm.defaultRate),
        payableAccountId: taxForm.payableAccountId,
        effectiveFrom: taxForm.effectiveFrom,
        effectiveTo: taxForm.effectiveTo || undefined
      })
      successMsg.value = 'Tax configuration created successfully'
    }
    modals.tax = false
    await loadTabSpecificData()
  } catch (err: any) {
    errorMsg.value = err.response?.data?.message || 'Failed to save tax config'
  }
}

// ─── Branch Actions ───
function openAddBranchModal() {
  editingId.value = null
  Object.assign(branchForm, {
    code: '',
    name: '',
    address: '',
    phone: '',
    isActive: true
  })
  modals.branch = true
}

function editBranch(b: Branch) {
  editingId.value = b.id
  Object.assign(branchForm, {
    code: b.code,
    name: b.name,
    address: b.address || '',
    phone: b.phone || '',
    isActive: b.isActive
  })
  modals.branch = true
}

async function submitBranch() {
  if (!activeCompanyId.value) return
  try {
    errorMsg.value = null
    successMsg.value = null
    if (editingId.value) {
      await branchApi.update(editingId.value, {
        code: branchForm.code,
        name: branchForm.name,
        address: branchForm.address || undefined,
        phone: branchForm.phone || undefined,
        isActive: branchForm.isActive
      })
      successMsg.value = 'Branch updated successfully'
    } else {
      await branchApi.create({
        companyId: activeCompanyId.value,
        code: branchForm.code,
        name: branchForm.name,
        address: branchForm.address || undefined,
        phone: branchForm.phone || undefined
      })
      successMsg.value = 'Branch created successfully'
    }
    modals.branch = false
    await loadTabSpecificData()
  } catch (err: any) {
    errorMsg.value = err.response?.data?.message || 'Failed to save branch'
  }
}
</script>
