# Testing the shapdf VS Code Extension

This guide explains how to test the extension locally before publishing.

## Prerequisites

1. **Node.js and npm** installed (v16 or higher recommended)
2. **VS Code** installed
3. **shapdf CLI** installed and in PATH:
   ```bash
   cargo install --path .. --features compress
   # Or from crates.io: cargo install shapdf
   ```

## Testing Methods

### Method 1: Launch Extension Development Host (Recommended)

This is the fastest way to test during development:

1. **Open the extension in VS Code:**
   ```bash
   cd vscode-shapdf
   code .
   ```

2. **Install dependencies:**
   ```bash
   npm install
   ```

3. **Compile TypeScript:**
   ```bash
   npm run compile
   ```

4. **Launch debugger:**
   - Press `F5` (or Run → Start Debugging)
   - This opens a new "Extension Development Host" window
   - Your extension is loaded in this window

5. **Test the extension:**
   - Open or create a `.shapdf` file in the dev host window
   - Verify syntax highlighting works
   - Press `Ctrl+Shift+V` (or `Cmd+Shift+V` on Mac) to test PDF preview
   - Make changes to the `.shapdf` file, save, and verify auto-preview works

6. **Check for errors:**
   - In the main VS Code window, open the Debug Console (View → Debug Console)
   - Any errors or console.log outputs will appear here

7. **Make changes and reload:**
   - After making code changes, recompile: `npm run compile`
   - In the Extension Development Host, reload with `Ctrl+R` / `Cmd+R`
   - Or restart debugging from the main window

### Method 2: Install from VSIX Package

This tests the packaged extension as users would install it:

1. **Install VSCE (VS Code Extension Manager):**
   ```bash
   npm install -g @vscode/vsce
   ```

2. **Compile and package:**
   ```bash
   npm run compile
   vsce package
   ```
   This creates `shapdf-0.2.1.vsix`

3. **Install the extension:**
   ```bash
   code --install-extension shapdf-0.2.1.vsix
   ```

   Or in VS Code:
   - View → Extensions
   - Click "..." menu → Install from VSIX
   - Select the `.vsix` file

4. **Test the installed extension:**
   - Reload VS Code (Reload Window command)
   - Open a `.shapdf` file
   - Test all features

5. **Uninstall after testing:**
   ```bash
   code --uninstall-extension shapdf.shapdf
   ```

### Method 3: Watch Mode for Rapid Development

For continuous testing during development:

1. **Run TypeScript in watch mode:**
   ```bash
   npm run watch
   ```

2. **In a separate terminal, press F5** to launch Extension Development Host

3. **Make changes:**
   - Edit TypeScript files
   - Watch mode automatically recompiles
   - Reload Extension Development Host: `Ctrl+R` / `Cmd+R`

## Testing Checklist

### Syntax Highlighting
- [ ] Commands (set, page, line, circle, rectangle) are highlighted
- [ ] Parameters (width, color, cap, etc.) are highlighted
- [ ] Units (mm, cm, in, pt, deg, rad) are highlighted
- [ ] Colors (#hex, rgb(), gray(), named) are highlighted
- [ ] Comments starting with # are styled correctly

### PDF Preview
- [ ] Preview opens with `Ctrl+Shift+V` / `Cmd+Shift+V`
- [ ] Preview opens from editor context menu
- [ ] Preview opens from command palette
- [ ] Preview opens side-by-side
- [ ] Preview icon appears in editor title bar
- [ ] Auto-preview on save works (when enabled)
- [ ] Proper error message when CLI not found
- [ ] Proper error message for invalid syntax

### Configuration
- [ ] `shapdf.cliPath` setting works with custom path
- [ ] `shapdf.autoPreview` setting can be toggled
- [ ] Settings appear in VS Code settings UI

### Edge Cases
- [ ] Unsaved files: should prompt to save or auto-save
- [ ] Large files: should handle without freezing
- [ ] Syntax errors: should show error from CLI
- [ ] Missing CLI: should show helpful error message

## Common Issues and Solutions

### "shapdf CLI not found"

**Problem:** Extension can't find the shapdf executable.

**Solutions:**
1. Verify installation:
   ```bash
   shapdf --version
   ```
2. Check PATH:
   ```bash
   which shapdf  # Unix/Mac
   where shapdf  # Windows
   ```
3. Set custom path in VS Code settings:
   ```json
   {
     "shapdf.cliPath": "/full/path/to/shapdf"
   }
   ```

### Extension Not Loading

**Problem:** Extension doesn't activate when opening `.shapdf` files.

**Solutions:**
1. Check activation events in `package.json`
2. Reload VS Code window (Ctrl+Shift+P → "Reload Window")
3. Check Developer Console (Help → Toggle Developer Tools → Console tab)

### TypeScript Compilation Errors

**Problem:** `npm run compile` fails.

**Solutions:**
1. Delete `node_modules` and reinstall:
   ```bash
   rm -rf node_modules package-lock.json
   npm install
   ```
2. Check TypeScript version compatibility
3. Verify all type definitions are installed

### Changes Not Reflected

**Problem:** Code changes don't appear in Extension Development Host.

**Solutions:**
1. Recompile: `npm run compile`
2. Reload Extension Development Host: `Ctrl+R` / `Cmd+R`
3. Or restart debugger (stop and press F5 again)

## Debugging Tips

### Add Breakpoints
- Open TypeScript files in VS Code
- Click in the gutter to add breakpoints
- Press F5 to debug
- Breakpoints will be hit in the Extension Development Host

### Console Logging
Add debug output:
```typescript
console.log('Preview triggered for:', document.uri.fsPath);
```
View output in Debug Console (main VS Code window)

### Check Extension Output
- View → Output
- Select "shapdf" from dropdown
- Any console.log/error from your extension appears here

### Inspect Variables
When breakpoint hits:
- View Variables panel in Debug sidebar
- Hover over variables in code
- Use Debug Console to evaluate expressions

## Pre-Publication Checklist

Before publishing to the marketplace:

- [ ] All tests pass
- [ ] Version bumped in `package.json` and `CHANGELOG.md`
- [ ] README is complete and accurate
- [ ] CHANGELOG documents all changes
- [ ] No debug code or console.logs left in production code
- [ ] Extension icon added (optional but recommended)
- [ ] Screenshots added to README (optional but recommended)
- [ ] Package builds without errors: `vsce package`
- [ ] VSIX installs and works: `code --install-extension shapdf-*.vsix`
- [ ] Tested on target VS Code version (check `engines.vscode` in package.json)

## Publishing to VS Code Marketplace

Once testing is complete:

1. **Create publisher account:**
   - Go to https://marketplace.visualstudio.com/manage
   - Create a publisher (if you don't have one)

2. **Get Personal Access Token:**
   - Azure DevOps: https://dev.azure.com
   - Create PAT with "Marketplace" scope

3. **Login with VSCE:**
   ```bash
   vsce login <publisher-name>
   ```

4. **Publish:**
   ```bash
   vsce publish
   ```

   Or publish specific version:
   ```bash
   vsce publish 0.2.1
   ```

5. **Verify publication:**
   - Check marketplace: https://marketplace.visualstudio.com/items?itemName=<publisher>.<extension>
   - Wait a few minutes for indexing
   - Install from marketplace to verify

## Resources

- [VS Code Extension API](https://code.visualstudio.com/api)
- [Publishing Extensions](https://code.visualstudio.com/api/working-with-extensions/publishing-extension)
- [Extension Manifest](https://code.visualstudio.com/api/references/extension-manifest)
- [VSCE Documentation](https://github.com/microsoft/vscode-vsce)
