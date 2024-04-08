<div align="center">
  
<h1 align="center">Request Maker</h1>

<img src="data/icons/com.emarifer.RequestMaker.png" width="128" height="128">

<hr />

<p style="margin-bottom: 16px;">
    HTTP request maker and APIs tester made with Rust & GTK+4.
</p>

> 🚧 This is a work in progress and therefore you should expect that the
> application may not have all the features at this moment.

<br />
  
![GitHub License](https://img.shields.io/github/license/emarifer/request-maker) ![Static Badge](https://img.shields.io/badge/Rust-%3E=1.77-orangered) ![Static Badge](https://img.shields.io/badge/GTK+-%3E=4.6-blue) ![Static Badge](https://img.shields.io/badge/GtkSourceView-%3E=5.4-blue) ![Static Badge](https://img.shields.io/badge/GLib-%3E=2.72-blue)

</div>

<hr />

## How to build

### Flatpak

Install the runtime:

```sh
flatpak install --user org.gnome.Sdk//45 org.freedesktop.Sdk.Extension.rust-stable//23.08
```

To build and run the Flatpak:

```sh
flatpak-builder --user flatpak_app build-aux/com.emarifer.RequestMaker.json
flatpak-builder --run flatpak_app build-aux/com.emarifer.RequestMaker.json request-maker
```

To install the Flatpak into your system or user Flatpak, use the `--install`
flag and maybe the `--user`:

```sh
flatpak-builder --user --install flatpak_app build-aux/com.emarifer.RequestMaker.json
```

You will find `Request Maker` in your application launcher, or you can launch it with
`flatpak run com.emarifer.RequestMaker`.

Likewise, you can uninstall the application by running `flatpak uninstall --user com.emarifer.RequestMaker`. In any case, the complete uninstallation of the application requires deleting the `flatpak_app/` and `.flatpak-builder/` folders generated in the current folder when installing.

>[!NOTE]
>It may be necessary to install `flatpak builder` on your system by running `sudo apt install flatpak-builder`.

## Licenses

`Request Maker` is published under the terms of the GNU General Public License v3.0 or later.

```
Copyright 2024 the Request Maker authors

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.
```
The `Request Maker` icon is published under the a [Creative Commons
Attribution-ShareAlike 4.0 International license][ccbysa].

## Credits and acknowledgments

`Request Maker` is maintained by [Enrique Marín][emarifer].

Big shoutout to many of the GTK and GNOME Circle applications out there whose
source code I've read in order to know how to use some of the GTK features that
you cannot learn just by reading the official docs.

[ccbysa]: https://creativecommons.org/licenses/by-sa/4.0/
[emarifer]: https://github.com/emarifer