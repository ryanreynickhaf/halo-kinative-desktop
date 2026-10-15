# Halo Kinative (contoh aplikasi desktop, T-62)

Aplikasi Tauri v2 paling kecil: satu jendela, satu kolom nama, satu tombol. Panduan lengkap: `sop/aplikasi-desktop.md`.

**Status: belum pernah di-build.** Berkas ditulis di server Linux kantor tanpa Rust/Tauri; yang pertama kali ngebuktiin
jalan atau enggak adalah run GitHub Actions pertama. Kalau gagal, baca lognya, benerin, ulang.

## Isi
- `src/index.html`: tampilan (HTML polos, tanpa build step)
- `src-tauri/`: konfigurasi & inti Rust (hampir kosong)
- `app-icon.svg`: sumber ikon; workflow bikin ukuran-ukurannya lewat `npx tauri icon`
- `.github/workflows/build.yml`: build Windows + macOS (Apple Silicon & Intel), hasil jadi artifact

## Cara pakai (repo uji, bukan ruang-sunyi-pos)
1. Owner bikin repo GitHub publik baru, upload isi folder ini ke akar repo (langkah rinci di task T-62).
2. Tab **Actions** → **build-desktop** → **Run workflow**.
3. Selesai ±10–20 menit → buka run-nya → bagian **Artifacts** → unduh per OS.

Tanpa tanda tangan: Windows nampilin SmartScreen, macOS nampilin peringatan Gatekeeper (lihat SOP bagian 3).
