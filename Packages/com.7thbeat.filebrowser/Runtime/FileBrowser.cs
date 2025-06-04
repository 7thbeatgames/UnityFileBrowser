using System.Collections.Generic;
using Interop;
using JetBrains.Annotations;

namespace UnityFileDialog
{
    public static class FileBrowser
    {
        /// <summary>
        /// Open file browser and select the file(does not open file)
        /// </summary>
        /// <param name="path">path to reveal</param>
        /// <returns></returns>
        public static bool Reveal(string path) => NativeFunctions.opener_reveal(path);

        /// <summary>
        /// Wrapper for <see cref="FileDialog.PickFile" />
        /// </summary>
        /// <param name="directory">context directory path</param>
        /// <param name="filterName">file filter name</param>
        /// <param name="filterExtensions">file filter extensions</param>
        /// <param name="title">window title</param>
        /// <returns>file picked in the picker, null if canceled</returns>
        [CanBeNull]
        public static string PickFile(string directory = null, string filterName = null,
            string[] filterExtensions = null, string title = null)
        {
            using var dialog = new FileDialog();
            if (!string.IsNullOrEmpty(directory))
                dialog.SetDirectory(directory);
            if (!string.IsNullOrEmpty(filterName))
                dialog.AddFilter(filterName, filterExtensions);
            if (!string.IsNullOrEmpty(title))
                dialog.SetTitle(title);

            return dialog.PickFile();
        }

        /// <summary>
        /// Wrapper for <see cref="FileDialog.PickFiles" />
        /// </summary>
        /// <param name="directory">context directory path</param>
        /// <param name="filterName">file filter name</param>
        /// <param name="filterExtensions">file filter extensions</param>
        /// <param name="title">window title</param>
        /// <returns>array of files picked in the picker, null if canceled</returns>
        [CanBeNull]
        public static string[] PickFiles(string directory = null, string filterName = null,
            string[] filterExtensions = null, string title = null)
        {
            using var dialog = new FileDialog();
            if (!string.IsNullOrEmpty(directory))
                dialog.SetDirectory(directory);
            if (!string.IsNullOrEmpty(filterName))
                dialog.AddFilter(filterName, filterExtensions);
            if (!string.IsNullOrEmpty(title))
                dialog.SetTitle(title);

            return dialog.PickFiles();
        }

        /// <summary>
        /// Wrapper for <see cref="FileDialog.SaveFile" />
        /// </summary>
        /// <param name="directory">context directory path</param>
        /// <param name="filename">default file name in picker</param>
        /// <param name="filterName">file filter name</param>
        /// <param name="filterExtensions">file filter extensions</param>
        /// <param name="title">window title</param>
        /// <returns>file picked in the picker, null if canceled</returns>
        [CanBeNull]
        public static string SaveFile(string directory = null, string filename = null, string filterName = null,
            string[] filterExtensions = null, string title = null)
        {
            using var dialog = new FileDialog();
            if (!string.IsNullOrEmpty(filename))
                dialog.SetFileName(filename);
            if (!string.IsNullOrEmpty(directory))
                dialog.SetDirectory(directory);
            if (!string.IsNullOrEmpty(filterName))
                dialog.AddFilter(filterName, filterExtensions);
            if (!string.IsNullOrEmpty(title))
                dialog.SetTitle(title);

            return dialog.SaveFile();
        }

        /// <summary>
        /// Wrapper for <see cref="FileDialog.PickFolder" />
        /// </summary>
        /// <param name="directory">context directory path</param>
        /// <param name="filterName">file filter name</param>
        /// <param name="filterExtensions">file filter extensions</param>
        /// <param name="title">window title</param>
        /// <returns>folder picked in the picker, null if canceled</returns>
        [CanBeNull]
        public static string PickFolder(string directory = null, string filterName = null,
            string[] filterExtensions = null, string title = null)
        {
            using var dialog = new FileDialog();
            if (!string.IsNullOrEmpty(directory))
                dialog.SetDirectory(directory);
            if (!string.IsNullOrEmpty(filterName))
                dialog.AddFilter(filterName, filterExtensions);
            if (!string.IsNullOrEmpty(title))
                dialog.SetTitle(title);

            return dialog.PickFolder();
        }

        /// <summary>
        /// Wrapper for <see cref="FileDialog.PickFolders" />
        /// </summary>
        /// <param name="directory">context directory path</param>
        /// <param name="filterName">file filter name</param>
        /// <param name="filterExtensions">file filter extensions</param>
        /// <param name="title">window title</param>
        /// <returns>array of folders picked in the picker, null if canceled</returns>
        [CanBeNull]
        public static string[] PickFolders(string directory = null, string filterName = null,
            string[] filterExtensions = null, string title = null)
        {
            using var dialog = new FileDialog();
            if (!string.IsNullOrEmpty(directory))
                dialog.SetDirectory(directory);
            if (!string.IsNullOrEmpty(filterName))
                dialog.AddFilter(filterName, filterExtensions);
            if (!string.IsNullOrEmpty(title))
                dialog.SetTitle(title);

            return dialog.PickFiles();
        }
    }
}