# web-fast ⚡

CLI to scaffold web projects with React + Vite + Tailwind CSS already configured and ready to use. No setup, no wasted time.

## What is it?

`web-fast` is a command-line tool written in Rust that generates a modern web project structure in seconds, with everything already set up — including React, Vite, and Tailwind CSS.

> **Note:** This is an early version. More templates are coming soon, including Prisma, monorepo support, and more.

## What's available now

| Template | Description |
|---|---|
| `tailwind` | React + Vite + Tailwind CSS |

## Coming soon

- Prisma integration
- Monorepo setup
- More templates...

## Installation

### Linux and macOS

> Automatic installer coming soon. For now, download the binary manually from the [Releases](https://github.com/Rhuann01/webFast/releases/latest) page.

After downloading, extract the file and move the binary to your PATH:

```sh
tar -xzf web-fast-*.tar.gz
sudo mv web-fast /usr/local/bin/
```

### Windows

**Option 1 — PowerShell (automatic)**

Open PowerShell as administrator and run:

```powershell
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
irm https://raw.githubusercontent.com/Rhuann01/webFast/main/install.ps1 | iex
```

> Automatic installer coming soon. For now, use Option 2 below.

**Option 2 — Manual**

1. Go to the [Releases](https://github.com/Rhuann01/webFast/releases/latest) page
2. Download `web-fast-x86_64-pc-windows-msvc.zip`
3. Extract the `.exe`
4. Move it to a folder of your choice (e.g. `C:\Programs\web-fast\`)
5. Add that folder to your PATH:
   - Search for "environment variables" in Windows
   - Under "User variables", click `Path` → `Edit`
   - Add the path to the folder containing the `.exe`
   - Click OK and **restart your terminal**

## Usage

```sh
web-fast --help
```

## Requirements

None. The binary is standalone — no need to have Rust, Node, or anything else installed.

## License

MIT © Rhuann