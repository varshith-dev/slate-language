const vscode = require('vscode');
const path = require('path');
const fs = require('fs');
const { exec } = require('child_process');

function activate(context) {
    let activePanels = new Map();

    let disposable = vscode.commands.registerCommand('slate.showPreview', () => {
        const activeEditor = vscode.window.activeTextEditor;
        if (!activeEditor) {
            vscode.window.showErrorMessage('No active editor found.');
            return;
        }

        const document = activeEditor.document;
        if (document.languageId !== 'slate' && !document.fileName.endsWith('.slt')) {
            vscode.window.showErrorMessage('Active document is not a Slate (.slt) file.');
            return;
        }

        const fileUri = document.uri;
        const filePath = fileUri.fsPath;
        const fileDir = path.dirname(filePath);
        const fileName = path.basename(filePath);
        
        // Output SVG path
        const ext = path.extname(filePath);
        const svgPath = filePath.replace(ext, '.svg');

        if (activePanels.has(filePath)) {
            activePanels.get(filePath).reveal(vscode.ViewColumn.Two);
            return;
        }

        const panel = vscode.window.createWebviewPanel(
            'slatePreview',
            `Slate: ${fileName}`,
            vscode.ViewColumn.Two,
            {
                enableScripts: true,
                localResourceRoots: [vscode.Uri.file(fileDir)]
            }
        );

        activePanels.set(filePath, panel);

        // Compile and update function
        const updatePreview = () => {
            const compilerPath = getCompilerPath();
            const command = `"${compilerPath}" compile "${filePath}" -o "${svgPath}"`;
            
            exec(command, { cwd: fileDir }, (error, stdout, stderr) => {
                if (error) {
                    panel.webview.html = getErrorHtml(error.message || stderr);
                    return;
                }
                
                try {
                    if (fs.existsSync(svgPath)) {
                        const svgContent = fs.readFileSync(svgPath, 'utf8');
                        panel.webview.html = getWebviewContent(fileName, svgContent);
                    } else {
                        panel.webview.html = getErrorHtml(`SVG output file was not generated at: ${svgPath}`);
                    }
                } catch (e) {
                    panel.webview.html = getErrorHtml(`Failed to read SVG file: ${e.message}`);
                }
            });
        };

        // Initial preview
        updatePreview();

        // Re-compile and update preview when the document is saved
        const saveSubscription = vscode.workspace.onDidSaveTextDocument(doc => {
            if (doc.uri.fsPath === filePath) {
                updatePreview();
            }
        });

        panel.onDidDispose(() => {
            activePanels.delete(filePath);
            saveSubscription.dispose();
        });
    });

    context.subscriptions.push(disposable);
}

function getCompilerPath() {
    // 1. Check user configuration (if any)
    const configPath = vscode.workspace.getConfiguration('slate').get('compilerPath');
    if (configPath) {
        return configPath;
    }

    // 2. Check if a workspace is open and check common local paths
    if (vscode.workspace.workspaceFolders && vscode.workspace.workspaceFolders.length > 0) {
        const workspaceRoot = vscode.workspace.workspaceFolders[0].uri.fsPath;
        
        // Check local binary paths
        const releasePath = path.join(workspaceRoot, 'target', 'release', 'slate.exe');
        if (fs.existsSync(releasePath)) {
            return releasePath;
        }

        const debugPath = path.join(workspaceRoot, 'target', 'debug', 'slate.exe');
        if (fs.existsSync(debugPath)) {
            return debugPath;
        }

        const rootExe = path.join(workspaceRoot, 'slate.exe');
        if (fs.existsSync(rootExe)) {
            return rootExe;
        }
    }

    // 3. Fallback to global command
    return 'slate';
}

function getWebviewContent(fileName, svgContent) {
    return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Slate Preview: ${fileName}</title>
    <style>
        body {
            margin: 0;
            padding: 24px;
            background-color: #F8FAFC;
            display: flex;
            justify-content: center;
            align-items: flex-start;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
            color: #0F172A;
        }
        .preview-container {
            background-color: #ffffff;
            box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.05), 0 2px 4px -1px rgba(0, 0, 0, 0.03), 0 0 0 1px rgba(0, 0, 0, 0.05);
            border-radius: 12px;
            padding: 16px;
            max-width: 100%;
            overflow: auto;
        }
        svg {
            display: block;
            max-width: 100%;
            height: auto;
        }
    </style>
</head>
<body>
    <div class="preview-container">
        ${svgContent}
    </div>
</body>
</html>`;
}

function getErrorHtml(errorMessage) {
    return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Slate Preview Error</title>
    <style>
        body {
            margin: 0;
            padding: 24px;
            background-color: #FEF2F2;
            color: #991B1B;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
        }
        .error-container {
            border: 1px solid #FCA5A5;
            background-color: #FFFFFF;
            border-radius: 8px;
            padding: 20px;
            box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.05);
        }
        h3 {
            margin-top: 0;
            color: #DC2626;
        }
        pre {
            background-color: #F9FAFB;
            border: 1px solid #E5E7EB;
            padding: 12px;
            border-radius: 6px;
            white-space: pre-wrap;
            word-wrap: break-word;
            font-family: Consolas, Monaco, monospace;
            font-size: 13px;
            color: #374151;
        }
    </style>
</head>
<body>
    <div class="error-container">
        <h3>Slate Compilation Error</h3>
        <p>An error occurred while compiling the Slate file to SVG:</p>
        <pre>${escapeHtml(errorMessage)}</pre>
    </div>
</body>
</html>`;
}

function escapeHtml(text) {
    return text
        .replace(/&/g, "&amp;")
        .replace(/</g, "&lt;")
        .replace(/>/g, "&gt;")
        .replace(/"/g, "&quot;")
        .replace(/'/g, "&#039;");
}

function deactivate() {}

module.exports = {
    activate,
    deactivate
};
