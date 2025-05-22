# Maintainer: Your Name <your@email.com>

pkgname=ghostview
gitname=ghostview
gitrepo="https://github.com/ghostkellz/ghostview.git"
pkgver=0.1.0
pkgrel=1
pkgdesc="A beautiful, modular GUI for discovering and managing packages across the Arch Linux ecosystem and beyond."
arch=('x86_64')
url="https://github.com/ghostkellz/ghostview"
license=('MIT')
depends=('tauri' 'nodejs' 'npm' 'rust' 'ghostbrew')
makedepends=('git' 'npm' 'rust' 'cargo')
source=("$gitname::git+$gitrepo")
md5sums=('SKIP')

pkgver() {
  cd "$srcdir/$gitname"
  git describe --tags | sed 's/^v//;s/-/./g'
}

build() {
  cd "$srcdir/$gitname"
  npm install
  npm run build
  cd src-tauri
  cargo build --release
}

package() {
  cd "$srcdir/$gitname"
  install -Dm755 src-tauri/target/release/ghostview-tauri "$pkgdir/usr/bin/ghostview"
  install -Dm644 README.md "$pkgdir/usr/share/doc/ghostview/README.md"
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/ghostview/LICENSE"
}
