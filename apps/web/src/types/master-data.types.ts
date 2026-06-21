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

