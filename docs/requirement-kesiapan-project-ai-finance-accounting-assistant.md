# Requirement Kesiapan Project
## AI Finance & Accounting Assistant / AI Accounting Automation Assistant

Dokumen ini berisi syarat, kesiapan, dan requirement non-teknis maupun teknis agar project **AI Finance & Accounting Assistant / AI Accounting Automation Assistant** dapat diwujudkan menjadi sistem nyata dan siap digunakan oleh perusahaan.

---

## 1. Tujuan Dokumen

Dokumen ini dibuat untuk menjadi panduan awal sebelum project masuk ke tahap development.

Tujuannya adalah memastikan bahwa:

1. Data keuangan client siap digunakan.
2. Alur accounting client sudah jelas.
3. PIC dari pihak client tersedia.
4. Modul yang akan dibangun memiliki batasan scope yang jelas.
5. Risiko project dapat dikurangi sejak awal.
6. Implementasi dilakukan bertahap dan realistis.
7. Sistem AI tidak berjalan tanpa kontrol manusia.
8. Keamanan dan audit trail disiapkan sejak awal.

---

## 2. Ringkasan Syarat Utama

Agar project ini dapat berjalan dengan baik, syarat utama yang harus dipenuhi adalah:

```text
Data rapi
+ PIC client aktif
+ alur accounting jelas
+ COA tersedia
+ aturan pajak jelas
+ approval wajib
+ audit trail
+ infrastruktur siap
+ budget operasional
+ implementasi bertahap
+ SOP dan training
```

---

## 3. Kesiapan Data Keuangan

### 3.1 Data yang Perlu Disiapkan Client

Client perlu menyiapkan data-data berikut:

- File Excel kas/bank
- File hutang
- File piutang
- File penjualan
- File pembelian
- File biaya operasional
- Data customer
- Data supplier/vendor
- Chart of Accounts / daftar akun
- Data pajak
- Contoh invoice penjualan
- Contoh invoice pembelian
- Contoh bukti pembayaran
- Contoh laporan laba rugi
- Contoh laporan neraca
- Contoh laporan arus kas
- Contoh laporan pajak jika sudah ada

### 3.2 Kondisi Data yang Ideal

Data idealnya memiliki karakteristik berikut:

- Format kolom konsisten
- Tanggal transaksi jelas
- Nomor dokumen jelas
- Nama customer/vendor konsisten
- Nilai transaksi jelas
- Pajak dipisahkan jika ada
- Status pembayaran jelas
- Tidak banyak file duplikat
- Tidak banyak sheet dengan format berbeda-beda
- Ada periode transaksi yang jelas

### 3.3 Jika Data Belum Rapi

Jika file Excel masih berantakan, maka perlu dibuat tahap **Data Cleaning & Data Standardization**.

Aktivitasnya meliputi:

- Merapikan struktur file Excel
- Menyamakan format kolom
- Membersihkan data duplikat
- Mapping customer/vendor
- Mapping akun accounting
- Mapping pajak
- Menentukan template import standar

---

## 4. Kesiapan PIC dari Pihak Client

Project ini membutuhkan PIC aktif dari pihak client.

### 4.1 PIC yang Diperlukan

Minimal client menyediakan:

1. **Owner / Direktur**
   - Pengambil keputusan utama
   - Menentukan prioritas laporan
   - Menentukan user yang boleh akses

2. **Finance / Accounting PIC**
   - Menjelaskan proses pembukuan
   - Memvalidasi jurnal
   - Memvalidasi laporan
   - Memvalidasi aturan pajak

3. **Admin Data / IT Internal**
   - Menyediakan file Excel
   - Menyediakan akses Synology
   - Membantu pengujian data
   - Membantu proses migrasi

### 4.2 Pentingnya PIC

Tanpa PIC yang aktif, project berisiko:

- Requirement tidak jelas
- Alur accounting salah dipahami
- Proses approval lambat
- Testing tidak selesai
- Sistem tidak sesuai kebutuhan operasional
- Scope melebar tanpa kontrol

---

## 5. Kesiapan Alur Accounting

Sebelum development besar dimulai, alur accounting harus disepakati.

### 5.1 Proses yang Harus Dipetakan

Beberapa proses yang harus jelas:

- Bagaimana penjualan dicatat
- Bagaimana pembelian dicatat
- Bagaimana hutang dicatat
- Bagaimana piutang dicatat
- Bagaimana pembayaran customer dicatat
- Bagaimana pembayaran supplier dicatat
- Bagaimana biaya operasional dicatat
- Bagaimana pajak dicatat
- Bagaimana jurnal manual dibuat
- Bagaimana koreksi transaksi dilakukan
- Kapan transaksi dianggap final
- Siapa yang boleh approve
- Siapa yang boleh posting
- Siapa yang boleh membatalkan transaksi

### 5.2 Prinsip Aman untuk AI

AI tidak boleh langsung melakukan posting final ke pembukuan.

Alur aman:

```text
AI membuat draft
      ↓
Finance review
      ↓
Owner / Manager approve
      ↓
Sistem posting transaksi
      ↓
Masuk laporan keuangan
```

---

## 6. Chart of Accounts / COA

### 6.1 Kebutuhan COA

Sistem accounting custom wajib memiliki **Chart of Accounts**.

COA menjadi dasar untuk:

- Jurnal
- Buku besar
- Laba rugi
- Neraca
- Arus kas
- Pencatatan pajak
- Mapping transaksi otomatis oleh AI

### 6.2 Contoh Kelompok Akun

Contoh kelompok akun:

- Aset
- Kas dan Bank
- Piutang Usaha
- Persediaan
- Aset Tetap
- Hutang Usaha
- Hutang Pajak
- Modal
- Pendapatan
- Harga Pokok Penjualan
- Beban Operasional
- Beban Gaji
- Beban Sewa
- Beban Transportasi
- PPN Masukan
- PPN Keluaran
- Pajak Terutang
- Laba Ditahan

### 6.3 Jika Client Belum Punya COA

Jika client belum memiliki COA yang rapi, maka perlu dibuatkan:

- Draft COA
- Mapping dari transaksi Excel ke akun
- Validasi oleh finance/accounting client
- Locking COA dasar sebelum sistem berjalan

---

## 7. Kesiapan Aturan Pajak

Karena sistem akan mencatat pajak, aturan pajak yang digunakan client harus jelas.

### 7.1 Jenis Pajak yang Perlu Dipetakan

Contoh pajak yang mungkin dibutuhkan:

- PPN Masukan
- PPN Keluaran
- Pajak Terutang
- PPh 21
- PPh 23
- PPh 25
- PPh Final
- Pajak dibayar di muka
- Hutang pajak
- Bukti potong
- Faktur pajak
- Jadwal pembayaran pajak

### 7.2 Fitur Pajak yang Bisa Dibangun

Fitur pajak yang dapat disediakan sistem:

- Rekap PPN Masukan
- Rekap PPN Keluaran
- Rekap PPh
- Rekap pajak terutang
- Jadwal jatuh tempo pajak
- Reminder kewajiban pajak
- Status dokumen pajak
- Lampiran bukti pajak
- Warning dokumen pajak belum lengkap

### 7.3 Catatan Penting Pajak

Sistem dapat membantu:

- Pencatatan pajak
- Monitoring pajak
- Reminder pajak
- Rekap pajak
- Validasi awal dokumen pajak

Namun, perhitungan final dan pelaporan resmi tetap harus divalidasi oleh:

- Tim finance/accounting client
- Konsultan pajak client
- Pihak yang berwenang secara internal

---

## 8. Approval Workflow

Approval adalah syarat wajib untuk project ini.

### 8.1 Transaksi yang Perlu Approval

Transaksi yang sebaiknya membutuhkan approval:

- Invoice penjualan
- Invoice pembelian
- Pembayaran hutang
- Penerimaan piutang
- Jurnal manual
- Jurnal koreksi
- Pembayaran pajak
- Transaksi nominal besar
- Perubahan data master penting
- Pembatalan transaksi

### 8.2 Status Approval

Status transaksi:

```text
Draft
Waiting Review
Waiting Approval
Approved
Posted
Rejected
Cancelled
```

### 8.3 Prinsip Approval

- AI hanya membuat draft
- Finance melakukan review
- Owner/manager melakukan approval
- Sistem melakukan posting setelah approval
- Semua aktivitas approval dicatat
- Transaksi yang sudah posted tidak boleh diubah sembarangan
- Koreksi transaksi harus melalui jurnal koreksi atau reversal

---

## 9. Audit Trail

Audit trail wajib ada karena sistem menyimpan data keuangan.

### 9.1 Aktivitas yang Harus Dicatat

Sistem harus mencatat:

- Siapa yang membuat transaksi
- Kapan transaksi dibuat
- Dari channel apa transaksi dibuat
- Input dari user
- Hasil rekomendasi AI
- Data sebelum perubahan
- Data sesudah perubahan
- Siapa yang mengubah
- Siapa yang review
- Siapa yang approve
- Siapa yang posting
- Siapa yang reject
- Alasan reject/revisi
- Waktu setiap aktivitas

### 9.2 Tujuan Audit Trail

Audit trail dibutuhkan untuk:

- Keamanan data
- Transparansi proses
- Pemeriksaan internal
- Mengurangi risiko manipulasi data
- Mengetahui kesalahan input
- Mengetahui aktivitas AI
- Membantu audit perusahaan

---

## 10. Infrastruktur yang Dibutuhkan

### 10.1 Infrastruktur Minimal

Minimal infrastruktur yang dibutuhkan:

- Server/VPS atau server lokal
- PostgreSQL database
- Backend Rust (Axum)
- Frontend Vue.js
- OpenClaw
- Storage dokumen/invoice
- Domain
- SSL/HTTPS
- Backup database
- Monitoring log/error

### 10.2 Pemanfaatan Synology

Jika client memiliki Synology, Synology dapat digunakan untuk:

- Menyimpan file Excel existing
- Menyimpan file invoice
- Menyimpan dokumen pendukung
- Menyimpan backup
- Sumber data awal saat migrasi

Namun untuk aplikasi production, tetap perlu dipertimbangkan:

- Apakah Synology dapat diakses aman dari luar
- Apakah performanya cukup
- Apakah tersedia Docker
- Apakah backup sudah aktif
- Apakah jaringan stabil
- Apakah perlu VPS/cloud untuk akses publik

---

## 11. Budget Operasional Bulanan

Selain biaya development, client perlu menyiapkan biaya berjalan.

### 11.1 Komponen Biaya Bulanan

Contoh biaya operasional:

- VPS/server
- Domain
- SSL jika memakai SSL premium
- WhatsApp Business API/gateway
- OpenAI API atau model AI lain
- OCR API jika menggunakan layanan berbayar
- Backup storage
- Monitoring
- Maintenance
- Support teknis
- Update sistem
- Security patch

### 11.2 Pemisahan Biaya

Proposal sebaiknya memisahkan:

```text
Biaya development sekali bayar
+
Biaya operasional bulanan
+
Biaya maintenance/support
```

---

## 12. Implementasi Bertahap

Project ini cukup besar, sehingga tidak disarankan dibuat sekaligus tanpa tahapan.

### 12.1 Tahap 1 — Discovery & Audit Data

Aktivitas:

- Cek file Excel
- Cek alur kerja accounting
- Cek laporan yang dibutuhkan owner
- Cek pajak yang dipakai
- Mapping COA
- Mapping user dan role
- Mapping approval
- Membuat blueprint sistem

Output:

- Dokumen requirement final
- Data mapping
- Workflow accounting
- Workflow AI
- Estimasi timeline dan biaya final

### 12.2 Tahap 2 — Accounting Core

Aktivitas:

- Master data
- COA
- Jurnal
- Kas/bank
- Hutang
- Piutang
- Invoice
- Pembayaran
- Posting jurnal

Output:

- Sistem accounting core
- Data transaksi dasar
- Posting jurnal otomatis
- Laporan dasar

### 12.3 Tahap 3 — Dashboard Owner

Aktivitas:

- Dashboard kas
- Dashboard hutang
- Dashboard piutang
- Dashboard laba rugi
- Dashboard beban operasional
- Dashboard pajak
- Laporan ringkas

Output:

- Owner bisa melihat kondisi keuangan melalui dashboard

### 12.4 Tahap 4 — AI Assistant

Aktivitas:

- Integrasi OpenClaw
- Telegram/WhatsApp bot
- Intent mapping
- Tool calling ke backend API
- Tanya jawab data keuangan
- Logging percakapan

Output:

- Owner bisa bertanya kondisi keuangan melalui chat

### 12.5 Tahap 5 — AI Draft Automation

Aktivitas:

- AI membuat draft invoice
- AI membuat draft jurnal
- AI membuat draft pembayaran
- AI membuat draft pajak
- Approval wajib sebelum posting

Output:

- AI membantu accounting membuat draft transaksi

### 12.6 Tahap 6 — OCR Invoice

Aktivitas:

- Upload invoice PDF/foto
- Ekstraksi data invoice
- Validasi vendor
- Validasi nominal
- Validasi PPN/PPh
- Deteksi invoice duplikat
- Draft jurnal otomatis

Output:

- Invoice automation berjalan dengan kontrol review

### 12.7 Tahap 7 — Production Hardening

Aktivitas:

- Security hardening
- Backup otomatis
- Monitoring
- Audit trail
- Testing data nyata
- Training user
- SOP penggunaan
- Go-live

Output:

- Sistem siap digunakan secara production

---

## 13. Batasan Scope

Agar project tidak melebar, scope harus jelas sejak awal.

### 13.1 Hal yang Harus Diputuskan

Beberapa hal yang perlu diputuskan:

- Apakah termasuk integrasi WhatsApp resmi?
- Apakah cukup Telegram untuk MVP?
- Apakah OCR invoice masuk tahap awal atau tahap lanjutan?
- Apakah migrasi data lama termasuk scope?
- Apakah integrasi e-Faktur/DJP termasuk?
- Apakah laporan pajak resmi termasuk?
- Apakah mobile app native diperlukan?
- Apakah dashboard web saja cukup?
- Apakah support bulanan termasuk?
- Apakah maintenance setelah go-live termasuk?
- Apakah client membutuhkan multi-company?
- Apakah client membutuhkan multi-branch?
- Apakah ada approval berdasarkan nominal?

### 13.2 Rekomendasi Scope MVP

Untuk MVP, disarankan fokus pada:

- Data master
- COA
- Kas/bank
- Hutang/piutang
- Invoice dasar
- Pembayaran dasar
- Jurnal
- Dashboard owner
- Telegram AI assistant
- Laporan dasar
- Approval dasar

WhatsApp, OCR invoice, dan pajak advanced sebaiknya masuk tahap lanjutan.

---

## 14. SOP Penggunaan

Setelah sistem dibuat, client perlu SOP.

### 14.1 SOP yang Perlu Dibuat

Contoh SOP:

- SOP login dan akses user
- SOP input invoice
- SOP approval transaksi
- SOP posting transaksi
- SOP koreksi jurnal
- SOP pembayaran hutang
- SOP penerimaan piutang
- SOP pencatatan pajak
- SOP membaca laporan
- SOP bertanya ke AI
- SOP upload invoice
- SOP backup data
- SOP closing periode
- SOP penanganan error

### 14.2 Tujuan SOP

SOP diperlukan agar:

- User tidak salah menggunakan sistem
- Proses accounting konsisten
- Approval berjalan rapi
- Data tidak rusak
- AI digunakan dengan benar
- Owner memahami batasan jawaban AI

---

## 15. Testing dengan Data Nyata

Sebelum go-live, sistem wajib diuji dengan data asli.

### 15.1 Skenario Testing

Minimal testing mencakup:

- Invoice penjualan
- Invoice pembelian
- Pembayaran customer
- Pembayaran supplier
- Biaya operasional
- Jurnal manual
- Jurnal koreksi
- Pencatatan PPN
- Pencatatan PPh
- Laporan kas
- Laporan hutang
- Laporan piutang
- Laporan laba rugi
- Laporan neraca
- Laporan arus kas
- Laporan pajak
- Pertanyaan AI via chat
- Approval transaksi
- Reject transaksi
- Revisi transaksi
- Audit trail

### 15.2 Output Testing

Output dari testing:

- Bug list
- Catatan revisi
- Validasi laporan
- Validasi jurnal
- Validasi pajak
- UAT sign-off
- Keputusan go-live

---

## 16. Komitmen Perubahan Cara Kerja

Project ini bukan hanya membuat aplikasi, tetapi juga mengubah cara kerja finance/accounting.

### 16.1 Tantangan Umum

Tantangan yang mungkin muncul:

- User masih terbiasa dengan Excel
- Tim accounting takut sistem baru menyulitkan
- Owner ingin hasil cepat
- Data lama tidak rapi
- Proses approval belum terbiasa
- Beberapa proses belum terdokumentasi

### 16.2 Strategi Transisi

Strategi transisi yang disarankan:

```text
Bulan 1:
Excel tetap berjalan, sistem mulai membaca data

Bulan 2:
Transaksi baru mulai masuk sistem

Bulan 3:
Approval dan laporan mulai dari sistem

Bulan 4:
Excel hanya digunakan sebagai backup/export
```

---

## 17. Risiko dan Mitigasi

### 17.1 Risiko Data Tidak Konsisten

Mitigasi:

- Discovery data
- Data cleaning
- Template import
- Validasi data

### 17.2 Risiko AI Salah Membaca Invoice

Mitigasi:

- AI hanya membuat draft
- Finance wajib review
- Confidence score ditampilkan
- Approval sebelum posting

### 17.3 Risiko Kesalahan Jurnal

Mitigasi:

- Validasi debit/kredit
- Mapping COA
- Review jurnal
- Audit trail
- Reversal untuk koreksi

### 17.4 Risiko Pajak Salah Hitung

Mitigasi:

- Rule pajak configurable
- Validasi oleh finance/accounting
- Validasi oleh konsultan pajak
- Sistem hanya membantu pencatatan dan monitoring

### 17.5 Risiko Akses Data Sensitif

Mitigasi:

- Role-based access
- Whitelist user
- HTTPS
- Backup
- Audit log
- Pembatasan akses dokumen

### 17.6 Risiko Scope Melebar

Mitigasi:

- Scope tertulis
- Change request
- Prioritas MVP
- Milestone bertahap

---

## 18. Rekomendasi Tahap Awal yang Dijual ke Client

Sebelum menawarkan full development, disarankan menjual tahap awal:

## Discovery & Blueprint AI Finance Accounting System

Isi tahap awal:

- Audit file Excel
- Mapping proses accounting
- Mapping laporan owner
- Mapping pajak
- Desain COA
- Desain database awal
- Desain workflow AI
- Desain approval
- Estimasi final biaya dan timeline

### Manfaat Tahap Discovery

Tahap ini membantu:

- Menghindari salah scope
- Menghindari salah desain sistem
- Mengetahui kualitas data
- Mengetahui kompleksitas accounting
- Menghasilkan proposal final yang lebih akurat
- Membuat client lebih percaya
- Mengurangi risiko project gagal

---

## 19. Checklist Kesiapan Client

Gunakan checklist berikut sebelum project dimulai.

### 19.1 Data

- [ ] File kas/bank tersedia
- [ ] File hutang tersedia
- [ ] File piutang tersedia
- [ ] File penjualan tersedia
- [ ] File pembelian tersedia
- [ ] File biaya operasional tersedia
- [ ] Data customer tersedia
- [ ] Data supplier tersedia
- [ ] COA tersedia
- [ ] Data pajak tersedia
- [ ] Contoh invoice tersedia
- [ ] Contoh laporan tersedia

### 19.2 Proses Bisnis

- [ ] Proses penjualan jelas
- [ ] Proses pembelian jelas
- [ ] Proses pembayaran jelas
- [ ] Proses hutang/piutang jelas
- [ ] Proses jurnal jelas
- [ ] Proses pajak jelas
- [ ] Proses approval jelas
- [ ] Proses koreksi transaksi jelas

### 19.3 PIC

- [ ] Owner / direktur tersedia
- [ ] Finance/accounting PIC tersedia
- [ ] Admin data tersedia
- [ ] PIC IT/internal tersedia jika diperlukan

### 19.4 Teknologi

- [ ] Akses Synology tersedia
- [ ] Server/VPS disepakati
- [ ] Domain disiapkan
- [ ] SSL/HTTPS disiapkan
- [ ] WhatsApp/Telegram channel disepakati
- [ ] Backup disiapkan

### 19.5 Budget

- [ ] Budget development disepakati
- [ ] Budget operasional bulanan disiapkan
- [ ] Budget maintenance/support disepakati
- [ ] Biaya API AI/OCR/WhatsApp dipahami

### 19.6 Go-Live

- [ ] Testing dengan data nyata selesai
- [ ] User training selesai
- [ ] SOP tersedia
- [ ] Backup aktif
- [ ] PIC support tersedia
- [ ] UAT sign-off selesai

---

## 20. Kesimpulan

Agar **AI Finance & Accounting Assistant / AI Accounting Automation Assistant** benar-benar bisa diwujudkan, client perlu siap dari sisi:

- Data
- PIC
- Proses accounting
- COA
- Pajak
- Approval
- Audit trail
- Infrastruktur
- Budget
- SOP
- Testing
- Komitmen perubahan cara kerja

Rekomendasi terbaik adalah memulai dari tahap:

```text
Discovery & Blueprint
      ↓
Accounting Core
      ↓
Dashboard Owner
      ↓
AI Assistant
      ↓
AI Draft Automation
      ↓
OCR Invoice
      ↓
Production Hardening
```

Dengan pendekatan bertahap, project menjadi lebih aman, realistis, dan memiliki peluang lebih besar untuk berhasil digunakan dalam operasional perusahaan.
