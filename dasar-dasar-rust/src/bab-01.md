# Berkenalan dengan Rust

Bab ini membahas tentang pengetahuan dasar untuk mulai berkenalan dengan Rust.
Pembahasan pada bab ini diantaranya adalah :

1. Apa itu Rust?
2. Instalasi Rust dan paket pendukung lainnya
3. Instalasi IDE dan Text Editor
4. Command

## Apa itu Rust?
Rust adalah bahasa pemrograman _low-level_ yang mulai dikembangkan sejak tahun 2006 oleh Graydon Hoare salah seorang engineer di Mozilla. Rust adalah bahasa pemrograman sistem (_system programming_) yang difokuskan pada tiga tujuan: keamanan, kecepatan, dan concurrency. Rust memelihara tujuan ini tanpa garbage collector yang membuatnya menjadi bahasa yang berguna untuk sejumlah kasus penggunaan bahasa lain. Tidak hanya itu rust juga bahasa dengan general purpose language, bahasa low-level yang bisa membantu kita untuk mengeksplorasi potensi sisi system, embedded system, dan hal-hal kritis terkait performance.

## Instalasi Rust dan Paket Pendukung Lainnya
### Instalasi Rust
Untuk menginstal rust kita akan menggunakan [rustup](https://www.rustup.rs/), namun sebelum itu kita perlu mengetahui apa itu rustup.
Rustup adalah official toolchain manajemen dari [The Rust Programming Language](https://www.rust-lang.org/). Dengan rustup kita dapat dengan mudah : menginstal, mengupdate, mengganti versi compiler (stable, beta, dan nightly) dan mempermudah kita dalam mengkompilasi kode sumber berikut dengan standard library agar aplikasi kita dapat dijalankan platform yang didukung oleh Rust. Untuk menginstal rust sangat mudah sekali, kita hanya perlu menjalankan :
```
curl https://sh.rustup.rs -sSf | sh
```
Jika perintah diatas dijalankan rustup akan memberikan pilihan apakah kita akan menginstal rust secara custom atau default.
```
info: downloading installer

Welcome to Rust!

This will download and install the official compiler for the Rust programming
language, and its package manager, Cargo.

It will add the cargo, rustc, rustup and other commands to Cargo's bin

directory, located at:

  /home/<user>/.cargo/bin

This path will then be added to your PATH environment variable by modifying the
profile file located at:

  /home/<user>/.profile

You can uninstall at any time with rustup self uninstall and these changes will
be reverted.

Current installation options:

   default host triple: x86_64-unknown-linux-gnu
     default toolchain: stable
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
```
Pada saat kita menginstal Rust maka kita akan mendapatkan tiga buah tool yaitu : cargo, rustc dan rustup.
1. **cargo** adalah package manager untuk Rust.
2. **rustc** adalah compiler yang digunakan untuk mengkompilasikode sumber ke static binary.
3. **rustup** adalah toolchain manajemen yang sudah kita bahas diatas.

### Instalasi paket pendukung
Pada tahap instalasi diatas sebenarnya kita sudah dapat bermain dengan Rust, namun, ada "the most useful tools" yang sangat berguna bagi kita yaitu : racer, rustfmt, rustsym.
* **racer**

  Tool ini berfungsi untuk auto-suggestion pada saat kita menuliskan keyword, library, dan lainnya yang hendak kita pakai. Untuk menginstal racer jalankan perintah :
  ```
  cargo install racer
  ```
  Contoh penggunaan racer :
  ```
  racer complete std::io::R

  // Output
  MATCH Read,427,10,/home/<user>/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/io/mod.rs,Trait,pub trait Read
  MATCH Result,48,9,/home/<user>/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/io/error.rs,Type,pub type Result<T> = result::Result<T, Error>;
  MATCH Repeat,107,11,/home/<user>/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/io/util.rs,Struct,pub struct Repeat
  ```
* **rustfmt**

  Tool ini berfungsi untuk memformat penulisan kode sumber yang kita buat
  ```
  cargo install rustfmt
  ```
* **rustsym**

  Tool ini berfungsi untuk meng-query atau mencari karakter didalam kode sumber yang kita buat
  ```
  cargo install rustsym
  ```

## Instalasi Text Editor
* **Arch** `sudo pacman -S atom` atau `yaourt atom-editor-git`
* **Debian/Ubuntu** Download file binari pada [Website Atom](https://atom.io/) lalu jalankan `sudo dpkg -i atom-amd64.deb`
Setelah instalasi text editor selesai kita perlu mengintegrasikan tool tambahan yang sudah kita install tadi seperti : racer, rustfmt dan rustsym ke text editor, untuk menginstalnya cukup jalankan perintah :
  ```
  apm install racer
  apm install rustfmt
  apm install rustsym
  ```
  Jika proses instalasi selesai kita juga perlu setup racer agar dapat berjalan di text editor kita, buka atom text editor lalu masuk ke **Preferences** atau **CTRL+, > Packages** klik package racer lalu kita isikan PATH Variable yang dibutuhkan racer contoh :

| Display Name                             | Description                                                                                  |
|:-----------------------------------------|:---------------------------------------------------------------------------------------------|
| Path to the Racer executable             | `/home/<user>/.cargo/bin/racer`                                                              |
| Path to the Rust source code directory   | `/home/<user>/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src`   |
| Cargo home directory                     | `/home/<user>/.cargo/`                                                                       |

  > Note: Ubah `<user>` dengan nama user pada komputer anda.

## Command
Pada sub bab ini kita akan membahas tentang penggunaan _command line interface_ dari tool : rustc, rustup dan cargo yang berguna pada saat kita membangun aplikasi.
### Rust Compiler (rustc)
Rust Compiler (`rustc`) adalah tool yang digunakan untuk mengkompilasikode sumber kedalam _static-binary_ karen Rust adalah bahasa pemrogramman `ahead-of-time compiled` yang artinya kita mengkompilasiaplikasi kita lalu kita dapat mendistribusikan aplikasi kita dan langsung dapat di-running tanpa harus menginstal Rust terlebih dahulu, tidak seperti bahasa lainnya misalnya `.rb`, `.py` atau `.js` yang mana kita harus menginstal _ruby_, _pyton_, atau _javascript_ terlebih dahulu sebelum me-running file tersebut.
Untuk mengkompilasicaranya cukup mudah hanya dengan `rustc <nama-file>.rs`. Contohnya terdapat program sederhana yang menampilkan kalimat "Hello!, Aku <3 Rust." dengan nama file `hello.rs` seperti dibawah ini :
```rust
fn main() {
    println!("Hello!, Aku <3 Rust.");
}
```
maka untuk mengkompilasi file tersebut :
```
rustc hello.rs
```
dan untuk menjalankan nya :
```
./hello

// Output
Hello!, Aku <3 Rust.
```
bagaimana jika ingin mengganti nama file hasil kompilasi? mudah saja, kita tambahan options `-o` diikuti dengan nama file yang kita inginkan lalu nama file kode sumber, contoh :
```
rustc -o program_pertama hello.rs
```
ketika menjalankan kembali hasil kompilasi hasilnya akan sama saja.

### Rustup
Rustup digunakan untuk berhubungan dengan toolchain dan kita perlu mengetahui standard penamaan rilis channel toolchain :
```
<channel>[-<date>][-<host>]
<channel>       = stable|beta|nightly|<version>
<date>          = YYYY-MM-DD
<host>          = <target-triple>
```
Untuk menggunakan rustup sangat mudah format penulisan nya adalah `rustup [FLAGS] [SUBCOMMAND]`, berikut daftar subcommand yang ada :
* **Show** digunakan untuk melihat daftar toolchain yang terinstall dan aktif, contoh :

  ```
  rustup show

  // Output
  Default host: x86_64-unknown-linux-gnu
  installed toolchains
  --------------------
  stable-x86_64-unknown-linux-gnu (default)
  nightly-x86_64-unknown-linux-gnu

  active toolchain
  ----------------
  stable-x86_64-unknown-linux-gnu (default)
  rustc 1.15.1 (021bd294c 2017-02-08)
  ```
* **Update** digunakan untuk mengupdate toolchain, contoh :

  ```
  rustup update

  // Output
  info: syncing channel updates for 'stable-x86_64-unknown-linux-gnu'
  info: downloading component 'rustc'
   36.4 MiB /  36.4 MiB (100 %) 291.2 KiB/s ETA:   0 s
  info: downloading component 'rust-std'
   50.0 MiB /  50.0 MiB (100 %) 217.6 KiB/s ETA:   0 s
  info: downloading component 'cargo'
    4.7 MiB /   4.7 MiB (100 %) 288.0 KiB/s ETA:   0 s
  info: downloading component 'rust-src'
   27.4 MiB /  27.4 MiB (100 %) 233.6 KiB/s ETA:   0 s
  info: installing component 'rustc'
  info: installing component 'rust-std'
  info: installing component 'cargo'
  info: installing component 'rust-src'
  info: syncing channel updates for 'nightly-x86_64-unknown-linux-gnu'
  info: checking for self-updates

  stable-x86_64-unknown-linux-gnu updated - rustc 1.16.0 (30cf806ef 2017-03-10)
  nightly-x86_64-unknown-linux-gnu unchanged - rustc 1.17.0-nightly (0aeb9c129 2017-03-15)
  ```
  atau bisa juga mengupdate per toolchain, tambahkan saja nama toolchain nya
  ```
  rustup update nightly

  // Output
  info: syncing channel updates for 'nightly-x86_64-unknown-linux-gnu'
   282.9 KiB / 282.9 KiB (100 %) 121.6 KiB/s ETA:   0 s
  info: downloading component 'rustc'
    41.0 MiB /  41.0 MiB (100 %) 211.2 KiB/s ETA:   0 s
  info: downloading component 'rust-std'
    65.5 MiB /  65.5 MiB (100 %) 188.8 KiB/s ETA:   0 s
  info: downloading component 'cargo'
     8.6 MiB /   8.6 MiB (100 %) 252.3 KiB/s ETA:   0 s
  info: installing component 'rustc'
  info: installing component 'rust-std'
  info: installing component 'cargo'

    nightly-x86_64-unknown-linux-gnu updated - rustc 1.17.0-nightly (0aeb9c129 2017-03-15)
  ```
* **Default** digunakan untuk mengubah default toolchain yang kita gunakan seperti merubah channel toolchain dari `stable` ke `nightly`, contoh :

  ```
  rustup default nightly

  // Output
  info: using existing install for 'nightly-x86_64-unknown-linux-gnu'
  info: default toolchain set to 'nightly-x86_64-unknown-linux-gnu'

    nightly-x86_64-unknown-linux-gnu unchanged - rustc 1.17.0-nightly (b1e31766d 2017-03-03)
  ```
* **Toolchain** digunakan untuk mencari dan menampilkan daftar toolchain yang terinstall.
  * Menampilkan daftar toolchain

    ```
    rustup toolchain list

    // Output
    stable-x86_64-unknown-linux-gnu
    nightly-x86_64-unknown-linux-gnu (default)
    ```
  * Install toolchain

    ```
    rustup toolchain install beta

    // Output
    info: syncing channel updates for 'beta-x86_64-unknown-linux-gnu'
    156.4 KiB / 156.4 KiB (100 %) 129.9 KiB/s ETA:   0 s
    info: downloading component 'rustc'
     37.1 MiB /  37.1 MiB (100 %) 224.0 KiB/s ETA:   0 s
    info: downloading component 'rust-std'
     61.6 MiB /  61.6 MiB (100 %) 220.8 KiB/s ETA:   0 s
    info: downloading component 'cargo'
      8.6 MiB /   8.6 MiB (100 %) 281.6 KiB/s ETA:   0 s
    info: installing component 'rustc'
    info: installing component 'rust-std'
    info: installing component 'cargo'

      beta-x86_64-unknown-linux-gnu installed - rustc 1.17.0-beta.1 (408d49e60 2017-03-14)
    ```
  * Uninstall toolchain

    ```
    rustup toolchain uninstall beta

    // Output
    info: uninstalling toolchain 'beta-x86_64-unknown-linux-gnu'
    info: toolchain 'beta-x86_64-unknown-linux-gnu' uninstalled
    ```
  * Link (membuat custom toolchain)
    ```
    rustup toolchain link custom-toolchain /path/to/my/toolchain/sysroot
    ```
* **Target** digunakan untuk kompilasi ke platform yang dinginkan karena Rust mendukung kompilasi untuk berbagai platform (_cross-compilation_)
  * Menambahkan target
    ```
    rustup target add arm-linux-androideabi

    // Output
    info: downloading component 'rust-std' for 'arm-linux-androideabi'
     16.0 MiB /  16.0 MiB (100 %) 172.8 KiB/s ETA:   0 s
    info: installing component 'rust-std' for 'arm-linux-androideabi'
    ```
    dengan fitur target ini kita dapat membuild aplikasi untuk android dengan cargo dan menambahkan flag `--target`. Penggunaan Cargo akan kita bahas pada subbab berikutnya.
  * Melihat daftar target yang terinstall
    ```
    rustup target list

    // Output
    aarch64-apple-ios
    aarch64-linux-android
    aarch64-unknown-fuchsia
    aarch64-unknown-linux-gnu
    arm-linux-androideabi (installed)
    ...
    ```
  * Menghapus (remove) target yang terinstall caranya `rustup target remove <TARGET>`
    ```
    rustup target remove arm-linux-androideabi
    ```
* **Component**
  * Menambahkan component, untuk menambahkan caranya `rustup component add <COMPONENT>` contoh :
  ```
  rustup component add rust-src
  ```
  * Melihat daftar component

    ```
    rustup component list

    // Output
    cargo-x86_64-unknown-linux-gnu (default)
    rust-docs-x86_64-unknown-linux-gnu
    rust-src (installed)
    rust-std-aarch64-apple-ios
    rust-std-aarch64-linux-android
    ...
    ```
  * Menghapus component, untuk menghapus caranya `rustup component remove <COMPONENT>` contoh :
    ```
    rustup component remove rust-src
    ```
* **Override** digunakan untuk memberitahu compiler agar memakai toolchain tertentu pada direktori proyek secara spesifik.
  * Melihat direktori yang telah di override
  ```
  rustup override list
  ```
  * Menentukan (setup) toolchain dengan perintah `rustup override set <DIRECTORY>`
  ```
  rustup override set ~/Code/aplikasi-ku
  ```
  * Menghapus override (unset) toolchain
  ```
  rustup override unset
  ```
* **Run** digunakan untuk menjalankan toolchain yang tersedia
  * Penggunaan bersamaan dengan compiler.
    ```
    rustup run nightly rustc ~/Desktop/hello.rs
    ```
  * Penggunaan bersamaan dengan shell.
    ```
    rustup run nightly bash
    ```
* **Which** digunakan untuk menampilkan lokasi aplikasi binari yang sedang dijalankan
  ```
  rustup which rustc

  // Output
  /home/<user>/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc
  ```
* **Doc** digunakan untuk menampilkan dokumentasi Rust
  * Buku
  ```
  rustup doc --book
  ```
  * Standard Library
  ```
  rustup doc --std
  ```
  ketika perintah diatas kita jalankan maka secara otomatis dokumentasi akan ditampilkan dibrowser.
  > NOTE: Agar kita dapat menggunakan fitur ini, component rust-docs harus sudah terinstall, untuk menginstalnya jalankan perintah `rustup component add rust-docs`.

* **Self** digunakan untuk memodifikasi rustup seperti mengupdate dan menguninstall
  * Update rustup
  ```
  rustup self update
  ```
  * Uninstall rustup
  ```
  rustup self uninstall
  ```
* **Set** digunakan untuk melakukan pengaturan (settings) pada rustup seperti merubah default host, tapi fitur ini sangat jarang digunakan karena pada saat instalasi rustup, secara otomatis rustup sudah mengatur keperluan ini.
  ```
  rustup set default-host
  ```
* **Completion** fitur ini digunakan untuk menambahkan penyempurnaan CLI (_command line interface_) yang sangat berguna sekali pada saat bermain dengan rustup agar lebih cepat penilaiannya.
  * Bash
  ```
  # rustup completions bash > /etc/bash_completion.d/rustup.bash-completion
  ```
  * Zsh
  ```
  $ rustup completions zsh > ~/.zfunc/_rustup
  ```
  Lalu tambahkan kode dibawah ini didalam file `zshrc` :
  ```
  fpath+=~/.zfunc
  ```
  * Fish
  ```
  $ rustup completions fish > ~/.config/fish/completions/rustup.fish
  ```

### Cargo
Cargo adalah _package manager_ atau tool yang memudahkan kita dalam menambahkan dan memanajemen paket-paket yang menjadi ketergantuangan (_depencies_). Untuk itu cargo melakukan empat hal diantaranya :
1. Membuat metadata dengan nama file `Cargo.toml`
2. Menambahkan dependency yang dibutuhkan
3. Menjalan `rustc` untuk keperluan kompilasi
4. Memperkenalkan konvensi untuk membuat bekerja dengan proyek-proyek Rust mudah.

* **New** digunakan untuk membuat proyek baru
  ```
  cargo new nama_proyek --bin
  ```
  atau jika kita ingin membuat proyek librari
  ```
  cargo new nama_proyek
  ```
  jika kita jalankan perintah diatas, maka cargo akan membuatkan direktori dan file-file yang kita butuhkan seperti contoh dibawah ini :
  ```
  nama_proyek
  .
  ├── .git
  │   ├── HEAD
  │   ├── config
  │   ├── description
  │   ├── hooks
  │   ├── info
  │   ├── objects
  │   └── refs
  └── src
  │   └── main.rs
  ├── .gitignore
  └── Cargo.toml

  6 directories, 6 files
  ```
  1. direktori proyek yang sesuai dengan nama proyek yang kita sebutkan diatas berikut dengan inisialisasi `git`
  2. file `Cargo.toml` sebagai manifest proyek
  3. direktori `src` sebagai direktori utama kode sumber
  4. file `main.rs` atau `lib.rs` jika kita hendak membuat librari
* **Init** digunakan untuk menginisialisasi proyek yang sudah dibuat namun belum terintegrasi dengan cargo
  ```
  cargo init
  ```
  atau jika binari proyek
  ```
  cargo init --bin
  ```
* **Build** digunakan untuk mengkompilasi kode sumber kedalam bentuk file binari
  ```
  cargo build
  ```
  hasil kompilasi tersebut akan ditempatkan didalam direktori `target`
* **Run** sama halnya `build` pada perintah `run` sebenarnya cargo terlebih dahulu mengkompilasi namun langsung menjalankan hasil kompilasi tersebut
  ```
  cargo run
  ```
* **Clean** digunakan untuk membersihkan direktori/file yang dihasilkan pada saat `build` yakni direktori `target`
  ```
  cargo clean
  ```
* **Test** ketika bekerja dengan proyek yang cukup besar pastinya kita akan membutuhkan testing, hal ini ditangani oleh cargo maka kita dapat dengan mudah membuat test untuk proyek kita, berikut contoh dari unit testing :
  ```
  cargo test

  // Output
  Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
   Running target/debug/deps/<nama_proyek>-46a021ef15e0c20f

  running 11 tests
  test containers ... ok
  test ping ... ok
  test container ... ok
  test swarm ... ok
  test nodes ... ok
  test services ... ok
  test networks ... ok
  test images ... ok
  test tasks ... ok
  test volumes ... ok
  test info ... ok

  test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured

       Running target/debug/deps/<nama_proyek>-8a1cf4acd969d628

  running 0 tests

  test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured

     Doc-tests <nama_proyek>

  running 0 tests

  test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
  ```
* **Bench** digunakan untuk keperluan _benchmarking_, berikut contoh benchmarck :
  ```
  Finished release [optimized] target(s) in 29.70 secs
       Running target/release/deps/<nama_proyek>-53cdfcee641cf7b8

  running 2 tests
  test tests::it_works ... ignored
  test tests::bench_add_two ... bench:           0 ns/iter (+/- 0)

  test result: ok. 0 passed; 0 failed; 1 ignored; 1 measured
  ```
* **Install** digunakan untuk menginstal atau menambahkan paket seperti `racer`, `rustfmt` dan `rustsym` yang sudah kita gunakan pada sub bab sebelumnya
  ```
  cargo install <nama_paket>
  ```
  paket yang terinstall akan ditempat kedalam direktori `/home/<user>/.cargo/bin`
* **Update** digunakan untuk pemutakhiran semua paket depencies pada proyek kita
  ```
  cargo update
  ```
  atau dapat juga mengupdate perpaket
  ```
  cargo update -p <nama_paket>
  ```
* **Search** digunakan untuk mencari paket-paket yang terdapat di registry [crates.io](https://crates.io)
  ```
  cargo search reg

  // Output
    Updating registry `https://github.com/rust-lang/crates.io-index`
  winreg = "0.4.0"         # Rust bindings to MS Windows Registry API
  cpuprofiler = "0.0.3"    # Bindings to google's cpu profiler
  ecp = "0.1.1"            # easily build compiler plugins.
  ```
* **Publish** digunakan untuk mempublikasikan paket yang kita buat ke registry
  ```
  cargo publish
  ```
* **Doc** digunakan untuk men-generate dokumentasi paket yang kita buat
  ```
  cargo docs
  ```
  hasil dokumentasi otomatis akan dibuat didalam direktori `target/doc`
