import client from './client'
import type {
  Company, CreateCompanyRequest, UpdateCompanyRequest,
  Account, CreateAccountRequest, UpdateAccountRequest,
  Customer, CreateCustomerRequest, UpdateCustomerRequest,
  Supplier, CreateSupplierRequest, UpdateSupplierRequest,
  BankAccount, CreateBankAccountRequest, UpdateBankAccountRequest,
  TaxType, CreateTaxTypeRequest, UpdateTaxTypeRequest,
  Branch, CreateBranchRequest, UpdateBranchRequest,
  ItemCategory, CreateItemCategoryRequest, UpdateItemCategoryRequest,
  Item, CreateItemRequest, UpdateItemRequest,
  TaxRecord, TaxSummaryResponse, TaxCalendarEntry, CreateTaxCalendarRequest, UpdateTaxCalendarStatusRequest
} from '@/types/master-data.types'

export const companyApi = {
  list: async (): Promise<Company[]> => {
    const { data } = await client.get<Company[]>('/companies')
    return data
  },
  getById: async (id: string): Promise<Company> => {
    const { data } = await client.get<Company>(`/companies/${id}`)
    return data
  },
  create: async (req: CreateCompanyRequest): Promise<Company> => {
    const { data } = await client.post<Company>('/companies', req)
    return data
  },
  update: async (id: string, req: UpdateCompanyRequest): Promise<Company> => {
    const { data } = await client.put<Company>(`/companies/${id}`, req)
    return data
  }
}

export const accountApi = {
  listByCompany: async (companyId: string): Promise<Account[]> => {
    const { data } = await client.get<Account[]>(`/companies/${companyId}/accounts`)
    return data
  },
  getById: async (id: string): Promise<Account> => {
    const { data } = await client.get<Account>(`/accounts/${id}`)
    return data
  },
  create: async (req: CreateAccountRequest): Promise<Account> => {
    const { data } = await client.post<Account>('/accounts', req)
    return data
  },
  update: async (id: string, req: UpdateAccountRequest): Promise<Account> => {
    const { data } = await client.put<Account>(`/accounts/${id}`, req)
    return data
  }
}

export const customerApi = {
  listByCompany: async (companyId: string): Promise<Customer[]> => {
    const { data } = await client.get<Customer[]>(`/companies/${companyId}/customers`)
    return data
  },
  getById: async (id: string): Promise<Customer> => {
    const { data } = await client.get<Customer>(`/customers/${id}`)
    return data
  },
  create: async (req: CreateCustomerRequest): Promise<Customer> => {
    const { data } = await client.post<Customer>('/customers', req)
    return data
  },
  update: async (id: string, req: UpdateCustomerRequest): Promise<Customer> => {
    const { data } = await client.put<Customer>(`/customers/${id}`, req)
    return data
  }
}

export const supplierApi = {
  listByCompany: async (companyId: string): Promise<Supplier[]> => {
    const { data } = await client.get<Supplier[]>(`/companies/${companyId}/suppliers`)
    return data
  },
  getById: async (id: string): Promise<Supplier> => {
    const { data } = await client.get<Supplier>(`/suppliers/${id}`)
    return data
  },
  create: async (req: CreateSupplierRequest): Promise<Supplier> => {
    const { data } = await client.post<Supplier>('/suppliers', req)
    return data
  },
  update: async (id: string, req: UpdateSupplierRequest): Promise<Supplier> => {
    const { data } = await client.put<Supplier>(`/suppliers/${id}`, req)
    return data
  }
}

export const bankAccountApi = {
  listByCompany: async (companyId: string): Promise<BankAccount[]> => {
    const { data } = await client.get<BankAccount[]>(`/companies/${companyId}/bank-accounts`)
    return data
  },
  getById: async (id: string): Promise<BankAccount> => {
    const { data } = await client.get<BankAccount>(`/bank-accounts/${id}`)
    return data
  },
  create: async (req: CreateBankAccountRequest): Promise<BankAccount> => {
    const { data } = await client.post<BankAccount>('/bank-accounts', req)
    return data
  },
  update: async (id: string, req: UpdateBankAccountRequest): Promise<BankAccount> => {
    const { data } = await client.put<BankAccount>(`/bank-accounts/${id}`, req)
    return data
  }
}

export const taxTypeApi = {
  listByCompany: async (companyId: string): Promise<TaxType[]> => {
    const { data } = await client.get<TaxType[]>(`/companies/${companyId}/tax-types`)
    return data
  },
  getById: async (id: string): Promise<TaxType> => {
    const { data } = await client.get<TaxType>(`/tax-types/${id}`)
    return data
  },
  create: async (req: CreateTaxTypeRequest): Promise<TaxType> => {
    const { data } = await client.post<TaxType>('/tax-types', req)
    return data
  },
  update: async (id: string, req: UpdateTaxTypeRequest): Promise<TaxType> => {
    const { data } = await client.put<TaxType>(`/tax-types/${id}`, req)
    return data
  }
}

export const branchApi = {
  listByCompany: async (companyId: string): Promise<Branch[]> => {
    const { data } = await client.get<Branch[]>(`/companies/${companyId}/branches`)
    return data
  },
  getById: async (id: string): Promise<Branch> => {
    const { data } = await client.get<Branch>(`/branches/${id}`)
    return data
  },
  create: async (req: CreateBranchRequest): Promise<Branch> => {
    const { data } = await client.post<Branch>('/branches', req)
    return data
  },
  update: async (id: string, req: UpdateBranchRequest): Promise<Branch> => {
    const { data } = await client.put<Branch>(`/branches/${id}`, req)
    return data
  }
}

export const itemCategoryApi = {
  listByCompany: async (companyId: string): Promise<ItemCategory[]> => {
    const { data } = await client.get<ItemCategory[]>(`/companies/${companyId}/item-categories`)
    return data
  },
  getById: async (id: string): Promise<ItemCategory> => {
    const { data } = await client.get<ItemCategory>(`/item-categories/${id}`)
    return data
  },
  create: async (req: CreateItemCategoryRequest): Promise<ItemCategory> => {
    const { data } = await client.post<ItemCategory>('/item-categories', req)
    return data
  },
  update: async (id: string, req: UpdateItemCategoryRequest): Promise<ItemCategory> => {
    const { data } = await client.put<ItemCategory>(`/item-categories/${id}`, req)
    return data
  },
  delete: async (id: string): Promise<void> => {
    await client.delete(`/item-categories/${id}`)
  }
}

export const itemApi = {
  listByCompany: async (companyId: string): Promise<Item[]> => {
    const { data } = await client.get<Item[]>(`/companies/${companyId}/items`)
    return data
  },
  getById: async (id: string): Promise<Item> => {
    const { data } = await client.get<Item>(`/items/${id}`)
    return data
  },
  create: async (req: CreateItemRequest): Promise<Item> => {
    const { data } = await client.post<Item>('/items', req)
    return data
  },
  update: async (id: string, req: UpdateItemRequest): Promise<Item> => {
    const { data } = await client.put<Item>(`/items/${id}`, req)
    return data
  },
  delete: async (id: string): Promise<void> => {
    await client.delete(`/items/${id}`)
  }
}

export const taxRecordApi = {
  listByCompany: async (companyId: string, page?: number, perPage?: number): Promise<{ records: TaxRecord[], totalCount: number }> => {
    const params = { page, perPage }
    const response = await client.get<TaxRecord[]>(`/companies/${companyId}/tax-records`, { params })
    const totalCount = parseInt(response.headers['x-total-count'] || '0', 10)
    return { records: response.data, totalCount }
  },
  getSummary: async (companyId: string, startDate: string, endDate: string): Promise<TaxSummaryResponse> => {
    const { data } = await client.get<TaxSummaryResponse>(`/companies/${companyId}/tax-summary`, {
      params: { startDate, endDate }
    })
    return data
  }
}

export const taxCalendarApi = {
  listByCompany: async (companyId: string): Promise<TaxCalendarEntry[]> => {
    const { data } = await client.get<TaxCalendarEntry[]>(`/companies/${companyId}/tax-calendar`)
    return data
  },
  create: async (req: CreateTaxCalendarRequest): Promise<TaxCalendarEntry> => {
    const { data } = await client.post<TaxCalendarEntry>('/tax-calendar', req)
    return data
  },
  updateStatus: async (id: string, req: UpdateTaxCalendarStatusRequest): Promise<TaxCalendarEntry> => {
    const { data } = await client.put<TaxCalendarEntry>(`/tax-calendar/${id}/status`, req)
    return data
  }
}


