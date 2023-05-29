# Maintainer: Bruno Fauth <bvfauth@hotmail.com>

_name=ssrs
pkgname="python-$_name"
pkgver='0.1.2'
pkgrel=1
pkgdesc="'Simple Scripting for Rust' enables you to run rust source files as scripts, from the command line, by prefixing them with a shebang line. It's useful because sometimes you just want to write a quick-and-dirty 'script', without having to creaate a new crate/package."

arch=('any')
url="https://github.com/brunofauth/ssrs"
license=("MIT")

depends=('python')
makedepends=(python-build python-installer python-wheel)

source=("https://files.pythonhosted.org/packages/source/${_name::1}/$_name/$_name-$pkgver.tar.gz")
sha256sums=('c3c84c5b98b76618a09fa47c55d29dbf100900e62c111e11cc10bc9e0950959b')

build() {
    cd "$_name-$pkgver"
    python -m build --wheel --no-isolation
}

package() {
    cd "$_name-$pkgver"
    python -m installer --destdir="$pkgdir" dist/*.whl
}

