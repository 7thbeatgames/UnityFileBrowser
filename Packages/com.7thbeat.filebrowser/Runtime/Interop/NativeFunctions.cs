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
        public static extern IntPtr file_dialog_create();

        [DllImport(Lib)]
        public static extern void file_dialog_set_directory(IntPtr dialog, string directory);

        [DllImport(Lib)]
        public static extern void file_dialog_set_file_name(IntPtr dialog, string filename);

        [DllImport(Lib)]
        public static extern void file_dialog_add_filter(IntPtr dialog, string name, string[] extensions,
            int extensionsCount);

        [DllImport(Lib)]
        public static extern void file_dialog_destroy(IntPtr dialog);

        [DllImport(Lib)]
        public static extern IntPtr file_dialog_pick_file(IntPtr dialog);

        [DllImport(Lib)]
        public static extern IntPtr file_dialog_pick_files(IntPtr dialog);

        [DllImport(Lib)]
        public static extern IntPtr file_dialog_pick_folder(IntPtr dialog);

        [DllImport(Lib)]
        public static extern IntPtr file_dialog_pick_folders(IntPtr dialog);

        [DllImport(Lib)]
        public static extern IntPtr file_dialog_save_file(IntPtr dialog);

        [DllImport(Lib)]
        public static extern void file_dialog_set_can_create_directories(IntPtr dialog, bool canCreate);

        [DllImport(Lib)]
        public static extern void file_dialog_set_title(IntPtr dialog, string title);

        [DllImport(Lib)]
        public static extern void cstring_free(IntPtr str);

        [DllImport(Lib)]
        public static extern void cstring_buffer_free(IntPtr ptr);

        public static string GetStringAndFree(IntPtr stringPtr)
        {
            if (stringPtr == IntPtr.Zero) return null;

            var result = Marshal.PtrToStringUTF8(stringPtr);
            cstring_free(stringPtr);

            return result;
        }

        public static string[] GetCStringBufferAndFree(IntPtr ptr)
        {
            if (ptr == IntPtr.Zero) return null;
            var buf = Marshal.PtrToStructure<CStringBuffer>(ptr);
            var result = buf.GetValue();
            cstring_buffer_free(ptr);

            return result;
        }
    }
}