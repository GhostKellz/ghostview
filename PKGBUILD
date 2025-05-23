# Maintainer: Your Name <your@email.com>

pkgname=ghostview
pkgver=0.1.0
pkgrel=1
pkgdesc="A sleek, native GUI for Arch package management, built with Rust and Slint."
arch=('x86_64')
url="https://github.com/ghostkellz/ghostview"
license=('MIT')
depends=('rust' 'slint' 'gtk3')
makedepends=('git' 'cargo')
source=("$pkgname::git+https://github.com/ghostkellz/ghostview.git")
md5sums=('SKIP')

pkgver() {
  cd "$srcdir/$pkgname"
  git describe --tags | sed 's/^v//;s/-/./g'
}

build() {
  cd "$srcdir/$pkgname"
  cargo build --release
}

package() {
  cd "$srcdir/$pkgname"
  install -Dm755 target/release/ghostview "$pkgdir/usr/bin/ghostview"
  install -Dm644 README.md "$pkgdir/usr/share/doc/ghostview/README.md"
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/ghostview/LICENSE"
}
