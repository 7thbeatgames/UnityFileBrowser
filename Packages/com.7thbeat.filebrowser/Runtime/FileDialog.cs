using System;
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

        public string PickFile() => NativeFunctions.GetStringAndFree(NativeFunctions.file_dialog_pick_file(_ptr));

        public string[] PickFiles() =>
            NativeFunctions.GetCStringBufferAndFree(NativeFunctions.file_dialog_pick_files(_ptr));

        public string PickFolder() => NativeFunctions.GetStringAndFree(NativeFunctions.file_dialog_pick_folder(_ptr));

        public string[] PickFolders() =>
            NativeFunctions.GetCStringBufferAndFree(NativeFunctions.file_dialog_pick_folders(_ptr));

        public string SaveFile() => NativeFunctions.GetStringAndFree(NativeFunctions.file_dialog_save_file(_ptr));

        public void SetCanCreateDirectories(bool canCreate) =>
            NativeFunctions.file_dialog_set_can_create_directories(_ptr, canCreate);

        public void SetTitle(string title) => NativeFunctions.file_dialog_set_title(_ptr, title);

        public void Dispose()
        {
            NativeFunctions.file_dialog_destroy(_ptr);
        }
    }
}