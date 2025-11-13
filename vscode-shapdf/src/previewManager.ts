import * as vscode from 'vscode';
import * as path from 'path';
import * as fs from 'fs';
import * as crypto from 'crypto';
import { exec } from 'child_process';
import { promisify } from 'util';

const execAsync = promisify(exec);

export class PreviewManager {
    private context: vscode.ExtensionContext;
    private previewPanels: Map<string, vscode.Uri> = new Map();
    private documentContentHashes: Map<string, string> = new Map();
    private diagnosticCollection: vscode.DiagnosticCollection;

    constructor(context: vscode.ExtensionContext) {
        this.context = context;
        this.diagnosticCollection = vscode.languages.createDiagnosticCollection('shapdf');
        context.subscriptions.push(this.diagnosticCollection);
    }

    private computeContentHash(content: string): string {
        return crypto.createHash('sha256').update(content).digest('hex');
    }

    private hasContentChanged(document: vscode.TextDocument): boolean {
        const currentHash = this.computeContentHash(document.getText());
        const previousHash = this.documentContentHashes.get(document.uri.toString());
        return previousHash !== currentHash;
    }

    public async showPreview(sideBySide: boolean): Promise<void> {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            vscode.window.showErrorMessage('No active editor found');
            return;
        }

        if (editor.document.languageId !== 'shapdf') {
            vscode.window.showErrorMessage('Active file is not a .shapdf file');
            return;
        }

        // For explicit preview command, always show the PDF
        await this.generateAndShowPdf(editor.document, sideBySide, true);
    }

    public async updatePreview(document: vscode.TextDocument): Promise<void> {
        if (document.languageId !== 'shapdf') {
            return;
        }

        // Check if content has changed since last compilation
        if (!this.hasContentChanged(document)) {
            console.log('No content changes detected, skipping recompilation');
            return;
        }

        // For auto-save, don't open the PDF (just regenerate it silently)
        await this.generateAndShowPdf(document, false, false);
    }

    private parseErrorMessage(stderr: string): { line: number; message: string } | null {
        // Try to parse error messages in various formats
        // Format 1: Rust Debug format - "ParseError { line: 15, message: "Unknown command 'wh'" }"
        let match = stderr.match(/ParseError\s*\{\s*line:\s*(\d+),\s*message:\s*"([^"]+)"\s*\}/);
        if (match) {
            return { line: parseInt(match[1]) - 1, message: match[2].trim() };
        }

        // Format 2: "Error at line 5: message"
        match = stderr.match(/(?:error|Error).*?(?:at )?line (\d+):?\s*(.+)/i);
        if (match) {
            return { line: parseInt(match[1]) - 1, message: match[2].trim() };
        }

        // Format 3: "file.shapdf:5: message"
        match = stderr.match(/\.shapdf:(\d+):?\s*(.+)/i);
        if (match) {
            return { line: parseInt(match[1]) - 1, message: match[2].trim() };
        }

        // Format 4: Just "Error: message" without line number
        match = stderr.match(/(?:error|Error):?\s*(.+)/i);
        if (match) {
            return { line: 0, message: match[1].trim() };
        }

        return null;
    }

    private async generateAndShowPdf(document: vscode.TextDocument, sideBySide: boolean, openPdf: boolean): Promise<void> {
        // Save the document if it has unsaved changes
        if (document.isDirty) {
            await document.save();
        }

        const config = vscode.workspace.getConfiguration('shapdf');
        const cliPath = config.get<string>('cliPath', 'shapdf');

        // Clear any existing diagnostics for this document
        this.diagnosticCollection.delete(document.uri);

        try {
            // Create a temporary output path
            const sourceFile = document.uri.fsPath;
            const outputDir = path.dirname(sourceFile);
            const baseName = path.basename(sourceFile, '.shapdf');
            const outputFile = path.join(outputDir, `${baseName}.pdf`);

            // Show progress notification only if opening the PDF
            const progressOptions = {
                location: openPdf ? vscode.ProgressLocation.Notification : vscode.ProgressLocation.Window,
                title: 'Generating PDF...',
                cancellable: false
            };

            await vscode.window.withProgress(progressOptions, async () => {
                try {
                    // Run the shapdf CLI with explicit output path
                    const command = `"${cliPath}" --output "${outputFile}" "${sourceFile}"`;
                    console.log('Running command:', command);
                    console.log('Working directory:', outputDir);

                    const { stdout, stderr } = await execAsync(command, {
                        cwd: outputDir
                    });

                    if (stdout) {
                        console.log('shapdf stdout:', stdout);
                    }

                    if (stderr) {
                        console.error('shapdf stderr:', stderr);
                        // If stderr contains actual errors (not just warnings), throw
                        if (stderr.toLowerCase().includes('error')) {
                            throw new Error(stderr);
                        }
                    }

                    // Check if PDF was created
                    if (!fs.existsSync(outputFile)) {
                        throw new Error(`PDF file was not created at: ${outputFile}\nCommand: ${command}`);
                    }

                    // Update the content hash after successful generation
                    const currentHash = this.computeContentHash(document.getText());
                    this.documentContentHashes.set(document.uri.toString(), currentHash);

                    // Store the preview panel reference
                    this.previewPanels.set(document.uri.toString(), vscode.Uri.file(outputFile));

                    // Only open the PDF if explicitly requested
                    if (openPdf) {
                        const viewColumn = sideBySide ? vscode.ViewColumn.Beside : vscode.ViewColumn.Active;
                        await vscode.commands.executeCommand('vscode.open', vscode.Uri.file(outputFile), viewColumn);
                        vscode.window.showInformationMessage(`PDF generated: ${baseName}.pdf`);
                    } else {
                        // Silent update - show in status bar instead
                        console.log(`PDF silently updated: ${baseName}.pdf`);
                    }
                } catch (error) {
                    throw error;
                }
            });

        } catch (error: any) {
            let errorMessage = 'Failed to generate PDF';
            let stderrText = '';

            if (error.code === 'ENOENT') {
                errorMessage = `shapdf CLI not found at "${cliPath}". Please install it with 'cargo install shapdf' or configure the path in settings.`;
            } else if (error.stderr) {
                stderrText = error.stderr;
                errorMessage = `shapdf error: ${error.stderr}`;
            } else if (error.message) {
                stderrText = error.message;
                errorMessage = `Error: ${error.message}`;
            }

            // Parse error and create diagnostic
            if (stderrText) {
                const parsedError = this.parseErrorMessage(stderrText);
                if (parsedError) {
                    const line = Math.max(0, parsedError.line); // Ensure line is non-negative
                    const range = new vscode.Range(line, 0, line, Number.MAX_VALUE);
                    const diagnostic = new vscode.Diagnostic(
                        range,
                        parsedError.message,
                        vscode.DiagnosticSeverity.Error
                    );
                    diagnostic.source = 'shapdf';
                    this.diagnosticCollection.set(document.uri, [diagnostic]);
                }
            }

            vscode.window.showErrorMessage(errorMessage);
            console.error('PDF generation error:', error);
        }
    }

    public dispose(): void {
        this.previewPanels.clear();
        this.documentContentHashes.clear();
        this.diagnosticCollection.dispose();
    }
}
