# Maintainer: Barbel <barbel@barbel.org>
pkgname=oracle
pkgver=0.1.1
pkgrel=1
pkgdesc="A tool to manage environment variables"
arch=('x86_64')
url="https://github.com/barbeldotorg/oracle"
license=('AGPL-3')
depends=(
    'gcc-libs'
    'glibc'
    'wayland'
    'libxkbcommon'
)
makedepends=(
    'cargo'
    'git'
)
options=('!lto')
source=("git+$url.git#tag=v$pkgver")
sha256sums=('SKIP')

prepare() {
    cd "$pkgname"
    cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
    cd "$pkgname"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --release --locked --frozen
}

package() {
    cd "$pkgname"

    # Binary and assets
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
    install -Dm644 "$pkgname.desktop" "$pkgdir/usr/share/applications/$pkgname.desktop"
    install -Dm644 "$pkgname.svg" "$pkgdir/usr/share/icons/hicolor/scalable/apps/$pkgname.svg"

    # Shell completions
    local outdir
    outdir=$(find target/release/build -maxdepth 1 -name "$pkgname-*" -type d \
        -exec test -e '{}/out/oracle.bash' \; -print -quit)

    if [ -n "$outdir" ]; then
        install -Dm644 "$outdir/out/$pkgname.bash" \
            "$pkgdir/usr/share/bash-completion/completions/$pkgname"
        install -Dm644 "$outdir/out/_$pkgname" \
            "$pkgdir/usr/share/zsh/site-functions/_$pkgname"
        install -Dm644 "$outdir/out/$pkgname.fish" \
            "$pkgdir/usr/share/fish/vendor_completions.d/$pkgname.fish"
    else
        warning "Could not locate generated shell completions in OUT_DIR"
    fi
}