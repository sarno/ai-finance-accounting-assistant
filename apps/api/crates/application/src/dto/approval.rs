use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApprovalRequest {
    pub company_id: Uuid,
    pub document_type: String,
    pub document_id: Uuid,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApproveRequest {
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RejectRequest {
    pub reason: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApprovalResponse {
    pub id: Uuid,
    pub company_id: Uuid,
    pub document_type: String,
    pub document_id: Uuid,
    pub status: String,
    pub requested_by: Uuid,
    pub reviewed_by: Option<Uuid>,
    pub reviewed_at: Option<time::OffsetDateTime>,
    pub comment: Option<String>,
    pub created_at: time::OffsetDateTime,
    pub updated_at: time::OffsetDateTime,
}

use finance_assistant_domain::entities::approval::ApprovalRequest as DomainApprovalRequest;

impl From<DomainApprovalRequest> for ApprovalResponse {
    fn from(r: DomainApprovalRequest) -> Self {
        Self {
            id: r.id,
            company_id: r.company_id,
            document_type: r.document_type.as_str().to_string(),
            document_id: r.document_id,
            status: r.status.as_str().to_string(),
            requested_by: r.requested_by,
            reviewed_by: r.reviewed_by,
            reviewed_at: r.reviewed_at,
            comment: r.comment,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}
