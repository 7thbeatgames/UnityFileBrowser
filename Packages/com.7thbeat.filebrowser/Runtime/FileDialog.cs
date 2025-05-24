using System;
using Interop;

namespace UnityFileDialog
{
    public class FileDialog : IDisposable
    {
        private readonly IntPtr _ptr = NativeFunctions.file_dialog_create();

        /// <summary>
        /// Set the directory which the picker should open
        /// </summary>
        /// <param name="path"></param>
        public void SetDirectory(string path)
        {
            NativeFunctions.file_dialog_set_directory(_ptr, path);
        }

        /// <summary>
        /// Set the default file name in save dialog
        /// </summary>
        /// <param name="filename">file name to set</param>
        public void SetFileName(string filename)
        {
            NativeFunctions.file_dialog_set_file_name(_ptr, filename);
        }

        /// <summary>
        /// Add file extension filter for picker
        /// </summary>
        /// <param name="name">Name of the filter</param>
        /// <param name="extensions">Extension names of the filter</param>
        /// <exception cref="ArgumentNullException">thrown if either name or extensions is null</exception>
        public void AddFilter(string name, string[] extensions)
        {
            if (name == null) throw new ArgumentNullException(nameof(name));
            if (extensions == null) throw new ArgumentNullException(nameof(extensions));
            NativeFunctions.file_dialog_add_filter(_ptr, name, extensions, extensions.Length);
        }

        /// <summary>
        /// Opens a file picker dialog
        /// </summary>
        /// <returns>path of the picked file, null if selection is canceled</returns>
        public string PickFile() => NativeFunctions.GetStringAndFree(NativeFunctions.file_dialog_pick_file(_ptr));

        /// <summary>
        /// Opens a file picker dialog which allows multiple selection
        /// </summary>
        /// <returns>path array of picked files, null if selection is canceled</returns>
        public string[] PickFiles() =>
            NativeFunctions.GetCStringBufferAndFree(NativeFunctions.file_dialog_pick_files(_ptr));

        /// <summary>
        /// Opens a folder picker dialog
        /// </summary>
        /// <returns>path of picked folder, null if selection is canceled</returns>
        public string PickFolder() => NativeFunctions.GetStringAndFree(NativeFunctions.file_dialog_pick_folder(_ptr));

        /// <summary>
        /// Opens a folder picker dialog which allows multiple selection
        /// </summary>
        /// <returns>path array of picked folders, null if selection is canceled</returns>
        public string[] PickFolders() =>
            NativeFunctions.GetCStringBufferAndFree(NativeFunctions.file_dialog_pick_folders(_ptr));

        /// <summary>
        /// Opens a file save dialog
        /// </summary>
        /// <returns>path of the picked file, null if selection is canceled</returns>
        public string SaveFile() => NativeFunctions.GetStringAndFree(NativeFunctions.file_dialog_save_file(_ptr));

        /// <summary>
        /// Set whether to allow creating directory in the picker. Only works on macOS
        /// </summary>
        /// <param name="canCreate">whether to allow creating directory</param>
        public void SetCanCreateDirectories(bool canCreate) =>
            NativeFunctions.file_dialog_set_can_create_directories(_ptr, canCreate);

        /// <summary>
        /// Set window title of picker
        /// </summary>
        /// <param name="title">the title to set</param>
        public void SetTitle(string title) => NativeFunctions.file_dialog_set_title(_ptr, title);

        public void Dispose()
        {
            NativeFunctions.file_dialog_destroy(_ptr);
        }
    }
}