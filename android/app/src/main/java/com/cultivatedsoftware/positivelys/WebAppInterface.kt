package com.cultivatedsoftware.positivelys

import android.content.Intent
import android.net.Uri
import android.os.Environment
import android.provider.MediaStore
import android.webkit.JavascriptInterface
import androidx.core.content.FileProvider
import com.cultivatedsoftware.positivelys.models.AppRequest
import com.cultivatedsoftware.positivelys.services.AppService
import com.google.gson.Gson
import java.io.File
import java.io.IOException
import java.text.SimpleDateFormat
import java.util.*


class WebAppInterface(private val mContext: AppPageActivity) {
    val gson: Gson = Gson()


    @JavascriptInterface
    fun invokePhotoPicker(imagePickerResultId: String) {
        mContext.imagePickerInputId = imagePickerResultId

        val chooserIntent = Intent(Intent.ACTION_CHOOSER)
        val pickPhoto = Intent(
            Intent.ACTION_PICK,
            MediaStore.Images.Media.EXTERNAL_CONTENT_URI
        )
        chooserIntent.putExtra(Intent.EXTRA_INTENT, pickPhoto)
        chooserIntent.putExtra(Intent.EXTRA_TITLE, "Image Chooser")

        // TODO this should save the image to app cache or temp dir that other apps don't have access to
        val takePictureIntent = Intent(MediaStore.ACTION_IMAGE_CAPTURE)
        takePictureIntent.resolveActivity(mContext.packageManager)?.also {
            // Create the File where the photo should go
            val photoFile: File? = try {
                createImageFile(mContext)
            } catch (ex: IOException) {
                // Error occurred while creating the File
                null
            }
            // Continue only if the File was successfully created
            photoFile?.also {
                val photoURI: Uri = FileProvider.getUriForFile(
                    mContext,
                    "com.example.android.fileprovider",
                    it
                )
                takePictureIntent.putExtra(MediaStore.EXTRA_OUTPUT, photoURI)
            }
        }

        val intentArray = arrayOf(takePictureIntent)
        chooserIntent.putExtra(Intent.EXTRA_INITIAL_INTENTS, intentArray)

        mContext.startActivityForResult(chooserIntent, IMAGE_REQUEST_CODE)
    }

    @Throws(IOException::class)
    private fun createImageFile(mContext: AppPageActivity): File {
        // Create an image file name
        val timeStamp: String = SimpleDateFormat("yyyyMMdd_HHmmss").format(Date())
        val storageDir: File? = mContext.getExternalFilesDir(Environment.DIRECTORY_PICTURES)
        return File.createTempFile(
            "JPEG_${timeStamp}_", /* prefix */
            ".jpg", /* suffix */
            storageDir /* directory */

        ).apply {
            mContext.imagePickerPath = absolutePath
        }
    }


    @JavascriptInterface
    fun makeAppRequest(appRequestAsJson: String) {
        val databasePath =
            mContext.packageManager.getPackageInfo(mContext.packageName, 0).applicationInfo.dataDir
        val appService = AppService(databasePath, mContext.filesDir.absolutePath)

        val appRequest = gson.fromJson<AppRequest>(appRequestAsJson, AppRequest::class.java)

        var response = appService.makeRequest(appRequest)

        var wasRedirect = false
        if (response.status_code == 302) {
            val location = response.headers?.get("Location")
            val redirectAppRequest = AppRequest(location.toString(), "GET")
            response = appService.makeRequest(redirectAppRequest)

            wasRedirect = true
        }

        val contentLocation = response.headers?.get("Content-Location")
        val intent = Intent(mContext, AppPageActivity::class.java).apply {
            putExtra(APP_PAGE_ACTIVITY_URL, contentLocation)
        }

        if (wasRedirect) {
            intent.putExtra(APP_PAGE_WAS_REDIRECT, true)
        }

        intent.addFlags(Intent.FLAG_ACTIVITY_SINGLE_TOP)

        mContext.applicationContext.startActivity(intent)
    }
}
