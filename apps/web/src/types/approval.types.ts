export interface ApprovalRequest {
  id: string
  companyId: string
  documentType: string
  documentId: string
  status: 'pending' | 'approved' | 'rejected' | 'cancelled'
  requestedBy: string
  reviewedBy?: string
  reviewedAt?: string
  comment?: string
  createdAt: string
}
