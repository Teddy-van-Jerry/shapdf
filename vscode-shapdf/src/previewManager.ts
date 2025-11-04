import * as vscode from 'vscode';
import * as path from 'path';
import * as fs from 'fs';
import { exec } from 'child_process';
import { promisify } from 'util';

const execAsync = promisify(exec);

export class PreviewManager {
    private context: vscode.ExtensionContext;
    private previewPanels: Map<string, vscode.Uri> = new Map();

    constructor(context: vscode.ExtensionContext) {
        this.context = context;
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

        await this.generateAndShowPdf(editor.document, sideBySide);
    }

    public async updatePreview(document: vscode.TextDocument): Promise<void> {
        if (document.languageId !== 'shapdf') {
            return;
        }

        // Check if there's a preview panel already open for this document
        const previewUri = this.previewPanels.get(document.uri.toString());
        if (previewUri) {
            await this.generateAndShowPdf(document, false);
        }
    }

    private async generateAndShowPdf(document: vscode.TextDocument, sideBySide: boolean): Promise<void> {
        // Save the document if it has unsaved changes
        if (document.isDirty) {
            await document.save();
        }

        const config = vscode.workspace.getConfiguration('shapdf');
        const cliPath = config.get<string>('cliPath', 'shapdf');

        try {
            // Create a temporary output path
            const sourceFile = document.uri.fsPath;
            const outputDir = path.dirname(sourceFile);
            const baseName = path.basename(sourceFile, '.shapdf');
            const outputFile = path.join(outputDir, `${baseName}.pdf`);

            // Show progress notification
            await vscode.window.withProgress({
                location: vscode.ProgressLocation.Notification,
                title: 'Generating PDF...',
                cancellable: false
            }, async () => {
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

                    // Store the preview panel reference
                    this.previewPanels.set(document.uri.toString(), vscode.Uri.file(outputFile));

                    // Open the PDF
                    const viewColumn = sideBySide ? vscode.ViewColumn.Beside : vscode.ViewColumn.Active;
                    await vscode.commands.executeCommand('vscode.open', vscode.Uri.file(outputFile), viewColumn);

                    vscode.window.showInformationMessage(`PDF generated: ${baseName}.pdf`);
                } catch (error) {
                    throw error;
                }
            });

        } catch (error: any) {
            let errorMessage = 'Failed to generate PDF';

            if (error.code === 'ENOENT') {
                errorMessage = `shapdf CLI not found at "${cliPath}". Please install it with 'cargo install shapdf' or configure the path in settings.`;
            } else if (error.stderr) {
                errorMessage = `shapdf error: ${error.stderr}`;
            } else if (error.message) {
                errorMessage = `Error: ${error.message}`;
            }

            vscode.window.showErrorMessage(errorMessage);
            console.error('PDF generation error:', error);
        }
    }

    public dispose(): void {
        this.previewPanels.clear();
    }
}
