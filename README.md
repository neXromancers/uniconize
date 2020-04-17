# uniconize (not a unicorn)

This a is fix for Wine games blackscreening on sway and other tiling window
managers that don't support iconized windows.

## Installation

- Manual: with a recent Rust toolchain and a fresh clone of this repo,
  `cargo install --path .`
- [crates.io](https://crates.io/crates/uniconize): `cargo install uniconize`
- Arch Linux: [AUR package](https://aur.archlinux.org/packages/uniconize/)
- Other distros: make a pull request to add your package or build script!

## Usage

Just run this program! On sway, you'll want to throw `exec uniconize` somewhere
in your configuration. There are no options.

## Explanation

i3 and sway are tiling window managers and do not support iconized windows by
design. Unfortunately for them, [the ICCCM standard][icccm] says that they must.
That said, for native applications, it's fine to ignore iconization requests
because, by X11 convention, state will not change until the window manager says
so.

However, Windows applications running in Wine are more problematic. On Windows,
when the application sets the iconized flag, the window is guaranteed to be
iconized, without any feedback from the window manager. Wine can therefore not
conform to the convention, and can only blindly assume that the application has
been iconized.

This is a problem on i3 and sway, as because they do not support such a state,
they have no mechanism to bring a client back from it. And since focusing out of
a fullscreen window will immediately iconize it on Windows, games tend to get
stuck into that state pretty easily.

As it turns out though, it is sufficient to tell the game that it's been
uniconized to bring it back from the dead, without resorting to Wine's virtual
desktop. i3 already includes this behavior, but fixing this in sway appears more
complicated, so I've opted to write this little program instead.

For more information, please take a look at the following links:
- [Christophe Tronche's excellent documentation on this topic][tronche]
- [My i3 pull request implementing the same fix (merged)][i3_fix]
- [My initial attempt at fixing this in i3][i3_draft]
- [The Wine bug report for this problem][wine_bug]

[icccm]: https://www.x.org/releases/X11R7.6/doc/xorg-docs/specs/ICCCM/icccm.html
[tronche]: https://tronche.com/gui/x/icccm/sec-4.html#s-4.1.4
[i3_fix]: https://github.com/i3/i3/pull/3421
[i3_draft]: https://github.com/i3/i3/pull/3370
[wine_bug]: https://bugs.winehq.org/show_bug.cgi?id=45690
