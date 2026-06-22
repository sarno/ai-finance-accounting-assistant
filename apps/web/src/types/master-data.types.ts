export interface Company {
  id: string
  name: string
  taxNumber?: string
  address?: string
  currency: string
  isActive: boolean
  createdAt: string
}

export interface CreateCompanyRequest {
  name: string
  taxNumber?: string
  address?: string
  currency: string
}

export interface UpdateCompanyRequest {
  name: string
  taxNumber?: string
  address?: string
  currency: string
  isActive: boolean
}

export interface Account {
  id: string
  companyId: string
  code: string
  name: string
  accountType: string
  parentId?: string
  isActive: boolean
  createdAt: string
}

export interface CreateAccountRequest {
  companyId: string
  code: string
  name: string
  accountType: string
  parentId?: string
}

export interface UpdateAccountRequest {
  code: string
  name: string
  accountType: string
  parentId?: string
  isActive: boolean
}

export interface Customer {
  id: string
  companyId: string
  name: string
  taxNumber?: string
  email?: string
  phone?: string
  address?: string
  isActive: boolean
  createdAt: string
}

export interface CreateCustomerRequest {
  companyId: string
  name: string
  taxNumber?: string
  email?: string
  phone?: string
  address?: string
}

export interface UpdateCustomerRequest {
  name: string
  taxNumber?: string
  email?: string
  phone?: string
  address?: string
  isActive: boolean
}

export interface Supplier {
  id: string
  companyId: string
  name: string
  taxNumber?: string
  email?: string
  phone?: string
  address?: string
  isActive: boolean
  createdAt: string
}

export interface CreateSupplierRequest {
  companyId: string
  name: string
  taxNumber?: string
  email?: string
  phone?: string
  address?: string
}

export interface UpdateSupplierRequest {
  name: string
  taxNumber?: string
  email?: string
  phone?: string
  address?: string
  isActive: boolean
}

export interface BankAccount {
  id: string
  companyId: string
  accountId: string
  bankName: string
  accountNumber: string
  accountName: string
  currency: string
  isActive: boolean
  createdAt: string
}

export interface CreateBankAccountRequest {
  companyId: string
  accountId: string
  bankName: string
  accountNumber: string
  accountName: string
  currency: string
}

export interface UpdateBankAccountRequest {
  accountId: string
  bankName: string
  accountNumber: string
  accountName: string
  currency: string
  isActive: boolean
}

export interface TaxType {
  id: string
  companyId: string
  code: string
  name: string
  category: string
  defaultRate: number
  payableAccountId: string
  effectiveFrom: string
  effectiveTo?: string
  isActive: boolean
  createdAt: string
}

export interface CreateTaxTypeRequest {
  companyId: string
  code: string
  name: string
  category: string
  defaultRate: number
  payableAccountId: string
  effectiveFrom: string
  effectiveTo?: string
}

export interface UpdateTaxTypeRequest {
  name: string
  category: string
  defaultRate: number
  payableAccountId: string
  effectiveFrom: string
  effectiveTo?: string
  isActive: boolean
}

export interface Branch {
  id: string
  companyId: string
  code: string
  name: string
  address?: string
  phone?: string
  isActive: boolean
  createdAt: string
}

export interface CreateBranchRequest {
  companyId: string
  code: string
  name: string
  address?: string
  phone?: string
}

export interface UpdateBranchRequest {
  code: string
  name: string
  address?: string
  phone?: string
  isActive: boolean
}

export interface ItemCategory {
  id: string
  companyId: string
  name: string
  description?: string
  isActive: boolean
  createdAt: string
  updatedAt: string
}

export interface CreateItemCategoryRequest {
  companyId: string
  name: string
  description?: string
}

export interface UpdateItemCategoryRequest {
  name: string
  description?: string
  isActive: boolean
}

export interface Item {
  id: string
  companyId: string
  categoryId?: string
  code: string
  name: string
  description?: string
  unitPrice: number
  saleAccountId?: string
  purchaseAccountId?: string
  taxTypeId?: string
  isActive: boolean
  createdAt: string
  updatedAt: string
}

export interface CreateItemRequest {
  companyId: string
  categoryId?: string
  code: string
  name: string
  description?: string
  unitPrice: number
  saleAccountId?: string
  purchaseAccountId?: string
  taxTypeId?: string
}

export interface UpdateItemRequest {
  categoryId?: string
  code: string
  name: string
  description?: string
  unitPrice: number
  saleAccountId?: string
  purchaseAccountId?: string
  taxTypeId?: string
  isActive: boolean
}

export type TaxRecordStatus = 'not_required' | 'required' | 'drafted' | 'validated' | 'reported' | 'paid' | 'archived'
export type TaxPaymentStatus = 'unpaid' | 'paid'
export type TaxFilingStatus = 'unfiled' | 'filed'

export interface TaxRecord {
  id: string
  companyId: string
  taxTypeId: string
  sourceDocumentType: string
  sourceDocumentId: string
  taxPeriod: string
  taxBaseAmount: number
  taxRate: number
  taxAmount: number
  status: TaxRecordStatus
  counterpartyName?: string
  counterpartyNpwp?: string
  createdAt: string
}

export interface TaxSummaryResponse {
  totalVatOutput: number
  totalVatInput: number
  netTaxDue: number
  records: TaxRecord[]
}

export interface TaxCalendarEntry {
  id: string
  companyId: string
  taxTypeId: string
  taxPeriod: string
  paymentDueDate: string
  filingDueDate: string
  paymentStatus: TaxPaymentStatus
  filingStatus: TaxFilingStatus
  reminderSentAt?: string
  createdAt: string
}

export interface CreateTaxCalendarRequest {
  companyId: string
  taxTypeId: string
  taxPeriod: string
  paymentDueDate: string
  filingDueDate: string
}

export interface UpdateTaxCalendarStatusRequest {
  paymentStatus?: TaxPaymentStatus
  filingStatus?: TaxFilingStatus
}
