# shapdf for VS Code

Visual Studio Code extension for [shapdf](https://github.com/wqz/shapdf) - efficient programmable generation of shapes in PDF format.

## Features

### Syntax Highlighting

Full syntax highlighting for `.shapdf` files:

- **Commands**: `set`, `page`, `line`, `circle`, `rectangle`
- **Parameters**: `width`, `color`, `cap`, `anchor`, `angle`
- **Units**: `mm`, `cm`, `in`, `pt`, `deg`, `rad`
- **Colors**: Hex colors (`#ff6600`), RGB (`rgb(0.2,0.6,0.9)`), grayscale (`gray(0.3)`), and named colors (`red`, `green`, `blue`, etc.)
- **Comments**: Lines starting with `#`

### PDF Preview

Generate and preview PDFs directly from VS Code:

- **Side-by-side preview** with keyboard shortcut (`Ctrl+Shift+V` / `Cmd+Shift+V` on Mac)
- **Auto-refresh** on save (configurable)
- **CLI-based rendering** using the shapdf command-line tool

## Requirements

The shapdf CLI must be installed and accessible in your PATH:

```bash
cargo install shapdf
```

Verify installation:

```bash
shapdf --version
```

## Usage

### Opening a Preview

1. Open any `.shapdf` file in VS Code
2. Use one of the following methods:
   - Press `Ctrl+Shift+V` (Windows/Linux) or `Cmd+Shift+V` (Mac)
   - Right-click in the editor and select **"Open PDF Preview"**
   - Open the Command Palette (`Ctrl+Shift+P` / `Cmd+Shift+P`) and run **"shapdf: Open PDF Preview to the Side"**
   - Click the preview icon in the editor title bar

The generated PDF will be saved alongside your `.shapdf` file with the same base name.

### Auto-Preview

By default, the PDF preview automatically updates when you save the file. You can disable this in settings:

```json
{
  "shapdf.autoPreview": false
}
```

## Extension Settings

This extension contributes the following settings:

- `shapdf.cliPath`: Path to the shapdf CLI executable (default: `"shapdf"`)
- `shapdf.autoPreview`: Automatically update preview when document is saved (default: `true`)

### Custom CLI Path

If shapdf is not in your PATH, specify the full path:

```json
{
  "shapdf.cliPath": "/usr/local/bin/shapdf"
}
```

Or on Windows:

```json
{
  "shapdf.cliPath": "C:\\Users\\YourName\\bin\\shapdf.exe"
}
```

## Commands

- `shapdf.preview`: Open PDF Preview
- `shapdf.previewToSide`: Open PDF Preview to the Side (default keybinding)

## Example

Create a file `example.shapdf`:

```shapdf
# Simple shapes example
set default_page_size 210mm 210mm
set default_color rgb(0.2,0.2,0.2)

page default
line 10mm 10mm 190mm 10mm width=2mm color=#ff6600 cap=round
circle 40mm 150mm 12mm color=gray(0.3)
rectangle 100mm 110mm 50mm 25mm anchor=center angle=25deg color=rgb(0.2,0.6,0.9)
```

Press `Ctrl+Shift+V` / `Cmd+Shift+V` to see the generated PDF.

## Troubleshooting

### "shapdf CLI not found"

Make sure shapdf is installed and accessible:

```bash
cargo install shapdf
which shapdf  # On Unix/Mac
where shapdf  # On Windows
```

If installed in a custom location, configure `shapdf.cliPath` in VS Code settings.

### Preview not updating

- Ensure `shapdf.autoPreview` is enabled in settings
- Save the file manually (`Ctrl+S` / `Cmd+S`)
- Check the Output panel (View → Output → shapdf) for errors

## Development

### Building the Extension

1. Install dependencies:
   ```bash
   cd vscode-shapdf
   npm install
   ```

2. Compile TypeScript:
   ```bash
   npm run compile
   ```

3. Test in VS Code:
   - Press `F5` to open a new Extension Development Host window
   - Open a `.shapdf` file and test the features

### Packaging

```bash
npm install -g @vscode/vsce
vsce package
```

This creates a `.vsix` file that can be installed with:

```bash
code --install-extension shapdf-0.1.0.vsix
```

## Contributing

Contributions are welcome! Please submit issues and pull requests to the [shapdf repository](https://github.com/wqz/shapdf).

## License

GPL-3.0-or-later

This extension is part of the [shapdf](https://github.com/Teddy-van-Jerry/shapdf) project and is licensed under the GNU General Public License v3.0 or later.
