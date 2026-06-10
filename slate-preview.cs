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
        private WebBrowser webBrowser;
        private string compilerPath = "";

        public PreviewForm(string initialFile = null)
        {
            SetBrowserEmulation();
            InitializeComponent();
            LocateCompiler();
            
            if (!string.IsNullOrEmpty(initialFile) && File.Exists(initialFile))
            {
                CompileAndPreview(initialFile);
            }
            else
            {
                // Load default welcome slate
                string tempWelcome = Path.Combine(Path.GetTempPath(), "welcome.slt");
                File.WriteAllText(tempWelcome, GetDefaultText());
                CompileAndPreview(tempWelcome);
                try { File.Delete(tempWelcome); } catch {}
            }
        }

        private void InitializeComponent()
        {
            this.Text = "Slate Studio Previewer";
            this.Size = new Size(950, 750);
            this.StartPosition = FormStartPosition.CenterScreen;
            this.MinimumSize = new Size(600, 400);
            this.BackColor = Color.White;

            webBrowser = new WebBrowser();
            webBrowser.Dock = DockStyle.Fill;
            webBrowser.AllowNavigation = false;
            webBrowser.IsWebBrowserContextMenuEnabled = false;
            webBrowser.WebBrowserShortcutsEnabled = false;

            this.Controls.Add(webBrowser);
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
                    return;
                }
            }

            if (File.Exists("slate.exe"))
            {
                compilerPath = Path.GetFullPath("slate.exe");
                return;
            }

            compilerPath = "slate";
        }

        private void CompileAndPreview(string sltPath)
        {
            string tempHtml = Path.Combine(Path.GetTempPath(), "temp_slate.html");
            try
            {
                string errors;
                if (RunCompiler(sltPath, tempHtml, out errors))
                {
                    if (File.Exists(tempHtml))
                    {
                        webBrowser.Navigate(tempHtml);
                    }
                }
                else
                {
                    ShowErrorInBrowser(errors);
                }
            }
            catch (Exception ex)
            {
                ShowErrorInBrowser(ex.Message);
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
            padding: 20px;
        }}
        h3 {{
            margin-top: 0;
            color: #DC2626;
        }}
        pre {{
            background-color: #F9FAFB;
            border: 1px solid #E5E7EB;
            padding: 12px;
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

        private string GetDefaultText()
        {
            return "# Welcome to Slate Studio\r\n## standalone visual markup and diagram previewer\r\n\r\n::: grid Features cols=2 gap=24\r\n  ::: card Quickstart\r\n    ### Interactive mockups\r\n    - Edit your visual scripts on the left.\r\n    - Save or wait to compile visually on the right.\r\n    [x] High-contrast syntax highlights active\r\n    [x] Direct local JSON previewing\r\n    \r\n    > Slate makes visual documents clean and readable.\r\n  :::\r\n\r\n  ::: card DemoDiagram\r\n    ### Business Workflow\r\n    ::: flowchart Pipeline\r\n      Start (circle)\r\n      Code (rect)\r\n      Verify (diamond)\r\n      End (circle)\r\n\r\n      Start -> Code\r\n      Code -> Verify\r\n      Verify -> End: Done\r\n    :::\r\n  :::\r\n:::";
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

        [System.Runtime.InteropServices.DllImport("user32.dll")]
        private static extern bool SetProcessDPIAware();

        [STAThread]
        public static void Main(string[] args)
        {
            try { SetProcessDPIAware(); } catch {}
            Application.EnableVisualStyles();
            Application.SetCompatibleTextRenderingDefault(false);
            
            string initialFile = args.Length > 0 ? args[0] : null;
            Application.Run(new PreviewForm(initialFile));
        }
    }
}
