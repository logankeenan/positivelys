package com.cultivatedsoftware.positivelys.services

import android.content.Context
import android.content.res.AssetManager
import java.io.File
import java.io.FileOutputStream

class FileService(var assetManager: AssetManager) {

    fun listFilesRecursively(path: String): MutableList<String> {
        val files = mutableListOf<String>();

        val listOfFilesForPath = assetManager.list(path)

        listOfFilesForPath!!.forEach {
            val fileWithPath = "$path/$it"

            val isFile = assetManager.list(fileWithPath).isNullOrEmpty()
            if (isFile) {
                files.add(fileWithPath)
            } else {
                val listOfFilesForPath1 = listFilesRecursively(fileWithPath)

                files.addAll(listOfFilesForPath1)
            }
        }

        return files
    }


    fun writeFilesToDataDirectory(files: List<String>, context: Context) {
        files.forEach(fun(filePathAsString: String) {
            val file = File(filePathAsString)
            val filePath = file.path.replace(file.name, "")
            ensureDirExistsForFile(context, filePath)

            val open = assetManager.open(filePathAsString)
            val readBytes = open.readBytes()

            val fileToWrite = File(
                context.packageManager.getPackageInfo(
                    context.packageName,
                    0
                ).applicationInfo.dataDir + "/" + filePath, file.name
            )

            val fileOutputStream = FileOutputStream(fileToWrite, false)

            fileOutputStream.write(readBytes)
            fileOutputStream.close()

        })

    }

    private fun ensureDirExistsForFile(context: Context, filePath: String) {
        val filePathOnSystem = File(
            context.packageManager.getPackageInfo(
                context.packageName,
                0
            ).applicationInfo.dataDir + "/" + filePath
        )

        if (!filePathOnSystem.exists()) {
            filePathOnSystem.mkdirs()
        }
    }

}