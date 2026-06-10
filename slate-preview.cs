using System;
using System.IO;
using System.Drawing;
using System.Windows.Forms;
using System.Diagnostics;
using Microsoft.Win32;

namespace SlatePreviewer
{
    public class PreviewForm : Form
    {
        private Panel topPanel;
        private Label lblTitle;
        private Label lblFileName;
        private Button btnOpen;
        private Button btnSave;
        private Button btnCompile;
        private Button btnExport;
        private CheckBox chkLivePreview;
        
        private SplitContainer splitContainer;
        private TextBox txtEditor;
        private WebBrowser webBrowser;
        private StatusStrip statusStrip;
        private ToolStripStatusLabel statusLabel;

        private string currentFilePath = "";
        private string compilerPath = "";
        private Timer liveTimer;
        private bool isTextChanged = false;

        public PreviewForm(string initialFile = null)
        {
            SetBrowserEmulation();
            InitializeComponent();
            LocateCompiler();
            
            if (!string.IsNullOrEmpty(initialFile) && File.Exists(initialFile))
            {
                LoadFile(initialFile);
            }
            else
            {
                LoadDefaultText();
            }
        }

        private void InitializeComponent()
        {
            this.Text = "Slate Visual Studio";
            this.Size = new Size(1100, 750);
            this.StartPosition = FormStartPosition.CenterScreen;
            this.MinimumSize = new Size(800, 500);
            this.BackColor = Color.FromArgb(248, 250, 252);

            // 1. Top Panel (Toolbar with flat styles)
            topPanel = new Panel();
            topPanel.Height = 56;
            topPanel.Dock = DockStyle.Top;
            topPanel.BackColor = Color.FromArgb(15, 23, 42); // slate-900

            lblTitle = new Label();
            lblTitle.Text = "SLATE STUDIO";
            lblTitle.Font = new Font("Segoe UI", 12F, FontStyle.Bold);
            lblTitle.ForeColor = Color.White;
            lblTitle.Location = new Point(16, 16);
            lblTitle.AutoSize = true;

            lblFileName = new Label();
            lblFileName.Text = "untitled.slt";
            lblFileName.Font = new Font("Segoe UI", 9F, FontStyle.Italic);
            lblFileName.ForeColor = Color.FromArgb(148, 163, 184); // slate-400
            lblFileName.Location = new Point(140, 20);
            lblFileName.AutoSize = true;

            // Buttons
            btnOpen = CreateFlatButton("Open file", 300, Color.FromArgb(51, 65, 85), Color.White);
            btnOpen.Click += new EventHandler(OnOpenClick);

            btnSave = CreateFlatButton("Save", 390, Color.FromArgb(51, 65, 85), Color.White);
            btnSave.Click += new EventHandler(OnSaveClick);

            btnCompile = CreateFlatButton("Compile", 480, Color.FromArgb(79, 70, 229), Color.White); // Indigo
            btnCompile.Click += new EventHandler(OnCompileClick);

            btnExport = CreateFlatButton("Export SVG", 570, Color.FromArgb(13, 148, 136), Color.White); // Teal
            btnExport.Click += new EventHandler(OnExportClick);

            chkLivePreview = new CheckBox();
            chkLivePreview.Text = "Live Compile";
            chkLivePreview.Font = new Font("Segoe UI", 9F, FontStyle.Bold);
            chkLivePreview.ForeColor = Color.White;
            chkLivePreview.Location = new Point(680, 16);
            chkLivePreview.Size = new Size(110, 24);
            chkLivePreview.Checked = true;

            topPanel.Controls.Add(lblTitle);
            topPanel.Controls.Add(lblFileName);
            topPanel.Controls.Add(btnOpen);
            topPanel.Controls.Add(btnSave);
            topPanel.Controls.Add(btnCompile);
            topPanel.Controls.Add(btnExport);
            topPanel.Controls.Add(chkLivePreview);

            // 2. SplitContainer
            splitContainer = new SplitContainer();
            splitContainer.Dock = DockStyle.Fill;
            splitContainer.SplitterDistance = 500;
            splitContainer.SplitterWidth = 6;
            splitContainer.BackColor = Color.FromArgb(226, 232, 240);

            // Left Panel (Editor Text Box)
            txtEditor = new TextBox();
            txtEditor.Multiline = true;
            txtEditor.ScrollBars = ScrollBars.Both;
            txtEditor.WordWrap = false;
            txtEditor.Dock = DockStyle.Fill;
            txtEditor.Font = new Font("Consolas", 10.5F);
            txtEditor.ForeColor = Color.FromArgb(15, 23, 42);
            txtEditor.BackColor = Color.White;
            txtEditor.BorderStyle = BorderStyle.None;
            txtEditor.TextChanged += new EventHandler(OnTextChanged);

            // Right Panel (Web Browser)
            webBrowser = new WebBrowser();
            webBrowser.Dock = DockStyle.Fill;
            webBrowser.AllowNavigation = false;
            webBrowser.IsWebBrowserContextMenuEnabled = false;
            webBrowser.WebBrowserShortcutsEnabled = false;

            splitContainer.Panel1.Controls.Add(txtEditor);
            splitContainer.Panel2.Controls.Add(webBrowser);

            // 3. Status Strip
            statusStrip = new StatusStrip();
            statusLabel = new ToolStripStatusLabel("Ready");
            statusStrip.Items.Add(statusLabel);
            statusStrip.BackColor = Color.FromArgb(241, 245, 249);
            statusStrip.ForeColor = Color.FromArgb(71, 85, 105);

            this.Controls.Add(splitContainer);
            this.Controls.Add(statusStrip);
            this.Controls.Add(topPanel);

            // Live compilation timer (debounces key inputs)
            liveTimer = new Timer();
            liveTimer.Interval = 800; // 800ms debounce
            liveTimer.Tick += new EventHandler(OnLiveTimerTick);
        }

        private Button CreateFlatButton(string text, int x, Color backColor, Color foreColor)
        {
            Button btn = new Button();
            btn.Text = text;
            btn.Location = new Point(x, 10);
            btn.Size = new Size(80, 32);
            btn.BackColor = backColor;
            btn.ForeColor = foreColor;
            btn.FlatStyle = FlatStyle.Flat;
            btn.FlatAppearance.BorderSize = 0;
            btn.Font = new Font("Segoe UI", 9F, FontStyle.Bold);
            btn.Cursor = Cursors.Hand;
            return btn;
        }

        private void LocateCompiler()
        {
            string home = Environment.GetEnvironmentVariable("USERPROFILE") ?? Environment.GetEnvironmentVariable("HOME") ?? "";
            if (!string.IsNullOrEmpty(home))
            {
                string path = Path.Combine(home, ".slate", "bin", "slate.exe");
                if (File.Exists(path))
                {
                    compilerPath = path;
                    LogStatus("Compiler found at: " + path);
                    return;
                }
            }

            // Check current folder
            if (File.Exists("slate.exe"))
            {
                compilerPath = Path.GetFullPath("slate.exe");
                LogStatus("Compiler found in workspace.");
                return;
            }

            // Fallback
            compilerPath = "slate";
            LogStatus("Warning: Local compiler not found, using global PATH command.");
        }

        private void LoadDefaultText()
        {
            txtEditor.Text = "# Welcome to Slate Studio\r\n## standalone visual markup and diagram previewer\r\n\r\n::: grid Features cols=2 gap=24\r\n  ::: card Quickstart\r\n    ### Interactive mockups\r\n    - Edit your visual scripts on the left.\r\n    - Save or wait to compile visually on the right.\r\n    [x] High-contrast syntax highlights active\r\n    [x] Direct SVG vector rendering output\r\n    \r\n    > Slate makes visual documents clean and readable.\r\n  :::\r\n\r\n  ::: card DemoDiagram\r\n    ### Business Workflow\r\n    ::: flowchart Pipeline\r\n      Start (circle)\r\n      Code (rect)\r\n      Verify (diamond)\r\n      End (circle)\r\n\r\n      Start -> Code\r\n      Code -> Verify\r\n      Verify -> End: Done\r\n    :::\r\n  :::\r\n:::";
            CompileAndPreview();
        }

        private void OnTextChanged(object sender, EventArgs e)
        {
            isTextChanged = true;
            if (chkLivePreview.Checked)
            {
                liveTimer.Stop();
                liveTimer.Start();
            }
        }

        private void OnLiveTimerTick(object sender, EventArgs e)
        {
            liveTimer.Stop();
            if (isTextChanged)
            {
                CompileAndPreview();
                isTextChanged = false;
            }
        }

        private void LoadFile(string path)
        {
            try
            {
                txtEditor.Text = File.ReadAllText(path);
                currentFilePath = Path.GetFullPath(path);
                lblFileName.Text = Path.GetFileName(path);
                isTextChanged = false;
                LogStatus("File opened: " + path);
                CompileAndPreview();
            }
            catch (Exception ex)
            {
                MessageBox.Show("Could not read file:\n" + ex.Message, "Error opening file", MessageBoxButtons.OK, MessageBoxIcon.Error);
                LoadDefaultText();
            }
        }

        private void OnOpenClick(object sender, EventArgs e)
        {
            using (OpenFileDialog openDlg = new OpenFileDialog())
            {
                openDlg.Filter = "Slate files (*.slt)|*.slt|All files (*.*)|*.*";
                if (openDlg.ShowDialog() == DialogResult.OK)
                {
                    LoadFile(openDlg.FileName);
                }
            }
        }

        private void OnSaveClick(object sender, EventArgs e)
        {
            SaveDocument();
        }

        private bool SaveDocument()
        {
            if (string.IsNullOrEmpty(currentFilePath))
            {
                using (SaveFileDialog saveDlg = new SaveFileDialog())
                {
                    saveDlg.Filter = "Slate files (*.slt)|*.slt";
                    if (saveDlg.ShowDialog() == DialogResult.OK)
                    {
                        currentFilePath = saveDlg.FileName;
                        lblFileName.Text = Path.GetFileName(saveDlg.FileName);
                    }
                    else
                    {
                        return false;
                    }
                }
            }

            try
            {
                File.WriteAllText(currentFilePath, txtEditor.Text);
                isTextChanged = false;
                LogStatus("Saved to: " + currentFilePath);
                return true;
            }
            catch (Exception ex)
            {
                MessageBox.Show("Could not write file:\n" + ex.Message, "Error saving file", MessageBoxButtons.OK, MessageBoxIcon.Error);
                return false;
            }
        }

        private void OnCompileClick(object sender, EventArgs e)
        {
            CompileAndPreview();
        }

        private void OnExportClick(object sender, EventArgs e)
        {
            using (SaveFileDialog saveDlg = new SaveFileDialog())
            {
                saveDlg.Filter = "Vector graphic (*.svg)|*.svg";
                if (saveDlg.ShowDialog() == DialogResult.OK)
                {
                    try
                    {
                        // Copy current temporary preview file or compile fresh
                        string tempInput = Path.Combine(Path.GetTempPath(), "slate_export.slt");
                        string tempOutput = Path.Combine(Path.GetTempPath(), "slate_export.svg");
                        File.WriteAllText(tempInput, txtEditor.Text);

                        string errors;
                        if (RunCompiler(tempInput, tempOutput, out errors))
                        {
                            if (File.Exists(tempOutput))
                            {
                                if (File.Exists(saveDlg.FileName)) File.Delete(saveDlg.FileName);
                                File.Copy(tempOutput, saveDlg.FileName);
                                LogStatus("Exported SVG to: " + saveDlg.FileName);
                                MessageBox.Show("SVG file successfully exported!", "Export Complete", MessageBoxButtons.OK, MessageBoxIcon.Information);
                            }
                        }
                        else
                        {
                            MessageBox.Show("Compilation errors present. Resolve errors first:\n" + errors, "Export Failed", MessageBoxButtons.OK, MessageBoxIcon.Warning);
                        }

                        // Clean up
                        if (File.Exists(tempInput)) File.Delete(tempInput);
                        if (File.Exists(tempOutput)) File.Delete(tempOutput);
                    }
                    catch (Exception ex)
                    {
                        MessageBox.Show("Export failed: " + ex.Message, "Error", MessageBoxButtons.OK, MessageBoxIcon.Error);
                    }
                }
            }
        }

        private void CompileAndPreview()
        {
            string tempInput = Path.Combine(Path.GetTempPath(), "temp_slate.slt");
            string tempOutput = Path.Combine(Path.GetTempPath(), "temp_slate.svg");

            try
            {
                File.WriteAllText(tempInput, txtEditor.Text);
                LogStatus("Compiling...");

                string errors;
                if (RunCompiler(tempInput, tempOutput, out errors))
                {
                    if (File.Exists(tempOutput))
                    {
                        string svgContent = File.ReadAllText(tempOutput);
                        ShowSvgInBrowser(svgContent);
                        LogStatus("Compilation successful.");
                    }
                    else
                    {
                        ShowErrorInBrowser("SVG output file was not found on disk.");
                    }
                }
                else
                {
                    ShowErrorInBrowser(errors);
                    LogStatus("Compilation failed.");
                }
            }
            catch (Exception ex)
            {
                ShowErrorInBrowser(ex.Message);
                LogStatus("Error running preview: " + ex.Message);
            }
            finally
            {
                try
                {
                    if (File.Exists(tempInput)) File.Delete(tempInput);
                    if (File.Exists(tempOutput)) File.Delete(tempOutput);
                }
                catch {}
            }
        }

        private bool RunCompiler(string inputPath, string outputPath, out string errors)
        {
            errors = "";
            try
            {
                ProcessStartInfo startInfo = new ProcessStartInfo();
                startInfo.FileName = compilerPath;
                startInfo.Arguments = string.Format("compile \"{0}\" -o \"{1}\"", inputPath, outputPath);
                startInfo.UseShellExecute = false;
                startInfo.CreateNoWindow = true;
                startInfo.RedirectStandardError = true;
                startInfo.RedirectStandardOutput = true;

                using (Process proc = Process.Start(startInfo))
                {
                    string output = proc.StandardOutput.ReadToEnd();
                    string error = proc.StandardError.ReadToEnd();
                    proc.WaitForExit();

                    if (proc.ExitCode != 0)
                    {
                        errors = string.IsNullOrEmpty(error) ? output : error;
                        return false;
                    }
                    return true;
                }
            }
            catch (Exception ex)
            {
                errors = "Failed to launch compiler process (" + compilerPath + "): " + ex.Message + 
                         "\nEnsure Slate is installed by running the setup utility.";
                return false;
            }
        }

        private void ShowSvgInBrowser(string svgContent)
        {
            string html = string.Format(@"<!DOCTYPE html>
<html>
<head>
    <meta http-equiv='X-UA-Compatible' content='IE=edge' />
    <style>
        body {{
            margin: 0;
            padding: 24px;
            background-color: #F8FAFC;
            display: flex;
            justify-content: center;
            align-items: flex-start;
            font-family: sans-serif;
        }}
        .artboard-container {{
            background-color: #FFFFFF;
            box-shadow: 0 4px 6px -1px rgba(0,0,0,0.05), 0 2px 4px -1px rgba(0,0,0,0.03), 0 0 0 1px rgba(0,0,0,0.05);
            border-radius: 12px;
            padding: 16px;
            max-width: 100%;
        }}
        svg {{
            display: block;
            max-width: 100%;
            height: auto;
        }}
    </style>
</head>
<body>
    <div class='artboard-container'>
        {0}
    </div>
</body>
</html>", svgContent);

            webBrowser.DocumentText = html;
        }

        private void ShowErrorInBrowser(string errorMessage)
        {
            string html = string.Format(@"<!DOCTYPE html>
<html>
<head>
    <meta http-equiv='X-UA-Compatible' content='IE=edge' />
    <style>
        body {{
            margin: 0;
            padding: 24px;
            background-color: #FEF2F2;
            color: #991B1B;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
        }}
        .error-card {{
            border: 1px solid #FCA5A5;
            background-color: #FFFFFF;
            border-radius: 8px;
            padding: 20px;
            box-shadow: 0 4px 6px -1px rgba(0,0,0,0.05);
        }}
        h3 {{
            margin-top: 0;
            color: #DC2626;
        }
        pre {{
            background-color: #F9FAFB;
            border: 1px solid #E5E7EB;
            padding: 12px;
            border-radius: 6px;
            white-space: pre-wrap;
            word-wrap: break-word;
            font-family: Consolas, Monaco, monospace;
            font-size: 13px;
            color: #374151;
        }}
    </style>
</head>
<body>
    <div class='error-card'>
        <h3>Slate Compilation Error</h3>
        <p>There was an error compiling your script:</p>
        <pre>{0}</pre>
    </div>
</body>
</html>", EscapeHtml(errorMessage));

            webBrowser.DocumentText = html;
        }

        private string EscapeHtml(string text)
        {
            if (string.IsNullOrEmpty(text)) return "";
            return text
                .Replace("&", "&amp;")
                .Replace("<", "&lt;")
                .Replace(">", "&gt;")
                .Replace("\"", "&quot;")
                .Replace("'", "&#039;");
        }

        private void LogStatus(string text)
        {
            statusLabel.Text = "[" + DateTime.Now.ToString("HH:mm:ss") + "] " + text;
        }

        private static void SetBrowserEmulation()
        {
            try
            {
                string appName = Path.GetFileName(Process.GetCurrentProcess().MainModule.FileName);
                using (RegistryKey key = Registry.CurrentUser.OpenSubKey(@"Software\Microsoft\Internet Explorer\Main\FeatureControl\FEATURE_BROWSER_EMULATION", true))
                {
                    if (key != null)
                    {
                        key.SetValue(appName, 11001, RegistryValueKind.DWord);
                    }
                }
            }
            catch {}
        }

        [STAThread]
        public static void Main(string[] args)
        {
            Application.EnableVisualStyles();
            Application.SetCompatibleTextRenderingDefault(false);
            
            string initialFile = args.Length > 0 ? args[0] : null;
            Application.Run(new PreviewForm(initialFile));
        }
    }
}
