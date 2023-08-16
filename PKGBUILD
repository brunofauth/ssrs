# Maintainer: Bruno Fauth <bvfauth at hotmail dot com>

_name=ssrs
pkgname="python-$_name"
pkgver='0.1.3'
pkgrel=1
pkgdesc="'Simple Scripting for Rust' enables you to run rust source files as scripts, from the command line, by prefixing them with a shebang line. It's useful because sometimes you just want to write a quick-and-dirty 'script', without having to creaate a new crate/package."

arch=('any')
url="https://github.com/brunofauth/${_name}"
license=("MIT")

depends=('python')
makedepends=(python-build python-installer python-wheel)

source=("https://files.pythonhosted.org/packages/source/${_name::1}/$_name/$_name-$pkgver.tar.gz")
sha256sums=('6aadf0ed5eaf96d822a9f4467f350dae45c911eca398ef8b69be41d8cf8489a4')

build() {
    cd "$_name-$pkgver"
    python -m build --wheel --no-isolation
}

package() {
    cd "$_name-$pkgver"
    python -m installer --destdir="$pkgdir" dist/*.whl
}

