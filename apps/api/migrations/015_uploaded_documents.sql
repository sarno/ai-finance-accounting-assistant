-- Migration: Add uploaded_documents table
CREATE TABLE uploaded_documents (
    id UUID PRIMARY KEY,
    company_id UUID NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    original_file_name TEXT NOT NULL,
    storage_path TEXT NOT NULL,
    mime_type TEXT,
    size_bytes BIGINT,
    document_type TEXT,
    status TEXT NOT NULL DEFAULT 'pending',
    extracted_fields JSONB,
    validation_results JSONB,
    ai_confidence DOUBLE PRECISION,
    uploaded_by UUID REFERENCES users(id) ON DELETE SET NULL,
    uploaded_at TIMESTAMPTZ NOT NULL,
    error_message TEXT
);

CREATE INDEX idx_uploaded_documents_company ON uploaded_documents(company_id);
CREATE INDEX idx_uploaded_documents_status ON uploaded_documents(status);
