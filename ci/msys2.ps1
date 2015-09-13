If (!${env:MSYS2_ARCH}) {
  Exit 0
}

Start-FileDownload `
  ("http://kent.dl.sourceforge.net/project/msys2/Base/" + `
    $env:MSYS2_ARCH + "/msys2-base-" + $env:MSYS2_ARCH + "-" + `
    $env:MSYS2_BASEVER + ".tar.xz") `
  -FileName "msys2.tar.xz"

7z x msys2.tar.xz
7z x msys2.tar > $nul
Move-Item ("msys" + $env:MSYS2_BITS) msys2
.\msys2\usr\bin\bash.exe -lc ""
.\msys2\usr\bin\bash.exe -lc `
  ("for i in {1..3}; do " + `
      "pacman --noconfirm -Suy mingw-w64-"+ $env:MSYS2_ARCH + "-gcc " + `
        "&& break || sleep 15; " + `
  "done")
.\msys2\autorebase.bat
