# Maintainer: Jan Fidra <tkmxqrd@gmail.com>

pkgname=logviewer
pkgver=1.0.0
pkgrel=1
pkgdesc="A simple log viewer and filter for journalctl"
arch=('x86_64')
url="https://github.com/tkmxqrdxddd/logviewer-systemd"
license=('GPL3')
depends=('gcc-libs')
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/tkmxqrdxddd/logviewer-systemd/archive/v$pkgver.tar.gz")
sha256sums=('44d78a36c8c6ae0ef8df926498d405c10e2e06e24a0f84539a64de4b8a7ad9c8')

build() {
    cd "$pkgname-$pkgver"
    cargo build --release
}

package() {
    cd "$pkgname-$pkgver"
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}