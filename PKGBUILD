# Maintainer: Jan Fidra <tkmxqrd@gmail.com>

pkgname=logviewer-systemd
pkgver=1.1.0
pkgrel=1
pkgdesc="A simple log viewer and filter for journalctl"
arch=('x86_64')
url="https://github.com/tkmxqrdxddd/logviewer-systemd"
license=('GPL3')
depends=('gcc-libs')
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/tkmxqrdxddd/logviewer-systemd/archive/v$pkgver.tar.gz")
sha256sums=('7b6de27f64e55d206f412dbdac2fe9168fb5368e29b1a4ce82af936830f4a689')

build() {
    cd "$pkgname-$pkgver"
    cargo build --release
}

package() {
    cd "$pkgname-$pkgver"
    # Find the binary name from Cargo.toml
    local binary_name=$(grep -oP '(?<=name = ")[^"]*' Cargo.toml)
    install -Dm755 "target/release/$binary_name" "$pkgdir/usr/bin/$pkgname"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
