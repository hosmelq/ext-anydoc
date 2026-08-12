PHP_ARG_ENABLE([anydoc], [whether to enable anydoc],
  [AS_HELP_STRING([--enable-anydoc], [Enable the anydoc extension (Rust via ext-php-rs)])],
  [no])

if test "$PHP_ANYDOC" != "no"; then
  AC_PATH_PROG([CARGO], [cargo], [])
  if test -z "$CARGO"; then
    AC_MSG_ERROR([cargo not found. Install a Rust toolchain containing cargo.])
  fi

  AC_PATH_PROG([RUSTC], [rustc], [])
  if test -z "$RUSTC"; then
    AC_MSG_ERROR([rustc not found. Install a Rust toolchain containing rustc.])
  fi

  PHP_CONFIG_PATH=`command -v "$PHP_CONFIG" 2>/dev/null`
  if test -z "$PHP_CONFIG_PATH"; then
    AC_MSG_ERROR([php-config not found: $PHP_CONFIG])
  fi
  PHP_CONFIG="$PHP_CONFIG_PATH"

  PHP_EXECUTABLE=`"$PHP_CONFIG" --php-binary`
  if test ! -x "$PHP_EXECUTABLE"; then
    AC_MSG_ERROR([PHP executable reported by php-config is not executable: $PHP_EXECUTABLE])
  fi

  PHP_SUBST([CARGO])
  PHP_SUBST([RUSTC])
  PHP_SUBST([PHP_CONFIG])
  PHP_SUBST([PHP_EXECUTABLE])
  PHP_ADD_MAKEFILE_FRAGMENT([$srcdir/Makefile.frag])
fi
