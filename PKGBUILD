# Maintainer: Akarsh Jain <akarsh.1995.02@gmail.com>

pkgname=interception-caps2esc-hjkl-arrow
pkgver=1.0.0
pkgrel=1
pkgdesc='Interception plugin caps2esc-hjkl-arrow: Capslock tap for escape key, Capslock hold + hjkl for arrow keys.'
arch=('x86_64')
license=('MIT')
url='https://github.com/akarsh1995/interception-caps2esc-hjkl-arrow'
depends=('interception-tools')
makedepends=('git' 'cmake')
source=(https://github.com/akarsh1995/interception-caps2esc-hjkl-arrow)
md5sums=('SKIP')

build() {
	cmake -S ${pkgname} \
    -B build \
		-DCMAKE_INSTALL_PREFIX=/usr \
		-DCMAKE_BUILD_TYPE=Release \
		-Wno-dev
	cmake --build build
}

package() {
    DESTDIR="$pkgdir" cmake --install build
    install -Dm644 caps2esc-hjkl-arrow.yaml "${pkgdir}/etc/interception/udevmon.d/caps2esc-hjkl-arrow.yaml"
}
