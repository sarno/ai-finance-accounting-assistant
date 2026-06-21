export interface ApprovalRequest {
  id: string
  companyId: string
  documentType: string
  documentId: string
  status: 'pending' | 'approved' | 'rejected' | 'cancelled'
  requestedBy: string
  requestedByName?: string
  documentReference?: string
  reviewedBy?: string
  reviewedAt?: string
  comment?: string
  createdAt: string
}
