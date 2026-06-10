using System;
using System.Drawing;
using System.Windows.Forms;
using System.IO;
using System.Reflection;
using System.Runtime.InteropServices;
using Microsoft.Win32;

namespace SlateSetup
{
    public class SetupForm : Form
    {
        private Button btnInstall;
        private Button btnUninstall;
        private Button btnExit;
        private TextBox txtLog;
        private ProgressBar progressBar;
        private Label lblTitle;
        private Label lblSubtitle;
        private PictureBox picLogo;

        // P/Invoke for environment update broadcast
        [DllImport("user32.dll", SetLastError = true, CharSet = CharSet.Auto)]
        private static extern IntPtr SendMessageTimeout(
            IntPtr hWnd, 
            uint Msg, 
            UIntPtr wParam, 
            string lParam, 
            uint fuFlags, 
            uint uTimeout, 
            out UIntPtr lpdwResult
        );

        private static readonly IntPtr HWND_BROADCAST = new IntPtr(0xffff);
        private const uint WM_SETTINGCHANGE = 0x001a;
        private const uint SMTO_ABORTIFHUNG = 0x0002;

        [DllImport("shell32.dll", CharSet = CharSet.Auto, SetLastError = true)]
        public static extern void SHChangeNotify(uint wEventId, uint uFlags, IntPtr dwItem1, IntPtr dwItem2);

        private const uint SHCNE_ASSOCCHANGED = 0x08000000;
        private const uint SHCNF_IDLIST = 0x0000;

        public SetupForm()
        {
            InitializeComponent();
            LoadLogo();
            Log("Slate Setup Ready.");
        }

        private void InitializeComponent()
        {
            this.btnInstall = new Button();
            this.btnUninstall = new Button();
            this.btnExit = new Button();
            this.txtLog = new TextBox();
            this.progressBar = new ProgressBar();
            this.lblTitle = new Label();
            this.lblSubtitle = new Label();
            this.picLogo = new PictureBox();

            // Form Layout
            this.Text = "Slate Visual Setup";
            this.Size = new Size(540, 420);
            this.BackColor = Color.FromArgb(248, 250, 252); // slate light background
            this.FormBorderStyle = FormBorderStyle.FixedDialog;
            this.MaximizeBox = false;
            this.StartPosition = FormStartPosition.CenterScreen;

            // Logo
            this.picLogo.Size = new Size(64, 64);
            this.picLogo.Location = new Point(24, 20);
            this.picLogo.SizeMode = PictureBoxSizeMode.Zoom;
            this.picLogo.BackColor = Color.Transparent;

            // Title
            this.lblTitle.Text = "SLATE LANGUAGE";
            this.lblTitle.Font = new Font("Segoe UI", 16F, FontStyle.Bold);
            this.lblTitle.ForeColor = Color.FromArgb(15, 23, 42); // dark slate text
            this.lblTitle.Location = new Point(102, 18);
            this.lblTitle.Size = new Size(400, 32);

            // Subtitle
            this.lblSubtitle.Text = "Universal Visual Layout & Diagrams Setup Utility";
            this.lblSubtitle.Font = new Font("Segoe UI", 9F, FontStyle.Regular);
            this.lblSubtitle.ForeColor = Color.FromArgb(100, 116, 139); // slate muted
            this.lblSubtitle.Location = new Point(104, 50);
            this.lblSubtitle.Size = new Size(400, 20);

            // Install Button
            this.btnInstall.Text = "Install Slate CLI";
            this.btnInstall.Location = new Point(24, 95);
            this.btnInstall.Size = new Size(150, 40);
            this.btnInstall.BackColor = Color.FromArgb(79, 70, 229); // indigo
            this.btnInstall.ForeColor = Color.White;
            this.btnInstall.FlatStyle = FlatStyle.Flat;
            this.btnInstall.FlatAppearance.BorderSize = 0;
            this.btnInstall.Font = new Font("Segoe UI", 10F, FontStyle.Bold);
            this.btnInstall.Cursor = Cursors.Hand;
            this.btnInstall.Click += new EventHandler(this.OnInstallClick);

            // Uninstall Button
            this.btnUninstall.Text = "Uninstall";
            this.btnUninstall.Location = new Point(186, 95);
            this.btnUninstall.Size = new Size(150, 40);
            this.btnUninstall.BackColor = Color.FromArgb(254, 242, 242); // light red
            this.btnUninstall.ForeColor = Color.FromArgb(220, 38, 38); // Red text
            this.btnUninstall.FlatStyle = FlatStyle.Flat;
            this.btnUninstall.FlatAppearance.BorderColor = Color.FromArgb(220, 38, 38);
            this.btnUninstall.FlatAppearance.BorderSize = 1;
            this.btnUninstall.Font = new Font("Segoe UI", 10F, FontStyle.Bold);
            this.btnUninstall.Cursor = Cursors.Hand;
            this.btnUninstall.Click += new EventHandler(this.OnUninstallClick);

            // Exit Button
            this.btnExit.Text = "Exit";
            this.btnExit.Location = new Point(348, 95);
            this.btnExit.Size = new Size(150, 40);
            this.btnExit.BackColor = Color.FromArgb(241, 245, 249); // light gray
            this.btnExit.ForeColor = Color.FromArgb(71, 85, 105); // gray text
            this.btnExit.FlatStyle = FlatStyle.Flat;
            this.btnExit.FlatAppearance.BorderSize = 0;
            this.btnExit.Font = new Font("Segoe UI", 10F, FontStyle.Regular);
            this.btnExit.Cursor = Cursors.Hand;
            this.btnExit.Click += (s, e) => this.Close();

            // Progress Bar
            this.progressBar.Location = new Point(24, 155);
            this.progressBar.Size = new Size(474, 12);
            this.progressBar.Style = ProgressBarStyle.Continuous;
            this.progressBar.BackColor = Color.FromArgb(241, 245, 249);
            this.progressBar.ForeColor = Color.FromArgb(79, 70, 229);

            // Log TextBox
            this.txtLog.Multiline = true;
            this.txtLog.ReadOnly = true;
            this.txtLog.ScrollBars = ScrollBars.Vertical;
            this.txtLog.Location = new Point(24, 185);
            this.txtLog.Size = new Size(474, 175);
            this.txtLog.BackColor = Color.White;
            this.txtLog.ForeColor = Color.FromArgb(15, 23, 42);
            this.txtLog.Font = new Font("Consolas", 9F);
            this.txtLog.BorderStyle = BorderStyle.FixedSingle;

            // Add controls
            this.Controls.Add(this.picLogo);
            this.Controls.Add(this.lblTitle);
            this.Controls.Add(this.lblSubtitle);
            this.Controls.Add(this.btnInstall);
            this.Controls.Add(this.btnUninstall);
            this.Controls.Add(this.btnExit);
            this.Controls.Add(this.progressBar);
            this.Controls.Add(this.txtLog);
        }

        private void LoadLogo()
        {
            try
            {
                using (Stream stream = Assembly.GetExecutingAssembly().GetManifestResourceStream("logo.png"))
                {
                    if (stream != null)
                    {
                        this.picLogo.Image = Image.FromStream(stream);
                    }
                }
            }
            catch (Exception ex)
            {
                Log("Warning loading logo: " + ex.Message);
            }
        }

        private void Log(string text)
        {
            txtLog.AppendText("[" + DateTime.Now.ToString("HH:mm:ss") + "] " + text + Environment.NewLine);
        }

        private void OnInstallClick(object sender, EventArgs e)
        {
            btnInstall.Enabled = false;
            btnUninstall.Enabled = false;
            progressBar.Value = 0;

            Log("Beginning Slate installation...");
            try
            {
                string home = Environment.GetEnvironmentVariable("USERPROFILE") ?? Environment.GetEnvironmentVariable("HOME");
                if (string.IsNullOrEmpty(home))
                {
                    Log("Error: Home directory could not be resolved.");
                    return;
                }

                string slateDir = Path.Combine(home, ".slate");
                string binDir = Path.Combine(slateDir, "bin");

                Log("Creating installation directory...");
                Directory.CreateDirectory(binDir);
                progressBar.Value = 20;

                Log("Writing slate compiler binary...");
                ExtractResource("slate.exe", Path.Combine(binDir, "slate.exe"));
                progressBar.Value = 50;

                Log("Writing logo resource...");
                ExtractResource("logo.png", Path.Combine(slateDir, "logo.png"));
                ExtractResource("logo.ico", Path.Combine(slateDir, "logo.ico"));
                progressBar.Value = 70;

                Log("Registering .slt file association...");
                RegisterFileAssociation(".slt", "Slate.Document", "Slate Visual File", Path.Combine(slateDir, "logo.ico"));
                progressBar.Value = 80;

                Log("Updating environment PATH...");
                string oldPath = Environment.GetEnvironmentVariable("PATH", EnvironmentVariableTarget.User) ?? "";
                if (!oldPath.Contains(binDir))
                {
                    string newPath = oldPath;
                    if (!newPath.EndsWith(";") && !string.IsNullOrEmpty(newPath))
                    {
                        newPath += ";";
                    }
                    newPath += binDir;
                    Environment.SetEnvironmentVariable("PATH", newPath, EnvironmentVariableTarget.User);
                    Log("PATH variable configured successfully.");
                }
                else
                {
                    Log("Slate PATH already configured.");
                }
                progressBar.Value = 90;

                Log("Broadcasting system configuration update...");
                BroadcastSettingChange();
                SHChangeNotify(SHCNE_ASSOCCHANGED, SHCNF_IDLIST, IntPtr.Zero, IntPtr.Zero);
                progressBar.Value = 100;

                Log("Slate installation completed successfully!");
                MessageBox.Show("Slate has been successfully installed!\n\nPlease restart any open terminal windows and run 'slate help' to get started.", "Installation Successful", MessageBoxButtons.OK, MessageBoxIcon.Information);
            }
            catch (Exception ex)
            {
                Log("Installation failed: " + ex.Message);
                MessageBox.Show("Installation failed:\n" + ex.Message, "Setup Error", MessageBoxButtons.OK, MessageBoxIcon.Error);
            }
            finally
            {
                btnInstall.Enabled = true;
                btnUninstall.Enabled = true;
            }
        }

        private void OnUninstallClick(object sender, EventArgs e)
        {
            var confirm = MessageBox.Show("Are you sure you want to completely uninstall Slate?", "Confirm Uninstall", MessageBoxButtons.YesNo, MessageBoxIcon.Warning);
            if (confirm != DialogResult.Yes) return;

            btnInstall.Enabled = false;
            btnUninstall.Enabled = false;
            progressBar.Value = 0;

            Log("Beginning Slate uninstallation...");
            try
            {
                string home = Environment.GetEnvironmentVariable("USERPROFILE") ?? Environment.GetEnvironmentVariable("HOME");
                if (string.IsNullOrEmpty(home))
                {
                    Log("Error: Home directory could not be resolved.");
                    return;
                }

                string slateDir = Path.Combine(home, ".slate");
                string binDir = Path.Combine(slateDir, "bin");

                Log("Removing system files...");
                if (Directory.Exists(slateDir))
                {
                    Directory.Delete(slateDir, true);
                    Log("Deleted slate files.");
                }
                progressBar.Value = 40;

                Log("Removing file association registry entries...");
                try
                {
                    Registry.CurrentUser.DeleteSubKeyTree(@"Software\Classes\.slt", false);
                    Registry.CurrentUser.DeleteSubKeyTree(@"Software\Classes\Slate.Document", false);
                }
                catch (Exception ex)
                {
                    Log("Warning removing registry entries: " + ex.Message);
                }
                progressBar.Value = 60;

                Log("Removing Slate from PATH environment variable...");
                string oldPath = Environment.GetEnvironmentVariable("PATH", EnvironmentVariableTarget.User) ?? "";
                if (oldPath.Contains(binDir))
                {
                    string newPath = oldPath.Replace(binDir, "").Replace(";;", ";");
                    if (newPath.EndsWith(";")) newPath = newPath.Substring(0, newPath.Length - 1);
                    if (newPath.StartsWith(";")) newPath = newPath.Substring(1);

                    Environment.SetEnvironmentVariable("PATH", newPath, EnvironmentVariableTarget.User);
                    Log("PATH updated.");
                }
                else
                {
                    Log("Slate PATH not found, skipping registry updates.");
                }
                progressBar.Value = 85;

                Log("Broadcasting uninstallation update...");
                BroadcastSettingChange();
                SHChangeNotify(SHCNE_ASSOCCHANGED, SHCNF_IDLIST, IntPtr.Zero, IntPtr.Zero);
                progressBar.Value = 100;

                Log("Slate has been successfully uninstalled.");
                MessageBox.Show("Slate has been successfully uninstalled from your system.", "Uninstallation Successful", MessageBoxButtons.OK, MessageBoxIcon.Information);
            }
            catch (Exception ex)
            {
                Log("Uninstallation failed: " + ex.Message);
                MessageBox.Show("Uninstallation failed:\n" + ex.Message, "Setup Error", MessageBoxButtons.OK, MessageBoxIcon.Error);
            }
            finally
            {
                btnInstall.Enabled = true;
                btnUninstall.Enabled = true;
            }
        }

        private void ExtractResource(string resourceName, string outputPath)
        {
            using (Stream stream = Assembly.GetExecutingAssembly().GetManifestResourceStream(resourceName))
            {
                if (stream == null)
                {
                    throw new Exception("Manifest resource '" + resourceName + "' not found in setup assembly.");
                }

                // Delete output file if exists to overwrite
                if (File.Exists(outputPath))
                {
                    File.Delete(outputPath);
                }

                using (FileStream fileStream = new FileStream(outputPath, FileMode.Create, FileAccess.Write))
                {
                    stream.CopyTo(fileStream);
                }
            }
        }

        private static void BroadcastSettingChange()
        {
            try
            {
                UIntPtr result;
                SendMessageTimeout(HWND_BROADCAST, WM_SETTINGCHANGE, UIntPtr.Zero, "Environment", SMTO_ABORTIFHUNG, 5000, out result);
            }
            catch {}
        }

        private void RegisterFileAssociation(string extension, string progId, string description, string iconPath)
        {
            try
            {
                using (RegistryKey key = Registry.CurrentUser.CreateSubKey(@"Software\Classes\" + extension))
                {
                    key.SetValue("", progId);
                }

                using (RegistryKey key = Registry.CurrentUser.CreateSubKey(@"Software\Classes\" + progId))
                {
                    key.SetValue("", description);
                    using (RegistryKey defaultIcon = key.CreateSubKey("DefaultIcon"))
                    {
                        defaultIcon.SetValue("", iconPath);
                    }
                }
            }
            catch (Exception ex)
            {
                Log("Warning: Could not register file association: " + ex.Message);
            }
        }

        [STAThread]
        public static void Main()
        {
            Application.EnableVisualStyles();
            Application.SetCompatibleTextRenderingDefault(false);
            Application.Run(new SetupForm());
        }
    }
}
