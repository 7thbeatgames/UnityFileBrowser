using System;
using System.Runtime.InteropServices;
using Interop;

namespace UnityFileDialog
{
    public class FileDialog : IDisposable
    {
        private readonly IntPtr _ptr = NativeFunctions.file_dialog_create();

        public void SetDirectory(string path)
        {
            NativeFunctions.file_dialog_set_directory(_ptr, path);
        }
        
        public void SetFileName(string filename)
        {
            NativeFunctions.file_dialog_set_file_name(_ptr, filename);
        }

        public void AddFilter(string name, string[] extensions)
        {
            NativeFunctions.file_dialog_add_filter(_ptr, name, extensions, extensions.Length);
        }

        private string GetString(IntPtr stringPtr)
        {
            if (stringPtr == IntPtr.Zero) return null;

            var result = Marshal.PtrToStringUTF8(stringPtr);
            NativeFunctions.cstring_destroy(stringPtr);

            return result;
        }

        public string PickFile() => GetString(NativeFunctions.file_dialog_pick_file(_ptr));

        public string SaveFile() => GetString(NativeFunctions.file_dialog_save_file(_ptr));

        public void Dispose()
        {
            NativeFunctions.file_dialog_destroy(_ptr);
        }
    }
}