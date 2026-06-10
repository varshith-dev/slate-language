const vscode = require('vscode');
const path = require('path');
const fs = require('fs');
const { exec } = require('child_process');

function activate(context) {
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

        const filePath = document.uri.fsPath;
        const fileDir = path.dirname(filePath);
        const ext = path.extname(filePath);
        const jsonPath = filePath.replace(ext, '.json');

        const updatePreview = () => {
            const compilerPath = getCompilerPath();
            const command = `"${compilerPath}" compile "${filePath}" -o "${jsonPath}"`;
            
            exec(command, { cwd: fileDir }, (error, stdout, stderr) => {
                if (error) {
                    vscode.window.showErrorMessage(error.message || stderr);
                    return;
                }
                
                if (fs.existsSync(jsonPath)) {
                    vscode.workspace.openTextDocument(vscode.Uri.file(jsonPath)).then(doc => {
                        vscode.window.showTextDocument(doc, vscode.ViewColumn.Two, true);
                    });
                }
            });
        };

        updatePreview();
    });

    context.subscriptions.push(disposable);
}

function getCompilerPath() {
    const configPath = vscode.workspace.getConfiguration('slate').get('compilerPath');
    if (configPath) {
        return configPath;
    }

    if (vscode.workspace.workspaceFolders && vscode.workspace.workspaceFolders.length > 0) {
        const workspaceRoot = vscode.workspace.workspaceFolders[0].uri.fsPath;
        
        const releasePath = path.join(workspaceRoot, 'target', 'release', 'slate.exe');
        if (fs.existsSync(releasePath)) return releasePath;

        const debugPath = path.join(workspaceRoot, 'target', 'debug', 'slate.exe');
        if (fs.existsSync(debugPath)) return debugPath;

        const rootExe = path.join(workspaceRoot, 'slate.exe');
        if (fs.existsSync(rootExe)) return rootExe;
    }

    return 'slate';
}

function deactivate() {}

module.exports = {
    activate,
    deactivate
};
