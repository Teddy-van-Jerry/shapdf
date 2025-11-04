import * as vscode from 'vscode';
import { PreviewManager } from './previewManager';

let previewManager: PreviewManager;

export function activate(context: vscode.ExtensionContext) {
    console.log('shapdf extension is now active');

    previewManager = new PreviewManager(context);

    // Register commands
    context.subscriptions.push(
        vscode.commands.registerCommand('shapdf.preview', () => {
            previewManager.showPreview(false);
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('shapdf.previewToSide', () => {
            previewManager.showPreview(true);
        })
    );

    // Auto-preview on save if enabled
    context.subscriptions.push(
        vscode.workspace.onDidSaveTextDocument((document) => {
            const config = vscode.workspace.getConfiguration('shapdf');
            if (config.get<boolean>('autoPreview', true) && document.languageId === 'shapdf') {
                previewManager.updatePreview(document);
            }
        })
    );
}

export function deactivate() {
    if (previewManager) {
        previewManager.dispose();
    }
}
