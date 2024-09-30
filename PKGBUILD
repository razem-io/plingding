# Maintainer: Julian Pieles <plingding@pieles.digital>
pkgname=plingding
pkgver=0.2.0
pkgrel=1
pkgdesc="A Rust application that sends push notifications via multiple providers (e.g., Pushover, ntfy). It can be used from the command line to send messages with optional priority and image attachments."
arch=('x86_64')
url="https://github.com/razem-io/plingding"
license=('MIT')
depends=('gcc-libs')
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::$url/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')

prepare() {
  cd "$pkgname-$pkgver"
  cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
  cd "$pkgname-$pkgver"
  export RUSTUP_TOOLCHAIN=stable
  export CARGO_TARGET_DIR=target
  cargo build --frozen --release --all-features
}

check() {
  cd "$pkgname-$pkgver"
  export RUSTUP_TOOLCHAIN=stable
  cargo test --frozen --all-features
}

package() {
  cd "$pkgname-$pkgver"
  strip "target/release/$pkgname"
  install -Dm0755 -t "$pkgdir/usr/bin/" "target/release/$pkgname"
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
  install -Dm644 plingding.yaml.example "$pkgdir/usr/share/plingding/plingding.yaml.example"
}
