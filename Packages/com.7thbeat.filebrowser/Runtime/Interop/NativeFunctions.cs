using System;
using System.Runtime.InteropServices;

namespace Interop
{
    internal static class NativeFunctions
    {
        private const string Lib = "unityfiledialog";

        [DllImport(Lib)]
        public static extern bool opener_reveal_file(string path);

        [DllImport(Lib)]
        public static extern void cstring_destroy(IntPtr str);

        [DllImport(Lib)]
        public static extern IntPtr file_dialog_create();
        
        [DllImport(Lib)]
        public static extern void file_dialog_set_directory(IntPtr dialog, string directory);
        
        [DllImport(Lib)]
        public static extern void file_dialog_set_file_name(IntPtr dialog, string filename);
        
        [DllImport(Lib)]
        public static extern void file_dialog_add_filter(IntPtr dialog, string name, string[] extensions, int extensionsCount);
        
        [DllImport(Lib)]
        public static extern void file_dialog_destroy(IntPtr dialog);

        [DllImport(Lib)]
        public static extern IntPtr file_dialog_pick_file(IntPtr dialog);
        
        [DllImport(Lib)]
        public static extern IntPtr file_dialog_save_file(IntPtr dialog);
    }
}