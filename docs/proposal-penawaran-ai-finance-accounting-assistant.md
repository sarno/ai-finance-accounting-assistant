# PROPOSAL PENAWARAN

## Pengembangan AI Finance & Accounting Assistant

**Disiapkan untuk:** [Nama Client / Perusahaan]  
**Disiapkan oleh:** [Nama Anda / Nama Perusahaan Anda]  
**Tanggal:** 2 Juni 2026  
**Versi:** 1.0  

---

## 1. Ringkasan Proposal

Kami menawarkan pengembangan **AI Finance & Accounting Assistant**, yaitu sistem asisten keuangan dan akuntansi berbasis AI yang memungkinkan owner perusahaan mengetahui kondisi keuangan kapan saja melalui **WhatsApp / Telegram**, sekaligus membantu tim finance dan accounting dalam proses input, validasi, approval, dan pelaporan transaksi keuangan.

Sistem ini dirancang untuk perusahaan yang saat ini masih mengelola data keuangan menggunakan **Excel** dan menyimpan file pada **Synology NAS**, agar dapat bertransformasi menuju sistem keuangan yang lebih terstruktur, otomatis, aman, dan mudah diakses.

Dengan sistem ini, owner dapat bertanya secara natural seperti:

- “Berapa posisi kas hari ini?”
- “Berapa total piutang yang jatuh tempo?”
- “Berapa hutang yang harus dibayar minggu ini?”
- “Berapa laba bersih bulan ini?”
- “Beban operasional terbesar bulan ini apa?”
- “Berapa pajak yang harus disiapkan bulan ini?”

Selain tanya jawab keuangan, sistem juga akan dikembangkan untuk membantu proses accounting seperti:

- Input invoice
- Draft jurnal otomatis
- Pencatatan pembayaran
- Pencatatan hutang dan piutang
- Pencatatan pajak
- Approval transaksi
- Laporan keuangan
- Audit trail

---

## 2. Latar Belakang Masalah

Saat ini perusahaan masih menggunakan file Excel sebagai media utama pencatatan dan pelaporan keuangan. Proses permintaan informasi keuangan masih sangat bergantung kepada karyawan tertentu, sehingga owner tidak selalu bisa mendapatkan informasi secara cepat, terutama di luar jam kerja.

Beberapa kendala yang umum terjadi:

1. Data keuangan tersebar di banyak file Excel.
2. Owner harus menunggu staf finance/accounting untuk mendapatkan laporan.
3. Informasi kas, hutang, piutang, laba rugi, dan pajak tidak selalu tersedia secara real-time.
4. Proses input invoice, pembayaran, dan jurnal masih manual.
5. Risiko kesalahan input cukup tinggi.
6. Proses approval belum terdokumentasi dengan baik.
7. Riwayat perubahan data dan audit trail belum rapi.
8. Data keuangan sulit dianalisis secara cepat.

---

## 3. Solusi yang Ditawarkan

Solusi yang ditawarkan adalah membangun **AI Finance & Accounting Assistant**, yaitu gabungan antara:

1. **Sistem Accounting Custom**  
   Sebagai core pencatatan transaksi keuangan yang menyimpan data invoice, jurnal, kas, bank, hutang, piutang, pajak, dan laporan.

2. **AI Assistant**  
   Membantu owner bertanya kondisi keuangan melalui WhatsApp/Telegram dan membantu tim accounting membuat draft invoice, jurnal, pembayaran, dan transaksi.

3. **Dashboard Web**  
   Digunakan oleh admin, finance, accounting, dan owner untuk monitoring data, approval, laporan, dan koreksi transaksi.

4. **Workflow Approval**  
   Setiap transaksi yang dibuat AI masuk sebagai draft dan harus direview sebelum diposting ke pembukuan.

5. **Pencatatan Pajak**  
   Mencatat PPN Masukan, PPN Keluaran, PPh, pajak terutang, jadwal kewajiban pajak, dan rekap pajak.

---

## 4. Tujuan Project

Tujuan utama project ini adalah:

1. Memudahkan owner mengetahui kondisi keuangan perusahaan kapan saja.
2. Mengurangi ketergantungan owner terhadap staf untuk mendapatkan laporan.
3. Membantu finance/accounting mempercepat input transaksi.
4. Mengurangi risiko kesalahan pencatatan manual.
5. Membuat sistem approval transaksi yang lebih aman.
6. Menyediakan laporan keuangan yang lebih cepat dan terstruktur.
7. Membantu monitoring pajak dan kewajiban pembayaran.
8. Membangun fondasi sistem accounting custom yang bisa dikembangkan jangka panjang.

---

## 5. Ruang Lingkup Pekerjaan

### 5.1 Modul Master Data

- Data perusahaan
- Data user
- Role dan hak akses
- Customer
- Supplier
- Chart of Accounts / COA
- Bank account
- Kategori transaksi
- Setting pajak
- Produk / jasa jika diperlukan

### 5.2 Modul Accounting Core

- Chart of Accounts
- Jurnal umum
- Jurnal otomatis dari transaksi
- Buku besar
- Saldo akun
- Posting transaksi
- Validasi debit dan kredit
- Periode akuntansi
- Lock periode
- Audit trail

### 5.3 Modul Kas dan Bank

- Pencatatan kas masuk
- Pencatatan kas keluar
- Pencatatan mutasi bank
- Transfer antar akun kas/bank
- Saldo kas dan bank
- Rekonsiliasi sederhana
- Laporan posisi kas

### 5.4 Modul Piutang

- Invoice penjualan
- Data customer
- Umur piutang / aging receivable
- Status pembayaran invoice
- Reminder piutang jatuh tempo
- Laporan piutang
- Piutang overdue

### 5.5 Modul Hutang

- Invoice pembelian
- Data supplier/vendor
- Umur hutang / aging payable
- Jadwal pembayaran hutang
- Status pembayaran supplier
- Laporan hutang
- Hutang jatuh tempo

### 5.6 Modul Invoice Automation

- Upload invoice PDF/gambar
- Ekstraksi data invoice dengan OCR/AI
- Validasi vendor
- Validasi nomor invoice
- Validasi tanggal invoice
- Validasi jatuh tempo
- Validasi subtotal, PPN, PPh, dan total
- Deteksi duplikasi invoice
- Draft invoice otomatis
- Draft jurnal otomatis
- Approval sebelum posting

### 5.7 Modul Pembayaran

- Pembayaran hutang
- Penerimaan piutang
- Pembayaran biaya operasional
- Pembayaran pajak
- Matching pembayaran dengan invoice
- Status pembayaran
- Bukti pembayaran
- Approval pembayaran

### 5.8 Modul Pajak

- PPN Masukan
- PPN Keluaran
- Pajak terutang
- PPh 21
- PPh 23
- PPh 25
- PPh Final jika diperlukan
- Jadwal jatuh tempo pajak
- Reminder kewajiban pajak
- Rekap pajak bulanan
- Status dokumen pajak
- Lampiran bukti pajak

**Catatan:** Sistem membantu pencatatan dan monitoring pajak. Perhitungan final dan pelaporan resmi tetap perlu divalidasi oleh pihak finance/accounting atau konsultan pajak perusahaan.

### 5.9 Modul Approval

Status transaksi:

- Draft
- Waiting Review
- Waiting Approval
- Approved
- Posted
- Rejected
- Cancelled

Jenis approval:

- Approval invoice
- Approval jurnal
- Approval pembayaran
- Approval pajak
- Approval koreksi transaksi

### 5.10 Modul AI Assistant

AI Assistant digunakan melalui WhatsApp/Telegram dan dashboard web.

Contoh pertanyaan yang didukung:

- Berapa saldo kas hari ini?
- Berapa piutang jatuh tempo bulan ini?
- Siapa customer dengan piutang terbesar?
- Berapa hutang supplier yang jatuh tempo minggu ini?
- Berapa laba rugi bulan berjalan?
- Beban operasional terbesar bulan ini apa?
- Berapa pajak yang harus disiapkan bulan ini?
- Apakah cashflow aman untuk 3 bulan ke depan?
- Buatkan draft invoice untuk customer tertentu.
- Buatkan draft jurnal dari invoice ini.
- Tampilkan transaksi yang belum di-approve.

Prinsip keamanan AI:

- AI tidak langsung posting transaksi final.
- AI hanya membuat draft.
- Posting final harus melalui approval user yang berwenang.
- Semua aktivitas AI dicatat dalam audit log.

### 5.11 Modul Laporan

- Dashboard ringkasan keuangan
- Laporan kas dan bank
- Laporan piutang
- Laporan hutang
- Laporan laba rugi
- Laporan neraca
- Laporan arus kas
- Laporan beban operasional
- Laporan pajak
- Laporan approval
- Laporan audit trail

---

## 6. Teknologi yang Digunakan

| Komponen | Teknologi |
|---|---|
| Backend API | Rust (Axum Web Framework) |
| Frontend | Vue.js + Pinia |
| Database | PostgreSQL |
| AI / Automation Gateway | OpenClaw |
| AI Model | OpenAI API / model lain sesuai kebutuhan |
| Chat Channel | Telegram Bot API dan/atau WhatsApp Business API |
| File Storage | Synology NAS / Object Storage |
| Background Job | Tokio async worker (Rust) |
| Auth | JWT + Role Based Access Control |
| Deployment | Docker + Docker Compose |
| Reverse Proxy | Nginx / Caddy |
| Monitoring | `tracing` + Grafana / Prometheus |

---

## 7. Arsitektur Sistem

```text
Owner / Finance / Accounting
        ↓
WhatsApp / Telegram / Web Dashboard
        ↓
OpenClaw AI Gateway
        ↓
Rust (Axum) Backend API
        ↓
PostgreSQL Database
        ↓
Accounting Core, Tax, Approval, Reports
        ↓
Synology NAS / File Storage
```

---

## 8. Alur Kerja Sistem

### 8.1 Alur Tanya Jawab Keuangan

```text
Owner bertanya via WhatsApp/Telegram
        ↓
OpenClaw menerima pesan
        ↓
AI memahami intent pertanyaan
        ↓
AI memanggil API backend
        ↓
Backend mengambil data dari database
        ↓
AI menyusun jawaban ringkas
        ↓
Jawaban dikirim ke owner
```

### 8.2 Alur Input Invoice

```text
User upload invoice
        ↓
Sistem membaca dokumen dengan OCR/AI
        ↓
Data invoice diekstrak
        ↓
Backend melakukan validasi
        ↓
Sistem membuat draft invoice
        ↓
Sistem membuat draft jurnal
        ↓
Finance melakukan review
        ↓
Owner/Manager melakukan approval
        ↓
Transaksi diposting ke accounting
```

### 8.3 Alur Posting Jurnal

```text
Draft jurnal dibuat
        ↓
Validasi debit dan kredit
        ↓
Review oleh accounting
        ↓
Approval oleh user berwenang
        ↓
Posting jurnal
        ↓
Masuk laporan keuangan
```

---

## 9. Keamanan Sistem

Fitur keamanan yang disarankan:

1. Login user dengan JWT.
2. Role Based Access Control.
3. Pembatasan akses berdasarkan role.
4. Whitelist nomor WhatsApp/Telegram.
5. Approval sebelum posting transaksi.
6. Audit log untuk semua aktivitas.
7. Backup database otomatis.
8. HTTPS/SSL.
9. Validasi input di backend.
10. Logging AI prompt dan tool call.
11. Pembatasan akses dokumen keuangan.
12. Proteksi terhadap perubahan data setelah periode dikunci.
13. Enkripsi informasi sensitif jika diperlukan.

---

## 10. Dasar Estimasi Harga

Estimasi harga pada proposal ini disusun berdasarkan pendekatan **project-based pricing** dengan mempertimbangkan kompleksitas sistem, jumlah modul, integrasi AI, integrasi chat, kebutuhan accounting core, workflow approval, dan risiko implementasi.

Benchmark pasar Indonesia yang digunakan sebagai pembanding:

- Tarif freelance engineer Indonesia umumnya berada pada kisaran **Rp100.000 - Rp700.000 per jam** tergantung level junior, mid-level, atau senior.
- Untuk pekerjaan mid-level, kisaran umum dapat berada di **Rp200.000 - Rp400.000 per jam**.
- Untuk pekerjaan senior, kisaran umum dapat berada di **Rp400.000 - Rp700.000 per jam**.
- Web aplikasi custom / ERP / SaaS di Indonesia umumnya mulai dari **Rp50 juta** dan dapat masuk ke **ratusan juta rupiah** tergantung kompleksitas.

Dengan mempertimbangkan project ini memiliki backend custom, frontend dashboard, database accounting, AI assistant, integrasi OpenClaw, approval workflow, pajak, dan potensi OCR invoice, maka estimasi harga wajar berada pada kategori **aplikasi bisnis custom menengah sampai kompleks**.

---

## 11. Opsi Paket Penawaran

### Paket 1 - Discovery & Prototype AI Finance Assistant

**Fokus:** Validasi kebutuhan, audit data Excel, prototype tanya jawab keuangan, dan demo awal chatbot.

Ruang lingkup:

- Discovery proses finance/accounting
- Audit file Excel existing
- Mapping data dari Excel ke database
- Desain awal database
- Prototype dashboard ringkasan
- Prototype Telegram bot / chat interface
- Tanya jawab dasar: kas, hutang, piutang, laba rugi sederhana
- Dokumen rekomendasi roadmap implementasi

Estimasi durasi: **3 - 5 minggu**

**Harga penawaran:** **Rp35.000.000 - Rp55.000.000**

Cocok untuk client yang ingin membuktikan manfaat AI Assistant sebelum masuk ke sistem accounting penuh.

---

### Paket 2 - MVP AI Finance Assistant + Accounting Core Dasar

**Fokus:** Membangun fondasi sistem accounting custom dan dashboard owner.

Ruang lingkup:

- Backend Rust (Axum) + SQLx
- Frontend Vue.js + Pinia
- PostgreSQL database
- Login dan role sederhana
- Master data: customer, supplier, COA, bank account
- Import data awal dari Excel
- Modul kas/bank dasar
- Modul hutang dan piutang dasar
- Jurnal umum dasar
- Dashboard owner
- Laporan dasar: kas, hutang, piutang, laba rugi sederhana
- Telegram AI Assistant
- Audit log dasar

Estimasi durasi: **8 - 12 minggu**

**Harga penawaran:** **Rp95.000.000 - Rp145.000.000**

Cocok untuk tahap awal digitalisasi keuangan perusahaan.

---

### Paket 3 - AI Accounting Automation Assistant

**Fokus:** Sistem accounting operasional dengan AI draft transaction, approval, pembayaran, dan pajak dasar.

Ruang lingkup:

- Semua fitur Paket 2
- Invoice penjualan
- Invoice pembelian
- Payment received
- Payment paid
- Expense transaction
- Draft jurnal otomatis
- Posting jurnal otomatis
- Approval workflow
- Pencatatan PPN Masukan
- Pencatatan PPN Keluaran
- Pencatatan PPh dasar
- Rekap pajak bulanan
- Reminder hutang/piutang jatuh tempo
- Reminder pajak
- AI Assistant untuk tanya jawab laporan
- AI Assistant untuk membuat draft invoice/jurnal/pembayaran
- Dashboard approval
- Laporan laba rugi, neraca sederhana, arus kas sederhana
- Integrasi Telegram production
- Persiapan integrasi WhatsApp

Estimasi durasi: **16 - 24 minggu**

**Harga penawaran:** **Rp185.000.000 - Rp285.000.000**

Cocok untuk client yang ingin sistem accounting custom mulai dipakai operasional.

---

### Paket 4 - Full AI Finance & Accounting System

**Fokus:** Sistem lengkap dengan OCR invoice, WhatsApp, laporan advanced, pajak lebih lengkap, dan hardening production.

Ruang lingkup:

- Semua fitur Paket 3
- OCR invoice PDF/gambar
- Ekstraksi invoice otomatis
- Confidence score hasil ekstraksi
- Validasi duplikasi invoice
- Validasi NPWP/vendor
- Validasi pajak invoice
- Approval multi-level
- WhatsApp Business API / gateway integration
- Advanced dashboard owner
- Laporan laba rugi lengkap
- Neraca lengkap
- Arus kas lengkap
- Aging piutang dan hutang
- Tax dashboard
- Audit trail lengkap
- Export PDF/Excel
- Cashflow forecast sederhana
- AI insight dan rekomendasi
- Backup dan restore procedure
- Production deployment
- Dokumentasi admin dan user
- Training user

Estimasi durasi: **24 - 32 minggu**

**Harga penawaran:** **Rp350.000.000 - Rp550.000.000**

Cocok untuk client yang ingin membangun sistem jangka panjang sebagai platform keuangan internal perusahaan.

---

## 12. Rekomendasi Penawaran untuk Client

Untuk client yang saat ini masih menggunakan Excel dan Synology, rekomendasi paling aman adalah memulai dari:

**Paket 1 + Paket 2**

Dengan pendekatan ini, client tidak langsung mengambil risiko besar. Project dimulai dari discovery, audit data, prototype, lalu dilanjutkan ke accounting core dasar dan dashboard owner.

Setelah sistem dasar terbukti berjalan, pengembangan dapat dilanjutkan ke:

- Paket 3 untuk automation accounting.
- Paket 4 untuk OCR, WhatsApp production, dan laporan advanced.

---

## 13. Estimasi Biaya Tambahan Operasional

Biaya berikut tidak termasuk dalam harga pengembangan dan akan menjadi tanggungan client jika diperlukan:

| Komponen | Estimasi Biaya |
|---|---:|
| VPS / Cloud Server | Rp500.000 - Rp3.000.000 / bulan |
| Domain | Rp150.000 - Rp500.000 / tahun |
| SSL | Gratis - Rp1.500.000 / tahun |
| WhatsApp Business API / Gateway | Menyesuaikan provider |
| OpenAI API / AI Model | Menyesuaikan pemakaian token |
| OCR pihak ketiga | Menyesuaikan jumlah dokumen |
| Backup storage | Rp100.000 - Rp1.000.000 / bulan |
| Maintenance setelah masa support | 10% - 20% dari nilai project / tahun atau retainer bulanan |

---

## 14. Skema Pembayaran

### Skema 1 - Per Paket

```text
40% - Down Payment
30% - Setelah milestone development utama selesai
20% - Setelah UAT
10% - Setelah Go-Live
```

### Skema 2 - Per Milestone

```text
30% - Down Payment
20% - Setelah backend dan database foundation selesai
20% - Setelah frontend dashboard dan accounting core selesai
20% - Setelah AI assistant dan approval workflow selesai
10% - Setelah testing, training, dan go-live
```

### Skema 3 - Retainer Bulanan untuk Development Bertahap

```text
Rp35.000.000 - Rp65.000.000 / bulan
```

Cocok jika client ingin pengembangan dilakukan bertahap dengan prioritas fitur yang fleksibel.

---

## 15. Deliverables

Deliverables project:

1. Source code backend Rust (Axum).
2. Source code frontend Vue.js + Pinia.
3. Database PostgreSQL.
4. OpenClaw workflow configuration.
5. API documentation.
6. Database schema documentation.
7. User manual.
8. Admin manual.
9. Deployment documentation.
10. Training user.
11. Production deployment.
12. Backup configuration.
13. Basic support setelah go-live sesuai paket.

---

## 16. Batasan Pekerjaan

Hal-hal berikut tidak termasuk kecuali disepakati terpisah:

1. Biaya server/VPS/cloud.
2. Biaya WhatsApp Business API/gateway.
3. Biaya OpenAI API/model AI.
4. Biaya OCR pihak ketiga.
5. Biaya domain dan SSL premium.
6. Biaya konsultan pajak.
7. Integrasi ke DJP/e-Faktur/e-Bupot jika membutuhkan akses resmi/API khusus.
8. Migrasi data historis dalam jumlah besar jika format data tidak konsisten.
9. Custom laporan tambahan di luar scope awal.
10. Maintenance jangka panjang setelah masa support selesai.

---

## 17. Risiko dan Mitigasi

### Risiko 1 - Data Excel Tidak Konsisten

Mitigasi:

- Dilakukan tahap discovery.
- Dibuat template import.
- Dibuat proses validasi data.

### Risiko 2 - AI Salah Membaca Invoice

Mitigasi:

- AI hanya membuat draft.
- Finance wajib review.
- Sistem menampilkan confidence score.
- Approval sebelum posting.

### Risiko 3 - Kesalahan Jurnal

Mitigasi:

- Backend melakukan validasi debit/kredit.
- Mapping COA dibuat jelas.
- Draft jurnal bisa diedit sebelum posting.
- Audit trail dicatat.

### Risiko 4 - Pajak Salah Hitung

Mitigasi:

- Sistem membantu pencatatan dan monitoring.
- Perhitungan final pajak harus divalidasi finance/konsultan pajak.
- Rule pajak dibuat configurable.

### Risiko 5 - Akses Data Sensitif

Mitigasi:

- Role-based access.
- Whitelist user.
- Audit log.
- Enkripsi koneksi.
- Backup database.

---

## 18. Syarat Kerja Sama

1. Pembayaran dilakukan bertahap berdasarkan milestone.
2. Setiap milestone memiliki deliverable yang jelas.
3. Perubahan scope di luar kesepakatan akan dibuatkan estimasi tambahan.
4. Client menyediakan contoh data Excel, format invoice, COA, dan kebutuhan laporan.
5. Client menunjuk PIC untuk validasi proses accounting dan pajak.
6. Approval final atas alur accounting dan pajak dilakukan oleh pihak client.
7. Sistem AI tidak menggantikan tanggung jawab profesional finance/accounting, tetapi membantu otomasi dan percepatan kerja.

---

## 19. Masa Support

Masa support yang disarankan:

| Paket | Masa Support Setelah Go-Live |
|---|---:|
| Paket 1 | 14 hari |
| Paket 2 | 30 hari |
| Paket 3 | 45 hari |
| Paket 4 | 60 hari |

Support mencakup:

- Bug fixing sesuai scope.
- Bantuan penggunaan sistem.
- Monitoring awal setelah go-live.
- Perbaikan minor yang tidak mengubah scope utama.

---

## 20. Penutup

Dengan adanya **AI Finance & Accounting Assistant**, perusahaan akan memiliki sistem keuangan yang lebih cepat, rapi, aman, dan mudah diakses. Owner dapat mengetahui kondisi keuangan kapan saja melalui WhatsApp/Telegram, sementara tim finance dan accounting tetap memiliki kontrol penuh melalui proses review dan approval.

Sistem ini tidak hanya menjadi chatbot, tetapi menjadi fondasi transformasi digital untuk proses finance dan accounting perusahaan.

---

**Disiapkan oleh:**  
[Nama Anda / Nama Perusahaan Anda]

**Kontak:**  
[Nomor WhatsApp]  
[Email]  
[Website / Instagram]

---

## Lampiran: Catatan Benchmark Harga

Benchmark publik yang menjadi referensi umum:

1. Timedoor Indonesia - kisaran tarif freelance engineer Indonesia, termasuk junior, mid-level, dan senior freelancer.  
   https://id.timedoor.net/blogs/Panduan-Lengkap-tentang-Tingkat-Gaji-Tarif-dan-Karakteristik-Insinyur-IT-Indonesia/

2. Jobstreet Indonesia - kisaran gaji Software Developer dan Front End Developer Indonesia.  
   https://id.jobstreet.com/id/career-advice/role/software-developer/salary  
   https://id.jobstreet.com/id/career-advice/role/frontend-developer/salary

3. Lemon.io Rate Calculator - median billing rate senior software developer Indonesia 2026.  
   https://lemon.io/rate-calculator/indonesia/

4. KreasiAI - estimasi web aplikasi custom ERP/SaaS mulai dari Rp50 juta sampai ratusan juta.  
   https://www.kreasiai.com/detail-blog/biaya-rata-rata-pembuatan-website-di-indonesia-2025-panduan-lengkap-untuk-bisnis-anda
